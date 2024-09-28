use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::messages::message_id::MessageId;
use crate::structs::polls::input_poll_option::InputPollOption;
use crate::structs::polls::options::Options as PollOptions;
use crate::structs::updates::message::Message;
use crate::structs::updates::poll::Poll as Response;
use crate::traits::features::poll::Poll;
use telegram_bots_api::api::params::send_poll::SendPoll;
use telegram_bots_api::api::params::stop_poll::StopPoll;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Poll for BotsApi {
    async fn send_poll(
        &self,
        chat_id: ChatUId,
        question: String,
        kind: String,
        input_poll_options: Vec<InputPollOption>,
        poll_options: PollOptions,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let PollOptions {
            question_entities,
            question_parse_mode,
            explanation_entities,
            explanation_parse_mode,
            allows_multiple_answers,
            is_anonymous,
            is_closed,
            correct_option_id,
            open_period,
            close_date,
            explanation,
            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters,
            reply_markup,
            ..
        } = poll_options;

        let params = SendPoll {
            chat_id: chat_id.into(),
            question,
            kind: Some(kind),

            question_entities: question_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            question_parse_mode,
            explanation_entities: explanation_entities
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            explanation_parse_mode,
            allows_multiple_answers,
            is_anonymous,
            is_closed,
            correct_option_id,
            open_period,
            close_date,
            explanation,
            options: input_poll_options
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),

            business_connection_id,
            disable_notification,
            protect_content,
            message_effect_id,
            message_thread_id,
            reply_parameters: reply_parameters.map(|inner| inner.into()),
            reply_markup: reply_markup.map(|inner| inner.into()),
        };

        Ok(self.client.send_poll(&params).await?.into())
    }

    async fn stop_poll(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        options: PollOptions,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let PollOptions {
            business_connection_id,
            inline_keyboard_markup,
            ..
        } = options;

        let params = StopPoll {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            business_connection_id,
            reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
        };

        Ok(self.client.stop_poll(&params).await?.into())
    }
}
