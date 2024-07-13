use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::story::Story;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMessage {
    pub story: Story,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<Message> for StoryMessage {
    fn from(remote: Message) -> Self {
        let Message {
            story,
            media_group_id,
            has_media_spoiler,
            ..
        } = remote;

        Self {
            story: story.unwrap(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
