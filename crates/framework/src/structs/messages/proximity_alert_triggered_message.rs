use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggeredMessage {
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

impl From<Message> for ProximityAlertTriggeredMessage {
    fn from(remote: Message) -> Self {
        let Message {
            proximity_alert_triggered,
            ..
        } = remote;

        Self {
            proximity_alert_triggered: proximity_alert_triggered.unwrap(),
        }
    }
}
