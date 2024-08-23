use crate::structs::background_fills::background_fill_freeform_gradient::BackgroundFillFreeformGradient;
use crate::structs::background_fills::background_fill_gradient::BackgroundFillGradient;
use crate::structs::background_fills::background_fill_solid::BackgroundFillSolid;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::background_fill::BackgroundFill as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

impl From<Remote> for BackgroundFill {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Solid(solid) => Self::Solid(solid.into()),
            Remote::Gradient(gradient) => Self::Gradient(gradient.into()),
            Remote::FreeformGradient(free) => Self::FreeformGradient(free.into()),
        }
    }
}

impl Default for BackgroundFill {
    fn default() -> Self {
        Self::Solid(BackgroundFillSolid {
            ..Default::default()
        })
    }
}
