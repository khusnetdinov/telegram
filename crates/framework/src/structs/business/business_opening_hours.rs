use crate::structs::business::business_opening_hours_interval::BusinessOpeningHoursInterval;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_opening_hours::BusinessOpeningHours as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
impl From<Remote> for BusinessOpeningHours {
    fn from(remote: Remote) -> Self {
        Self {
            time_zone_name: remote.time_zone_name,
            // TODO: #[remote(map, into)]
            opening_hours: remote
                .opening_hours
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
        }
    }
}
