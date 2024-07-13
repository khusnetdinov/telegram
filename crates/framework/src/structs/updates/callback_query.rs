use crate::enums::maybe_inaccessible_message::MaybeInaccessibleMessage;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::callback_query::CallbackQuery as Remote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub chat_instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<MaybeInaccessibleMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

impl From<Remote> for CallbackQuery {
    fn from(remote: Remote) -> Self {
        Self {
            id: remote.id,
            // TODO: #[remote(into)]
            from: remote.from.into(),
            chat_instance: remote.chat_instance,
            // TODO: #[remote(option, into)]
            message: remote.message.map(|inner| inner.into()),
            inline_message_id: remote.inline_message_id,
            data: remote.data,
            game_short_name: remote.game_short_name,
        }
    }
}
