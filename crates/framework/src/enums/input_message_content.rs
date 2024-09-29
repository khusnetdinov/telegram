use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::input_message_content::InputMessageContent as Remote;

use crate::structs::inline_query_results::input_message_contents::input_contact_message_content::InputContactMessageContent;
use crate::structs::inline_query_results::input_message_contents::input_invoice_message_content::InputInvoiceMessageContent;
use crate::structs::inline_query_results::input_message_contents::input_location_message_content::InputLocationMessageContent;
use crate::structs::inline_query_results::input_message_contents::input_text_message_content::InputTextMessageContent;
use crate::structs::inline_query_results::input_message_contents::input_venue_message_content::InputVenueMessageContent;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}

impl Default for InputMessageContent {
    fn default() -> Self {
        Self::InputTextMessageContent(InputTextMessageContent {
            ..Default::default()
        })
    }
}
