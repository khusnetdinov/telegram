use telegram_bots_api::api::structs::message_reaction_count_update::MessageReactionCountUpdated as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct MessageReactionCountUpdated {
    inner: Inner,
}
