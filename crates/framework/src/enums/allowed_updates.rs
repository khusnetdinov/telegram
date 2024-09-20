use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::allowed_updates::AllowedUpdate as Remote;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
}

impl From<Remote> for AllowedUpdate {
    fn from(value: Remote) -> Self {
        match value {
            Remote::Message => Self::Message,
            Remote::EditedMessage => Self::EditedMessage,
            Remote::ChannelPost => Self::ChannelPost,
            Remote::EditedChannelPost => Self::EditedChannelPost,
            Remote::InlineQuery => Self::InlineQuery,
            Remote::ChosenInlineResult => Self::ChosenInlineResult,
            Remote::CallbackQuery => Self::CallbackQuery,
            Remote::ShippingQuery => Self::ShippingQuery,
            Remote::PreCheckoutQuery => Self::PreCheckoutQuery,
            Remote::Poll => Self::Poll,
            Remote::PollAnswer => Self::PollAnswer,
            Remote::MyChatMember => Self::MyChatMember,
            Remote::ChatMember => Self::ChatMember,
            Remote::ChatJoinRequest => Self::ChatJoinRequest,
        }
    }
}

impl From<AllowedUpdate> for Remote {
    fn from(value: AllowedUpdate) -> Self {
        match value {
            AllowedUpdate::Message => Self::Message,
            AllowedUpdate::EditedMessage => Self::EditedMessage,
            AllowedUpdate::ChannelPost => Self::ChannelPost,
            AllowedUpdate::EditedChannelPost => Self::EditedChannelPost,
            AllowedUpdate::InlineQuery => Self::InlineQuery,
            AllowedUpdate::ChosenInlineResult => Self::ChosenInlineResult,
            AllowedUpdate::CallbackQuery => Self::CallbackQuery,
            AllowedUpdate::ShippingQuery => Self::ShippingQuery,
            AllowedUpdate::PreCheckoutQuery => Self::PreCheckoutQuery,
            AllowedUpdate::Poll => Self::Poll,
            AllowedUpdate::PollAnswer => Self::PollAnswer,
            AllowedUpdate::MyChatMember => Self::MyChatMember,
            AllowedUpdate::ChatMember => Self::ChatMember,
            AllowedUpdate::ChatJoinRequest => Self::ChatJoinRequest,
        }
    }
}
