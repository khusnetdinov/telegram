use crate::structs::games::game_high_score::GameHighScore as Remote;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame {}

impl From<Remote> for CallbackGame {
    fn from(_remote: Remote) -> Self {
        Self {}
    }
}
