use crate::structs::reply_markups::inline_keyboards::inline_keyboard_button::InlineKeyboardButton;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::inline_keyboard_markup::InlineKeyboardMarkup as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
