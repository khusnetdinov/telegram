use crate::enums::background_fill::BackgroundFill;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_fill::BackgroundTypeFill as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundTypeFill {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub fill: BackgroundFill,
    pub dark_theme_dimming: u8,
}
impl From<Remote> for BackgroundTypeFill {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            fill: remote.fill.into(),
            dark_theme_dimming: remote.dark_theme_dimming,
        }
    }
}
