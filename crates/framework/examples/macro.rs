use telegram_bots_api::api::enums::reaction_type::ReactionType;
use telegram_bots_api::api::structs::chat::Chat;
use telegram_bots_api::api::structs::message_id::MessageId;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: MessageId,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
}
