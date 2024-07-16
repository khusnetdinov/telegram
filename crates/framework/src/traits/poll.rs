use crate::structs::options::Options;
use crate::structs::poll::Poll as Send;
use crate::structs::updates::message::Message;
use telegram_bots_api::api::structs::input_poll_option::InputPollOption;

#[async_trait::async_trait]
pub trait Poll {
    async fn send_poll(
        &self,
        chat_id: i64,
        poll: Send,
        poll_options: Vec<InputPollOption>,
        parse_mode: Option<String>,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;
}
