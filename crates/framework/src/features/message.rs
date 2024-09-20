use crate::bots_api::BotsApi;
use crate::traits::message::Message;
// use telegram_bots_api::api::params::send_message::SendMessage;

#[async_trait::async_trait]
impl Message for BotsApi {
    // async fn send_message(
    //     &self,
    //     chat_id: i64,
    //     text: String,
    //     message_options: MesageOptions
    // ) -> Result<Message, Box<dyn std::error::Error>> {
    //     let params = SendMessage {
    //
    //     };
    // }
}
