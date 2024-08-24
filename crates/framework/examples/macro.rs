use telegram_framework::enums::maybe_inaccessible_message::MaybeInaccessibleMessage as Remote;
use telegram_framework::structs::messages::inaccessible_message::InaccessibleMessage;
use telegram_framework::structs::updates::message::Message;
use telegram_macros::FromRemoteEnum;

#[derive(FromRemoteEnum)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}
