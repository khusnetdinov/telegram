use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_freeform_gradient::BackgroundFillFreeformGradient as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BackgroundFillFreeformGradient {
    pub kind: String,
    pub colors: Vec<u32>,
}
