pub mod game;
pub mod pinger;
pub mod ponger;

use flawless::message;
use flawless::message::receive;
use flawless::workflow;
use flawless::workflow::Input;
use game::Ball;
use pinger::Pinger;
use ponger::Ponger;

flawless::module! { name = "ping-pong", version = "0.0.1" }

#[message("opponent")]
pub struct Opponent(String);

#[workflow("start_pinger")]
pub fn start_pinger(mut player: Input<Pinger>) {
    log::info!(target: "pinger", "awaiting opponent...");
    let opponent = receive::<Opponent>().0;

    log::info!(target: "pinger", "serving to {}", &opponent);
    player.opponent = Some(opponent);

    let mut the_ball = 0;

    loop {
        player.hit(the_ball);

        if the_ball + 1 == player.game.rallies {
            break;
        };

        the_ball = receive::<Ball>().0;

        if the_ball == player.game.rallies {
            break;
        };
    }
}

#[workflow("start_ponger")]
pub fn start_ponger(player: Input<Ponger>) {
    let mut the_ball: i32;

    log::info!(target: "ponger", "...awaiting serve to return to {}", player.opponent);
    loop {
        the_ball = receive::<Ball>().0;

        if the_ball == player.game.rallies {
            break;
        }

        player.hit(the_ball);

        if the_ball + 1 == player.game.rallies {
            break;
        };
    }
}
