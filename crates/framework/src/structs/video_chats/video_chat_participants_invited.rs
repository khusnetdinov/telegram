use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::video_chat_participants_invited::VideoChatParticipantsInvited as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

impl From<IncomingMessage> for VideoChatParticipantsInvited {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            video_chat_participants_invited,
            ..
        } = remote;

        Self::from(video_chat_participants_invited.unwrap())
    }
}
