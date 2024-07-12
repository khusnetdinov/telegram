use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_bots_api::api::structs::voice::Voice;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceMessage {
    pub voice: Voice,
    pub media_group_id: Option<String>,
    pub has_media_spoiler: Option<bool>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
}

impl From<Message> for VoiceMessage {
    fn from(remote: Message) -> Self {
        let Message {
            voice,
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
            ..
        } = remote;

        Self {
            voice: voice.unwrap(),
            media_group_id,
            has_media_spoiler,
            caption,
            caption_entities,
            show_caption_above_media,
        }
    }
}
