use crate::structs::geo::location::Location;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;

pub type IncomingLocation = Location;

impl From<IncomingMessage> for IncomingLocation {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { location, .. } = remote;

        Self::from(location.unwrap())
    }
}
