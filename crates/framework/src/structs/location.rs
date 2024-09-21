use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::location::Location as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}

impl From<IncomingMessage> for Location {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage { location, .. } = remote;

        Self::from(location.unwrap())
    }
}
