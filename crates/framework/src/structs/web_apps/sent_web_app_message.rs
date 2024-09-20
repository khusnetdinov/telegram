use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::sent_web_app_message::SentWebAppMessage as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct SentWebAppMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
