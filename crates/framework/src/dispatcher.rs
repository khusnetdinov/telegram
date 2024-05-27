use crate::enums::messages::Messages;
use crate::structs::message::Message;
use crate::structs::update::Update;

#[derive(Debug, Default)]
pub struct Dispatcher {}

impl Dispatcher {
    pub fn dispatch(&self, update: &Update) -> Option<Messages> {
        let inner = update.message.as_deref().unwrap();

        match Message::from(inner.clone()) {
            message if message.is_text() => Some(Messages::Text(message.into())),
            message if message.is_command() => Some(Messages::Command(message.into())),
            message if message.is_photo() => Some(Messages::Photo(message.into())),
            _ => None,
        }
    }
}
