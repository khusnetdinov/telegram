use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralForumTopicUnhiddenMessage {
    pub general_forum_topic_unhidden: GeneralForumTopicUnhidden,
}

impl From<Inner> for GeneralForumTopicUnhiddenMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            general_forum_topic_unhidden,
            ..
        } = inner;

        Self {
            general_forum_topic_unhidden: general_forum_topic_unhidden.unwrap(),
        }
    }
}
