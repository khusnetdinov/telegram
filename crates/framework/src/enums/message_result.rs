use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::message_result::MessageResult as Remote;
use telegram_bots_api::api::structs::message::Message as RemoteMessage;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
#[serde(untagged)]
pub enum MessageResult {
    Inline(bool),
    Message(RemoteMessage),
}
