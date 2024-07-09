use crate::enums::reaction_type::ReactionType;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_count::ReactionCount as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionCount {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: ReactionType,
    pub total_count: i64,
}
impl From<Remote> for ReactionCount {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            kind: remote.kind.into(),
            total_count: remote.total_count,
        }
    }
}
