use crate::structs::games::game_high_score::GameHighScore as Remote;
use serde::{Deserialize, Serialize};
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct CallbackGame {}
