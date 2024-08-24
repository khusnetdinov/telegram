use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

impl From<Message> for VideoChatParticipantsInvited {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_participants_invited,
            ..
        } = remote;

        Self::from(video_chat_participants_invited.unwrap())
    }
}
