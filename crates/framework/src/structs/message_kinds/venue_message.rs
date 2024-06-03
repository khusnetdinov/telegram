use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::venue::Venue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VenueMessage {
    pub venue: Venue,
}

impl From<Inner> for VenueMessage {
    fn from(inner: Inner) -> Self {
        let Inner { venue, .. } = inner;

        Self {
            venue: venue.unwrap(),
        }
    }
}
