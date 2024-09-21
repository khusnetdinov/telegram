use crate::structs::chat::Chat;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_message_deleted::BusinessMessagesDeleted as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i32>,
}
