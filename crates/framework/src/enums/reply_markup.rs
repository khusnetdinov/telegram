use crate::structs::reply_markups::force_reply::ForceReply;
use crate::structs::reply_markups::inline_keyboards::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::structs::reply_markups::reply_keyboards::reply_keyboard_markup::ReplyKeyboardMarkup;
use crate::structs::reply_markups::reply_keyboards::reply_keyboard_remove::ReplyKeyboardRemove;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::reply_markup::ReplyMarkup as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
