use crate::bots_api::BotsApi;
use crate::enums::chat_uid::ChatUId;
use crate::structs::messages::message_id::MessageId;
use crate::structs::options::Options;
use crate::structs::poll::Poll as Response;
use crate::structs::polls::input_poll_option::InputPollOption;
use crate::structs::polls::options::Options as PollOptions;
use crate::structs::updates::message::Message;
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
        poll_options: PollOptions,
        input_poll_options: Vec<InputPollOption>,
        options: Option<Options>,
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
            ..
        } = poll_options;

        let params = if let Some(options) = options {
            SendPoll {
                chat_id: chat_id.into(),
                kind: Some(kind),
                question,
                // TODO: #[remote(option, map, into)]
                question_entities: question_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                question_parse_mode,
                explanation,
                // TODO: #[remote(option, map, into)]
                explanation_entities: explanation_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                explanation_parse_mode,
                allows_multiple_answers,
                is_anonymous,
                is_closed,
                correct_option_id,
                open_period,
                close_date,
                // TODO: #[remote(option, map, into)]
                options: input_poll_options
                    .iter()
                    .map(|inner| inner.to_owned().into())
                    .collect(),
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters.map(|inner| inner.into()),
                reply_markup: options.reply_markup.map(|inner| inner.into()),
                ..Default::default()
            }
        } else {
            SendPoll {
                chat_id: chat_id.into(),
                kind: Some(kind),
                question,
                // TODO: #[remote(option, map, into)]
                question_entities: question_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                question_parse_mode,
                explanation,
                // TODO: #[remote(option, map, into)]
                explanation_entities: explanation_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                explanation_parse_mode,
                allows_multiple_answers,
                is_anonymous,
                is_closed,
                correct_option_id,
                open_period,
                close_date,
                // TODO: #[remote(option, map, into)]
                options: input_poll_options
                    .iter()
                    .map(|inner| inner.to_owned().into())
                    .collect(),
                ..Default::default()
            }
        };

        Ok(self.client.send_poll(&params).await?.into())
    }

    async fn stop_poll(
        &self,
        chat_id: ChatUId,
        message_id: MessageId,
        options: Option<Options>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            let Options {
                inline_keyboard_markup,
                business_connection_id,
                ..
            } = options;

            StopPoll {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                reply_markup: inline_keyboard_markup.map(|inner| inner.into()),
                business_connection_id,
            }
        } else {
            StopPoll {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                ..Default::default()
            }
        };

        Ok(self.client.stop_poll(&params).await?.into())
    }
}
