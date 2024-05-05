use crate::enums::messages::Messages;
use crate::structs::message::Message;
use telegram_bots_api::api::structs::update::Update as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Update {
    pub inner: Inner,
}

impl Update {
    pub fn dispatch(&self) -> Option<Messages> {
        let inner = self.message.as_deref().unwrap();

        match Message::from(inner.clone()) {
            message if message.is_text() => Some(Messages::Text(message.into())),
            message if message.is_command() => Some(Messages::Command(message.into())),
            _ => None,
        }
    }
}
