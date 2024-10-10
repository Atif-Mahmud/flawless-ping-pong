use pong::{
    game::Game, pinger::Pinger, ponger::Ponger, start_pinger, start_ponger,
};
use serde_json::json;
use std::error::Error;

const RALLIES: i32 = 100;
const HOST: &str = "http://localhost:27288";
const ROUTE: &str = "http://localhost:27288/api/workflow/send-msg";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let flawless = flawless_utils::Server::new(HOST, None);
    let ping_pong_module = flawless_utils::load_module_from_build!("pong");
    let deployment = flawless.deploy(ping_pong_module).await?;

    let game = Game::new(ROUTE.to_owned(), RALLIES);

    let pinger_workflow = deployment
        .start::<start_pinger>(Pinger {
            game: game.clone(),
            opponent: None,
        })
        .await?;

    let ponger_workflow = deployment
        .start::<start_ponger>(Ponger {
            game: game.clone(),
            opponent: pinger_workflow.id().to_owned(),
        })
        .await?;

    if let Err(err) = reqwest::Client::new()
        .post(format!("{ROUTE}/{}", pinger_workflow.id()))
        .json(&json!({
            "type": "opponent",
            "data": format!("\"{}\"", ponger_workflow.id())
        }))
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        println!("{err}")
    }

    Ok(())
}
