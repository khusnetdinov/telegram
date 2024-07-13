// use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message;
use telegram_bots_api::api::structs::proximity_alert_triggered::ProximityAlertTriggered as Remote;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(FromRemote)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}
