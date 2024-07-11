use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_freeform_gradient::BackgroundFillFreeformGradient as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillFreeformGradient {
    pub kind: String,
    pub colors: Vec<u32>,
}
impl From<Remote> for BackgroundFillFreeformGradient {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            colors: remote.colors,
        }
    }
}
