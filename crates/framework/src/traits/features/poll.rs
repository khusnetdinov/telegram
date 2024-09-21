use crate::enums::chat_uid::ChatUId;
use crate::structs::options::Options;
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
        poll_options: PollOptions,
        input_poll_options: Vec<InputPollOption>,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    // async fn stop_poll(&self, params: &StopPoll) -> Result<Poll, Error>;
}
