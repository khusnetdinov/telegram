use crate::enums::chat_uid::ChatUId;
use crate::structs::messages::message_id::MessageId;
use crate::structs::poll::Poll as Response;
use crate::structs::polls::input_poll_option::InputPollOption;
use crate::structs::polls::options::Options as PollOptions;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Poll {
    async fn send_poll(
        &self,
        chat_id: ChatUId,
        question: String,
        kind: String,
        input_poll_options: Vec<InputPollOption>,
        poll_options: PollOptions,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    async fn stop_poll(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        options: PollOptions,
    ) -> Result<Response, Box<dyn std::error::Error>>;
}
