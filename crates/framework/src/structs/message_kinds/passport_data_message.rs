use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::passport_data::PassportData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportDataMessage {
    pub passport_data: PassportData,
}

impl From<Inner> for PassportDataMessage {
    fn from(inner: Inner) -> Self {
        let Inner { passport_data, .. } = inner;

        Self {
            passport_data: passport_data.unwrap(),
        }
    }
}
