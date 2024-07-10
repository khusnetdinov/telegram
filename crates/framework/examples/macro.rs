use telegram_bots_api::api::structs::order_info::OrderInfo;
use telegram_bots_api::api::structs::pre_checkout_query::PreCheckoutQuery as Remote;
use telegram_bots_api::api::structs::user::User;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
