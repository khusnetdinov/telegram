use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProximityAlertTriggeredMessage {
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

impl From<Inner> for ProximityAlertTriggeredMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            proximity_alert_triggered,
            ..
        } = inner;

        Self {
            proximity_alert_triggered: proximity_alert_triggered.unwrap(),
        }
    }
}
