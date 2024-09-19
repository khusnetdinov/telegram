use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::paid_media_preview::PaidMediaPreview as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct PaidMediaPreview {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
