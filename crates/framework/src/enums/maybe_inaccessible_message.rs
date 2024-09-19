use crate::structs::messages::inaccessible_message::InaccessibleMessage;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::maybe_inaccessible_message::MaybeInaccessibleMessage as Remote;
use telegram_bots_api::api::structs::message::Message as RemoteMessage;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum MaybeInaccessibleMessage {
    Message(Box<RemoteMessage>),
    InaccessibleMessage(InaccessibleMessage),
}
