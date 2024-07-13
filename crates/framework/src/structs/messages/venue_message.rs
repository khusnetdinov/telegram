use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::venue::Venue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VenueMessage {
    pub venue: Venue,
}

impl From<Message> for VenueMessage {
    fn from(remote: Message) -> Self {
        let Message { venue, .. } = remote;

        Self {
            venue: venue.unwrap(),
        }
    }
}
