use crate::structs::payments::shippings::shipping_option::ShippingOption;

#[async_trait::async_trait]
pub trait Order {
    async fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
        error_message: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
        shipping_options: Option<Vec<ShippingOption>>,
        error_message: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
