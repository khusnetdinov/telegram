use crate::structs::business::business_connection::BusinessConnection;
use crate::structs::callback_query::CallbackQuery;
use crate::structs::chats::chat_join_request::ChatJoinRequest;
use crate::structs::inline_query::InlineQuery;
use crate::structs::message::Message;
use crate::structs::payments::pre_checkout_query::PreCheckoutQuery;
use crate::structs::payments::shippings::shipping_query::ShippingQuery;
use crate::structs::poll::Poll;
use crate::structs::polls::poll_answer::PollAnswer;
use crate::structs::updates::business_message_deleted::BusinessMessagesDeleted;
use crate::structs::updates::chat_boost_removed::ChatBoostRemoved;
use crate::structs::updates::chat_boost_updated::ChatBoostUpdated;
use crate::structs::updates::chat_member_updated::ChatMemberUpdated;
use crate::structs::updates::inline_result_chosen::ChosenInlineResult;
use crate::structs::updates::message_reaction_count_updated::MessageReactionCountUpdated;
use crate::structs::updates::message_reaction_updated::MessageReactionUpdated;
use crate::structs::updates::paid_media_purchased::PaidMediaPurchased;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::update::Update as Remote;

/// <https://core.telegram.org/bots/api#update>
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Updates {
    /// Optional. New incoming message of any kind - text, photo, sticker, etc.
    Message(Message),
    /// Optional. New version of a message that is known to the bot and was edited. This update
    /// may at times be triggered by changes to message fields that are either unavailable or not
    /// actively used by your bot.
    EditedMessage(Message),
    /// Optional. New incoming channel post of any kind - text, photo, sticker, etc.
    ChannelPost(Message),
    /// Optional. New version of a channel post that is known to the bot and was edited. This
    /// update may at times be triggered by changes to message fields that are either unavailable
    /// or not actively used by your bot.
    EditedChannelPost(Message),
    /// Optional. The bot was connected to or disconnected from a business account, or a user
    /// edited an existing connection with the bot
    BusinessConnection(BusinessConnection),
    /// Optional. New message from a connected business account
    BusinessMessage(Message),
    /// Optional. New version of a message from a connected business account
    EditedBusinessMessage(Message),
    /// Optional. Messages were deleted from a connected business account
    DeletedBusinessMessages(BusinessMessagesDeleted),
    /// Optional. A reaction to a message was changed by a user. The bot must be an administrator
    /// in the chat and must explicitly specify "message_reaction" in the list of allowed_updates
    /// to receive these updates. The update isn't received for reactions set by bots.
    MessageReaction(MessageReactionUpdated),
    /// Optional. Reactions to a message with anonymous reactions were changed. The bot must be an
    /// administrator in the chat and must explicitly specify "message_reaction_count" in the list
    /// of allowed_updates to receive these updates. The updates are grouped and can be sent with
    /// delay up to a few minutes.
    MessageReactionCount(MessageReactionCountUpdated),
    /// Optional. New incoming inline query
    InlineQuery(InlineQuery),
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat
    /// partner. Please see our documentation on the feedback collecting for details on how to
    /// enable these updates for your bot.
    ChosenInlineResult(ChosenInlineResult),
    /// Optional. New incoming callback query
    CallbackQuery(CallbackQuery),
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
    /// Optional. A user purchased paid media with a non-empty payload sent by the bot in a
    /// non-channel chat
    PaidMediaPurchased(PaidMediaPurchased),
    /// Optional. New poll state. Bots receive only updates about manually stopped polls and polls,
    /// which are sent by the bot
    Poll(Poll),
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only
    /// in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),
    /// Optional. The bot's chat member status was updated in a chat. For private chats, this
    /// update is received only when the bot is blocked or unblocked by the user.
    MyChatMember(ChatMemberUpdated),
    /// Optional. A chat member's status was updated in a chat. The bot must be an administrator
    /// in the chat and must explicitly specify "chat_member" in the list of allowed_updates to
    /// receive these updates.
    ChatMember(ChatMemberUpdated),
    /// Optional. A request to join the chat has been sent. The bot must have the can_invite_users
    /// administrator right in the chat to receive these updates.
    ChatJoinRequest(ChatJoinRequest),
    /// Optional. A chat boost was added or changed. The bot must be an administrator in the chat
    /// to receive these updates.
    ChatBoost(ChatBoostUpdated),
    /// Optional. A boost was removed from a chat. The bot must be an administrator in the chat to
    /// receive these updates.
    RemovedChatBoost(ChatBoostRemoved),
    /// Not Telegram type: for unexpected messages, errors, debugging, logging purpose.
    Unexpected(Remote),
}

impl From<Remote> for Updates {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote {
                message: Some(message),
                ..
            } => Self::Message(Message::from(*message)),
            Remote {
                edited_message: Some(message),
                ..
            } => Self::EditedMessage(Message::from(*message)),
            Remote {
                channel_post: Some(message),
                ..
            } => Self::ChannelPost(Message::from(*message)),
            Remote {
                edited_channel_post: Some(message),
                ..
            } => Self::EditedChannelPost(Message::from(*message)),
            Remote {
                business_connection: Some(business_connection),
                ..
            } => Self::BusinessConnection(BusinessConnection::from(business_connection)),
            Remote {
                business_message: Some(message),
                ..
            } => Self::BusinessMessage(Message::from(message)),
            Remote {
                edited_business_message: Some(message),
                ..
            } => Self::EditedBusinessMessage(Message::from(message)),
            Remote {
                deleted_business_messages: Some(deleted_business_messages),
                ..
            } => Self::DeletedBusinessMessages(BusinessMessagesDeleted::from(
                deleted_business_messages,
            )),
            Remote {
                message_reaction: Some(message_reaction),
                ..
            } => Self::MessageReaction(MessageReactionUpdated::from(message_reaction)),
            Remote {
                message_reaction_count: Some(message_reaction_count),
                ..
            } => Self::MessageReactionCount(MessageReactionCountUpdated::from(
                message_reaction_count,
            )),
            Remote {
                inline_query: Some(inline_query),
                ..
            } => Self::InlineQuery(InlineQuery::from(inline_query)),
            Remote {
                chosen_inline_result: Some(chosen_inline_result),
                ..
            } => Self::ChosenInlineResult(ChosenInlineResult::from(chosen_inline_result)),
            Remote {
                callback_query: Some(callback_query),
                ..
            } => Self::CallbackQuery(CallbackQuery::from(callback_query)),
            Remote {
                shipping_query: Some(shipping_query),
                ..
            } => Self::ShippingQuery(ShippingQuery::from(shipping_query)),
            Remote {
                pre_checkout_query: Some(pre_checkout_query),
                ..
            } => Self::PreCheckoutQuery(PreCheckoutQuery::from(pre_checkout_query)),
            Remote {
                purchased_paid_media: Some(purchased_paid_media),
                ..
            } => Self::PaidMediaPurchased(PaidMediaPurchased::from(purchased_paid_media)),
            Remote {
                poll: Some(poll), ..
            } => Self::Poll(Poll::from(poll)),
            Remote {
                poll_answer: Some(poll_answer),
                ..
            } => Self::PollAnswer(PollAnswer::from(poll_answer)),
            Remote {
                my_chat_member: Some(my_chat_member),
                ..
            } => Self::MyChatMember(ChatMemberUpdated::from(my_chat_member)),
            Remote {
                chat_member: Some(chat_member),
                ..
            } => Self::ChatMember(ChatMemberUpdated::from(chat_member)),
            Remote {
                chat_join_request: Some(chat_join_request),
                ..
            } => Self::ChatJoinRequest(ChatJoinRequest::from(chat_join_request)),
            Remote {
                chat_boost: Some(chat_boost),
                ..
            } => Self::ChatBoost(ChatBoostUpdated::from(chat_boost)),
            Remote {
                removed_chat_boost: Some(removed_chat_boost),
                ..
            } => Self::RemovedChatBoost(ChatBoostRemoved::from(removed_chat_boost)),
            _ => Self::Unexpected(remote),
        }
    }
}
