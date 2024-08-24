use crate::enums::background_fill::BackgroundFill;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_type_fill::BackgroundTypeFill as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BackgroundTypeFill {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub fill: BackgroundFill,
    pub dark_theme_dimming: u8,
}
