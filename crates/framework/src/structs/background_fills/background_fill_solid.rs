use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_solid::BackgroundFillSolid as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundFillSolid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub color: u32,
}
impl From<Remote> for BackgroundFillSolid {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            color: remote.color,
        }
    }
}
