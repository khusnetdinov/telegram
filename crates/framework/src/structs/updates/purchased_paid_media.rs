use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::purchased_paid_media::PaidMediaPurchased as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct PaidMediaPurchased {
    pub from: User,
    pub paid_media_payload: String,
}
