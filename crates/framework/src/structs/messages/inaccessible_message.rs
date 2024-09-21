use crate::structs::chat::Chat;
use crate::structs::messages::message_id::MessageId;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inaccessible_message::InaccessibleMessage as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
}
