use crate::structs::media::sticker::Sticker;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_intro::BusinessIntro as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}

impl From<Remote> for BusinessIntro {
    fn from(remote: Remote) -> Self {
        Self {
            title: remote.title,
            message: remote.message,
            // TODO: #[remote(option, into)]
            sticker: remote.sticker.map(|inner| inner.into()),
        }
    }
}
