use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_emoji::ReactionTypeEmoji as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct ReactionTypeEmoji {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub emoji: String,
}
