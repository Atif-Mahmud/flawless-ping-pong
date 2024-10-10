use std::time::Duration;

use crate::game::{send_ball_over_http, Game};

use flawless::workflow::sleep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pinger {
    pub game: Game,
    pub opponent: Option<String>,
}

impl Pinger {
    pub fn hit(&self, i: i32) {
        sleep(Duration::new(1, 0));
        log::info!("🎾 ping: {}", i);
        match &self.opponent {
            Some(opponent) => {
                send_ball_over_http(&self.game.endpoint, opponent, i + 1)
            }
            None => log::error!(
                "cannot hit to a missing opponent, that's unsportsmanlike!"
            ),
        }
    }
}
