use crate::enums::chat_uid::ChatUId;
use crate::structs::message::Message;
use crate::structs::payments::labeled_price::LabeledPrice;
use crate::structs::payments::options::Options;

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
        chat_id: ChatUId,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
        options: Options,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
