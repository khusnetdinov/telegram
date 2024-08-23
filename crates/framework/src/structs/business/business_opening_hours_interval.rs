use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_opening_hours_interval::BusinessOpeningHoursInterval as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: u16,
    pub closing_minute: u16,
}
