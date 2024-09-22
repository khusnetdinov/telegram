use crate::structs::media::story::Story;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomingStory {
    pub story: Story,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<IncomingMessage> for IncomingStory {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            story,
            media_group_id,
            has_media_spoiler,
            ..
        } = remote;

        Self {
            story: story.unwrap().into(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
