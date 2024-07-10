use telegram_bots_api::api::structs::shipping_address::ShippingAddress;
use telegram_bots_api::api::structs::shipping_query::ShippingQuery as Remote;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
