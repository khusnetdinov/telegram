use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::message::Message as Inner;
use telegram_bots_api::api::structs::write_access_allowed::WriteAccessAllowed;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteAccessAllowedMessage {
    pub write_access_allowed: WriteAccessAllowed,
}

impl From<Inner> for WriteAccessAllowedMessage {
    fn from(inner: Inner) -> Self {
        let Inner {
            write_access_allowed,
            ..
        } = inner;

        Self {
            write_access_allowed: write_access_allowed.unwrap(),
        }
    }
}
