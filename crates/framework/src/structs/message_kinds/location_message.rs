use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::location::Location;
use telegram_bots_api::api::structs::message::Message as Inner;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationMessage {
    pub location: Location,
}

impl From<Inner> for LocationMessage {
    fn from(inner: Inner) -> Self {
        let Inner { location, .. } = inner;

        Self {
            location: location.unwrap(),
        }
    }
}
