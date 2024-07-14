use crate::enums::background_fill::BackgroundFill;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_pattern::BackgroundTypePattern as Remote;
use telegram_bots_api::api::structs::document::Document;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl From<Remote> for BackgroundTypePattern {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            document: remote.document,
            fill: remote.fill.into(),
            intensity: remote.intensity,
            is_inverted: remote.is_inverted,
            is_moving: remote.is_moving,
        }
    }
}
