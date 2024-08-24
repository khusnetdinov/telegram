use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_gradient::BackgroundFillGradient as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BackgroundFillGradient {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub top_color: u32,
    pub bottom_color: u32,
    pub rotation_angle: u16,
}
