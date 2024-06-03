use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatParticipantsInvitedMessage {
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}

impl From<Inner> for VideoChatParticipantsInvitedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            video_chat_participants_invited,
            ..
        } = inner;

        Self {
            video_chat_participants_invited: video_chat_participants_invited.unwrap(),
        }
    }
}
