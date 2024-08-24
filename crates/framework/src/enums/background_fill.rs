use crate::structs::background_fills::background_fill_freeform_gradient::BackgroundFillFreeformGradient;
use crate::structs::background_fills::background_fill_gradient::BackgroundFillGradient;
use crate::structs::background_fills::background_fill_solid::BackgroundFillSolid;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::background_fill::BackgroundFill as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

impl Default for BackgroundFill {
    fn default() -> Self {
        Self::Solid(BackgroundFillSolid {
            ..Default::default()
        })
    }
}
