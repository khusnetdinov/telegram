use crate::structs::messages::inaccessible_message::InaccessibleMessage;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage as Remote;
use telegram_bots_api::api::structs::message::Message; // TODO: use crate::structs::update_kinds::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}

impl From<Remote> for MaybeInaccessibleMessage {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::Message(message) => Self::Message(Box::new(*message)),
            Remote::InaccessibleMessage(inaccessible_message) => {
                Self::InaccessibleMessage(inaccessible_message.into())
            }
        }
    }
}
