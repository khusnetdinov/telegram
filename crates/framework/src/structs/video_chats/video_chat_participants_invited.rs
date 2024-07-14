use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}
impl From<Remote> for VideoChatParticipantsInvited {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(map, into)]
            users: remote
                .users
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}
impl From<Message> for VideoChatParticipantsInvited {
    fn from(remote: Message) -> Self {
        let Message {
            video_chat_participants_invited: Some(video_chat_participants_invited),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(video_chat_participants_invited)
    }
}
