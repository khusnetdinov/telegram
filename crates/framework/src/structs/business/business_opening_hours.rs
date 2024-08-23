use crate::structs::business::business_opening_hours_interval::BusinessOpeningHoursInterval;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_opening_hours::BusinessOpeningHours as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
