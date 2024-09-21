use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}

impl From<IncomingMessage> for ProximityAlertTriggered {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            proximity_alert_triggered,
            ..
        } = remote;

        Self::from(proximity_alert_triggered.unwrap())
    }
}
