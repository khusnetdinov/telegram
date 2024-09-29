use crate::structs::reply_markups::inline_keyboards::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_query_result_game::InlineQueryResultGame as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct InlineQueryResultGame {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub id: String,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
