use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::location::Location as Remote;
use telegram_bots_api::api::structs::message::Message;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}

impl From<Message> for Location {
    fn from(remote: Message) -> Self {
        let Message {
            location:
                Some(Remote {
                    longitude,
                    latitude,
                    horizontal_accuracy,
                    live_period,
                    heading,
                    proximity_alert_radius,
                    ..
                }),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self {
            longitude,
            latitude,
            horizontal_accuracy,
            live_period,
            heading,
            proximity_alert_radius,
        }
    }
}
