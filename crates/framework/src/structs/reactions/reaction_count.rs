use crate::enums::reaction_type::ReactionType;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_count::ReactionCount as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ReactionCount {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: ReactionType,
    pub total_count: i64,
}
