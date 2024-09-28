use crate::structs::geo::venue::Venue;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

pub type IncomingVenue = Venue;

impl From<IncomingMessage> for IncomingVenue {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { venue, .. } = remote;

        Self::from(venue.unwrap())
    }
}
