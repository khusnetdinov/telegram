use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::story::Story as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct Story {
    pub chat: Box<Chat>,
    pub id: i64,
}
