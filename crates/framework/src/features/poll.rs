use crate::bots_api::BotsApi;

// use crate::feature::bots_api::Options;
// use crate::structs::polls::input_poll_option::InputPollOption;
// use crate::structs::updates::message::Message;
use crate::traits::poll::Poll;

#[async_trait::async_trait]
impl Poll for BotsApi {
    // async fn send_poll(
    //     &self,
    //     chat_id: i64,
    //     question: String,
    //     poll_options: Vec<InputPollOption>,
    //     options: Option<Options>,
    // ) -> Result<Message, Box<dyn std::error::Error>> {
    //     todo!()
    // }
}
