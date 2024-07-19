use crate::bots_api::BotsApi;
use crate::structs::options::Options;
use crate::structs::polls::input_poll_option::InputPollOption;
use crate::structs::polls::options::Options as PollOptions;
use crate::structs::updates::message::Message;
use crate::traits::poll::Poll;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_poll::SendPoll;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Poll for BotsApi {
    async fn send_poll(
        &self,
        chat_id: i64,
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
                chat_id: ChatUId::from(chat_id),
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
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
                ..Default::default()
            }
        } else {
            SendPoll {
                chat_id: ChatUId::from(chat_id),
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
}
