use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Message {
    inner: Inner,
}

impl Message {
    pub fn is_text(&self) -> bool {
        let bot_command_entity_none = if self.entities.is_some() {
            !self
                .entities
                .as_ref()
                .unwrap()
                .iter()
                .any(|entity: &MessageEntity| entity.kind.as_str() == "bot_command")
        } else {
            true
        };

        self.text.is_some() && bot_command_entity_none
    }

    pub fn is_command(&self) -> bool {
        let bot_command_entity_any = self.entities.is_some()
            && self
                .entities
                .as_ref()
                .unwrap()
                .iter()
                .any(|entity: &MessageEntity| entity.kind.as_str() == "bot_command");

        self.text.is_some() && bot_command_entity_any
    }

    pub fn is_photo(&self) -> bool {
        self.photo.is_some()
    }
}
