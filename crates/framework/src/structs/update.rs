use crate::enums::update_kind::UpdateKind;
use telegram_bots_api::api::structs::update::Update as Inner;

/// <https://core.telegram.org/bots/api#update>
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and
    /// increase sequentially. This identifier becomes especially handy if you're using webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates for at least a week, then
    /// identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Not Telegram type: wrap raw struct with dispatched enum variant
    pub kind: UpdateKind,
}

impl From<Inner> for Update {
    fn from(inner: Inner) -> Self {
        Self {
            update_id: inner.update_id,
            kind: UpdateKind::from(inner),
        }
    }
}
