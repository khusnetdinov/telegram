use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::story::Story;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoryMessage {
    pub story: Story,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
}

impl From<Inner> for StoryMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            story,
            media_group_id,
            has_media_spoiler,
            ..
        } = inner;

        Self {
            story: story.unwrap(),
            media_group_id,
            has_media_spoiler,
        }
    }
}
