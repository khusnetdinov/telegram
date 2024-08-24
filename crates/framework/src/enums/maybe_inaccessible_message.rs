use crate::structs::messages::inaccessible_message::InaccessibleMessage;
use crate::structs::updates::message::Message;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage as Remote;
use telegram_macros::FromRemoteEnum;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}
