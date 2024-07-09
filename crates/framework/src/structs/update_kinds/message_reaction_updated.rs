use telegram_bots_api::api::structs::message_reaction_updated::MessageReactionUpdated as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, Clone, DerefInner, FromInner)]
pub struct MessageReactionUpdated {
    inner: Inner,
}
