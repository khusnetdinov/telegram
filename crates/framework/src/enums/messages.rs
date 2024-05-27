use crate::structs::message::Message;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::chat::Chat;
use telegram_bots_api::api::structs::message_entity::MessageEntity;
use telegram_bots_api::api::structs::message_id::MessageId;
use telegram_bots_api::api::structs::photo_size::PhotoSize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Messages {
    Command(CommandMessage),
    Text(TextMessage),
    Photo(PhotoMessage),
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextMessage {
    pub message_id: MessageId,
    pub date: i64,
    pub chat: Chat,
    pub text: String,
    pub from: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
}

impl From<Message> for TextMessage {
    fn from(message: Message) -> Self {
        Self {
            message_id: message.message_id.clone(),
            chat: message.chat.clone(),
            date: message.date,
            text: message.text.clone().unwrap(),
            from: User::from(message.from.clone().unwrap()),
            entities: message.entities.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandMessage {
    pub message_id: MessageId,
    pub date: i64,
    pub chat: Chat,
    pub text: String,
    pub from: User,
    pub entities: Vec<MessageEntity>,
}

impl From<Message> for CommandMessage {
    fn from(message: Message) -> Self {
        Self {
            message_id: message.message_id.clone(),
            chat: message.chat.clone(),
            date: message.date,
            text: message.text.clone().unwrap(),
            from: User::from(message.from.clone().unwrap()),
            entities: message.entities.clone().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoMessage {
    pub message_id: MessageId,
    pub date: i64,
    pub chat: Chat,
    pub from: User,
    pub photo: Vec<PhotoSize>,
}

impl From<Message> for PhotoMessage {
    fn from(message: Message) -> Self {
        Self {
            message_id: message.message_id.clone(),
            chat: message.chat.clone(),
            date: message.date,
            from: User::from(message.from.clone().unwrap()),
            photo: message.photo.clone().unwrap(),
        }
    }
}
