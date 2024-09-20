use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}

impl From<Message> for ProximityAlertTriggered {
    fn from(remote: Message) -> Self {
        let Message {
            proximity_alert_triggered,
            ..
        } = remote;

        Self::from(proximity_alert_triggered.unwrap())
    }
}
