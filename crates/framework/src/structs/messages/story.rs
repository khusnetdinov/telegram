use crate::structs::media::story::Story as Media;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub story: Media,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<Message> for Story {
    fn from(remote: Message) -> Self {
        let Message {
            story,
            media_group_id,
            has_media_spoiler,
            ..
        } = remote;

        Self {
            // TODO: #[remote(into)]
            story: story.unwrap().into(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
