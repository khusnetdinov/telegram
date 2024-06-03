use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_hidden::GeneralForumTopicHidden;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicHiddenMessage {
    pub general_forum_topic_hidden: GeneralForumTopicHidden,
}

impl From<Inner> for GeneralForumTopicHiddenMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            general_forum_topic_hidden,
            ..
        } = inner;

        Self {
            general_forum_topic_hidden: general_forum_topic_hidden.unwrap(),
        }
    }
}
