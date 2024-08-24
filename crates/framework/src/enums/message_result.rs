use crate::structs::updates::message::Message;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::message_result::MessageResult as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
#[serde(untagged)]
pub enum MessageResult {
    Inline(bool),
    Message(Message),
}
