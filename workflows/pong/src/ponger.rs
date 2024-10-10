use std::time::Duration;

use crate::game::{send_ball_over_http, Game};

use flawless::workflow::sleep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ponger {
    pub game: Game,
    pub opponent: String,
}

impl Ponger {
    pub fn hit(&self, i: i32) {
        sleep(Duration::new(1, 0));
        log::info!("🏓 pong: {}", i);

        send_ball_over_http(&self.game.endpoint, &self.opponent, i + 1);
    }
}
