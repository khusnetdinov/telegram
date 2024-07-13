use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}
impl From<Remote> for ProximityAlertTriggered {
    fn from(remote: Remote) -> Self {
        Self {
            // TODO: #[remote(into)]
            traveler: remote.traveler.into(),
            // TODO: #[remote(into)]
            watcher: remote.watcher.into(),
            distance: remote.distance,
        }
    }
}

impl From<Message> for ProximityAlertTriggered {
    fn from(remote: Message) -> Self {
        let Message {
            proximity_alert_triggered: Some(proximity_alert_triggered),
            ..
        } = remote
        else {
            unreachable!()
        };

        Self::from(proximity_alert_triggered)
    }
}
