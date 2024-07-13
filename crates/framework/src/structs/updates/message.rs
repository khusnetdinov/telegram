use crate::enums::messages::Messages;
use crate::traits::kind_dispatcher::KindDispatcher;
use telegram_bots_api::api::enums::message_origin::MessageOrigin;
use telegram_bots_api::api::structs::chat::Chat;
use telegram_bots_api::api::structs::external_reply_info::ExternalReplyInfo;
use telegram_bots_api::api::structs::inline_keyboard_markup::InlineKeyboardMarkup;
use telegram_bots_api::api::structs::message::Message as Remote;
use telegram_bots_api::api::structs::message_id::MessageId;
use telegram_bots_api::api::structs::story::Story;
use telegram_bots_api::api::structs::text_quote::TextQuote;
use telegram_bots_api::api::structs::user::User;

#[derive(Debug, Clone)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Optional. Unique identifier of a message thread to which the message belongs; for supergroups only
    pub message_thread_id: Option<i64>,
    /// Optional. Sender of the message; empty for messages sent to channels. For backward compatibility,
    /// the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Optional. Sender of the message, sent on behalf of a chat. For example, the channel itself for
    /// channel posts, the supergroup itself for messages from anonymous group administrators, the linked
    /// channel for messages automatically forwarded to the discussion group. For backward compatibility,
    /// the field from contains a fake sender user in non-channel chats, if the message was sent on
    /// behalf of a chat.
    pub sender_chat: Option<Box<Chat>>,
    /// Optional. If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
    /// Optional. The bot that actually sent the message on behalf of the business account.
    /// Available only for outgoing messages sent on behalf of the connected business account.
    pub sender_business_bot: Option<Box<User>>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,
    /// Optional. Unique identifier of the business connection from which the message was received.
    /// If non-empty, the message belongs to a chat of the corresponding business account that is
    /// independent from any potential bot chat which might share the same identifier.
    pub business_connection_id: Option<String>,
    /// Chat the message belongs to
    pub chat: Chat,
    /// Optional. Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// Optional. True, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Optional. True, if the message is a channel post that was automatically forwarded to the
    /// connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// Optional. For replies in the same chat and message thread, the original message. Note that
    /// the Message object in this field will not contain further reply_to_message fields even if
    /// it itself is a reply.
    pub reply_to_message: Option<Box<Remote>>,
    /// Optional. Information about the message that is being replied to, which may come from another
    /// chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Optional. For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Optional. For replies to a story, the original story
    pub reply_to_story: Option<Box<Story>>,
    /// Optional. Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Optional. Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// Optional. True, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Optional. True, if the message was sent by an implicit action, for example, as an away or
    /// a greeting business message, or as a scheduled message
    pub is_from_offline: Option<bool>,
    /// Optional. Signature of the post author for messages in channels, or the custom title of
    /// an anonymous group administrator
    pub author_signature: Option<String>,
    /// Optional. Unique identifier of the message effect added to the message
    pub effect_id: Option<String>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are represented as
    /// ordinary url buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Not Telegram type: wrap raw struct with dispatched enum variant
    pub kind: Messages,
}

impl From<Remote> for Message {
    fn from(remote: Remote) -> Self {
        let Remote {
            message_id: MessageId { message_id },
            message_thread_id,
            from,
            sender_chat,
            sender_boost_count,
            sender_business_bot,
            date,
            business_connection_id,
            chat,
            forward_origin,
            is_topic_message,
            is_automatic_forward,
            reply_to_message,
            external_reply,
            quote,
            reply_to_story,
            via_bot,
            edit_date,
            has_protected_content,
            is_from_offline,
            author_signature,
            effect_id,
            reply_markup,
            ..
        } = remote.clone();

        Self {
            message_id,
            message_thread_id,
            chat,
            from,
            sender_chat,
            sender_boost_count,
            sender_business_bot,
            forward_origin,
            is_topic_message,
            is_automatic_forward,
            reply_to_message,
            external_reply,
            quote,
            reply_to_story,
            business_connection_id,
            date,
            via_bot,
            edit_date,
            has_protected_content,
            is_from_offline,
            author_signature,
            effect_id,
            reply_markup,
            kind: Messages::from(remote),
        }
    }
}

impl KindDispatcher for Message {
    type Kind = Messages;

    fn dispatch(&self) -> &Self::Kind {
        &self.kind
    }
}
