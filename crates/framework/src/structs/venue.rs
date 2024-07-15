use crate::structs::location::Location;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::venue::Venue as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
impl From<Remote> for Venue {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            location: remote.location.into(),
            title: remote.title,
            address: remote.address,
            foursquare_id: remote.foursquare_id,
            foursquare_type: remote.foursquare_type,
            google_place_id: remote.google_place_id,
            google_place_type: remote.google_place_type,
        }
    }
}
impl From<Message> for Venue {
    fn from(remote: Message) -> Self {
        let Message {
            venue: Some(venue), ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(venue)
    }
}
