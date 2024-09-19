use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_custom_emoji::ReactionTypeCustomEmoji as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ReactionTypeCustomEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub custom_emoji_id: String,
}
