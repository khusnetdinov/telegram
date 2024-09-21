use crate::structs::media::sticker::Sticker as Media;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    pub sticker: Media,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<IncomingMessage> for Sticker {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            sticker,
            media_group_id,
            has_media_spoiler,
            ..
        } = remote;

        Self {
            // TODO: #[remote(into)]
            sticker: sticker.unwrap().into(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
