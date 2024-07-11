use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_gradient::BackgroundFillGradient as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillGradient {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub top_color: u32,
    pub bottom_color: u32,
    pub rotation_angle: u16,
}
impl From<Remote> for BackgroundFillGradient {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            top_color: remote.top_color,
            bottom_color: remote.bottom_color,
            rotation_angle: remote.rotation_angle,
        }
    }
}
