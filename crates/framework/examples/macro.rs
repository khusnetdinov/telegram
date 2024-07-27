use telegram_bots_api::api::structs::business_opening_hours::BusinessOpeningHours as Remote;
use telegram_bots_api::api::structs::business_opening_hours_interval::BusinessOpeningHoursInterval;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
