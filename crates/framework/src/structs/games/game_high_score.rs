use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::game_high_score::GameHighScore as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}
impl From<Remote> for GameHighScore {
    fn from(remote: Remote) -> Self {
        Self {
            position: remote.position,
            user: remote.user.into(),
            score: remote.score,
        }
    }
}
