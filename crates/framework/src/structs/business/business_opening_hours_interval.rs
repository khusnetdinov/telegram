use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_opening_hours_interval::BusinessOpeningHoursInterval as Remote;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,
    pub closing_minute: u16,
}

impl From<Remote> for BusinessOpeningHoursInterval {
    fn from(remote: Remote) -> Self {
        Self {
            opening_minute: remote.opening_minute,
            closing_minute: remote.closing_minute,
        }
    }
}
