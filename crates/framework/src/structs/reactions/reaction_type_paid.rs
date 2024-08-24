use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::reaction_type_paid::ReactionTypePaid as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ReactionTypePaid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}
