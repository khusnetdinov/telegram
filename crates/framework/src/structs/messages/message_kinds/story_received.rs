use crate::structs::media::story::Story;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryReceived {
    pub story: Story,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
}

impl From<IncomingMessage> for StoryReceived {
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
