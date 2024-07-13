use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatParticipantsInvitedMessage {
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}

impl From<Message> for VideoChatParticipantsInvitedMessage {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_participants_invited,
            ..
        } = remote;

        Self {
            video_chat_participants_invited: video_chat_participants_invited.unwrap(),
        }
    }
}
