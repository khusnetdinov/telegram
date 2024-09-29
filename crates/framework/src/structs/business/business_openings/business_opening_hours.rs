use crate::structs::business::business_openings::business_opening_hours_interval::BusinessOpeningHoursInterval;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::business_opening_hours::BusinessOpeningHours as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
