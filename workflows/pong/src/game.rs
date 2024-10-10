use flawless::idempotent::Idempotence;
use flawless::message;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[message("ball")]
pub struct Ball(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub endpoint: String,
    pub rallies: i32,
}

impl Game {
    pub fn new(endpoint: String, rallies: i32) -> Game {
        Game { endpoint, rallies }
    }
}

pub fn send_ball_over_http(endpoint: &str, opponent: &str, i: i32) {
    if let Err(err) = flawless_http::post(format!("{endpoint}/{opponent}"))
        .body(json!({
            "type": "ball",
            "data": format!("{i}")
        }))
        .set_header("Content-Type", "application/json")
        .idempotent()
        .send()
    {
        log::error!("{}", err.message().unwrap())
    }
}
