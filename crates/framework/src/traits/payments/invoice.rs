use crate::structs::payments::labeled_price::LabeledPrice;
use crate::structs::payments::options::Options;
use telegram_bots_api::api::structs::message::Message;

#[async_trait::async_trait]
pub trait Invoice {
    async fn create_invoice_link(
        &self,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
        options: Options,
    ) -> Result<String, Box<dyn std::error::Error>>;

    async fn send_invoice(
        &self,
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
        options: Options,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
