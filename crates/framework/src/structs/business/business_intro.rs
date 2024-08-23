use crate::structs::media::sticker::Sticker;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_intro::BusinessIntro as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}
