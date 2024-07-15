use crate::structs::media::animation::Animation;
use crate::structs::media::photo_size::PhotoSize;
use crate::structs::message_entity::MessageEntity;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::game::Game as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}
impl From<Remote> for Game {
    fn from(remote: Remote) -> Self {
        Self {
            title: remote.title,
            description: remote.description,
            // TODO: #[remote(map, into)]
            photo: remote
                .photo
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            text: remote.text,
            // TODO: #[remote(option, map, into)]
            text_entities: remote
                .text_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            animation: remote.animation.map(|inner| inner.into()),
        }
    }
}

impl From<Message> for Game {
    fn from(remote: Message) -> Self {
        let Message {
            game: Some(game), ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(game)
    }
}
