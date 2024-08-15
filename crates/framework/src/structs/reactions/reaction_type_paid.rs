use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_paid::ReactionTypePaid as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReactionTypePaid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
impl From<Remote> for ReactionTypePaid {
    fn from(remote: Remote) -> Self {
        Self { kind: remote.kind }
    }
}
