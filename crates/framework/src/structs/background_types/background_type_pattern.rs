use crate::enums::background_fill::BackgroundFill;
use crate::structs::media::document::Document;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_pattern::BackgroundTypePattern as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BackgroundTypePattern {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub document: Document,
    pub fill: BackgroundFill,
    pub intensity: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
