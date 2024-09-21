use crate::enums::paid_media::PaidMedia;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::paid_media_info::PaidMediaInfo as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct PaidMediaInfo {
    pub star_count: i64,
    pub paid_media: Vec<PaidMedia>,
}
