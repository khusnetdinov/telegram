use crate::structs::business_connection::BusinessConnection;
use crate::structs::business_messages_deleted::BusinessMessagesDeleted;
use crate::structs::callback_query::CallbackQuery;
use crate::structs::chat_boost_removed::ChatBoostRemoved;
use crate::structs::chat_boost_updated::ChatBoostUpdated;
use crate::structs::chat_join_request::ChatJoinRequest;
use crate::structs::chat_member_updated::ChatMemberUpdated;
use crate::structs::chosen_inline_result::ChosenInlineResult;
use crate::structs::inline_query::InlineQuery;
use crate::structs::message::Message;
use crate::structs::message_reaction_count_updated::MessageReactionCountUpdated;
use crate::structs::message_reaction_updated::MessageReactionUpdated;
use crate::structs::poll::Poll;
use crate::structs::poll_answer::PollAnswer;
use crate::structs::pre_checkout_query::PreCheckoutQuery;
use crate::structs::shipping_query::ShippingQuery;
use telegram_bots_api::api::structs::update::Update as Inner;

/// <https://core.telegram.org/bots/api#update>
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug)]
pub enum UpdateKind {
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
    Unexpected,
}

impl From<Inner> for UpdateKind {
    fn from(inner: Inner) -> Self {
        match inner {
            Inner {
                message: Some(message),
                ..
            } => Self::Message(Message::from(*message)),
            Inner {
                edited_message: Some(message),
                ..
            } => Self::EditedMessage(Message::from(*message)),
            Inner {
                channel_post: Some(message),
                ..
            } => Self::ChannelPost(Message::from(*message)),
            Inner {
                edited_channel_post: Some(message),
                ..
            } => Self::EditedChannelPost(Message::from(*message)),
            Inner {
                business_connection: Some(business_connection),
                ..
            } => Self::BusinessConnection(BusinessConnection::from(business_connection)),
            Inner {
                business_message: Some(message),
                ..
            } => Self::BusinessMessage(Message::from(message)),
            Inner {
                edited_business_message: Some(message),
                ..
            } => Self::EditedBusinessMessage(Message::from(message)),
            Inner {
                deleted_business_messages: Some(deleted_business_messages),
                ..
            } => Self::DeletedBusinessMessages(BusinessMessagesDeleted::from(
                deleted_business_messages,
            )),
            Inner {
                message_reaction: Some(message_reaction),
                ..
            } => Self::MessageReaction(MessageReactionUpdated::from(message_reaction)),
            Inner {
                message_reaction_count: Some(message_reaction_count),
                ..
            } => Self::MessageReactionCount(MessageReactionCountUpdated::from(
                message_reaction_count,
            )),
            Inner {
                inline_query: Some(inline_query),
                ..
            } => Self::InlineQuery(InlineQuery::from(inline_query)),
            Inner {
                chosen_inline_result: Some(chosen_inline_result),
                ..
            } => Self::ChosenInlineResult(ChosenInlineResult::from(chosen_inline_result)),
            Inner {
                callback_query: Some(callback_query),
                ..
            } => Self::CallbackQuery(CallbackQuery::from(callback_query)),
            Inner {
                shipping_query: Some(shipping_query),
                ..
            } => Self::ShippingQuery(ShippingQuery::from(shipping_query)),
            Inner {
                pre_checkout_query: Some(pre_checkout_query),
                ..
            } => Self::PreCheckoutQuery(PreCheckoutQuery::from(pre_checkout_query)),
            Inner {
                poll: Some(poll), ..
            } => Self::Poll(Poll::from(poll)),
            Inner {
                poll_answer: Some(poll_answer),
                ..
            } => Self::PollAnswer(PollAnswer::from(poll_answer)),
            Inner {
                my_chat_member: Some(my_chat_member),
                ..
            } => Self::MyChatMember(ChatMemberUpdated::from(my_chat_member)),
            Inner {
                chat_member: Some(chat_member),
                ..
            } => Self::ChatMember(ChatMemberUpdated::from(chat_member)),
            Inner {
                chat_join_request: Some(chat_join_request),
                ..
            } => Self::ChatJoinRequest(ChatJoinRequest::from(chat_join_request)),
            Inner {
                chat_boost: Some(chat_boost),
                ..
            } => Self::ChatBoost(ChatBoostUpdated::from(chat_boost)),
            Inner {
                removed_chat_boost: Some(removed_chat_boost),
                ..
            } => Self::RemovedChatBoost(ChatBoostRemoved::from(removed_chat_boost)),
            _ => Self::Unexpected,
        }
    }
}
