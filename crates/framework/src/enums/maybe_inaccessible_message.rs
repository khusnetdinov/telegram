use crate::structs::messages::inaccessible_message::InaccessibleMessage;
use crate::structs::updates::message::Message;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}

impl From<Remote> for MaybeInaccessibleMessage {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Message(message) => Self::Message(Box::new((*message).into())),
            Remote::InaccessibleMessage(inaccessible_message) => {
                Self::InaccessibleMessage(inaccessible_message.into())
            }
        }
    }
}
