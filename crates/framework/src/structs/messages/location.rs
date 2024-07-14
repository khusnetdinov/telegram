use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::location::Location as Remote;
use telegram_bots_api::api::structs::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
}
impl From<Remote> for Location {
    fn from(remote: Remote) -> Self {
        Self {
            longitude: remote.longitude,
            latitude: remote.latitude,
            horizontal_accuracy: remote.horizontal_accuracy,
            live_period: remote.live_period,
            heading: remote.heading,
            proximity_alert_radius: remote.proximity_alert_radius,
        }
    }
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
