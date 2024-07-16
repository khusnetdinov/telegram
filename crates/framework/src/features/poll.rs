use crate::bots_api::BotsApi;
use crate::feature::bots_api::Options;
use crate::structs::poll::Poll as Send;
use crate::structs::updates::message::Message;
use crate::traits::poll::Poll;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::send_poll::SendPoll;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::api::structs::input_poll_option::InputPollOption;

#[async_trait::async_trait]
impl Poll for BotsApi {
    async fn send_poll(
        &self,
        chat_id: i64,
        poll: Send,
        poll_options: Vec<InputPollOption>,
        parse_mode: Option<String>,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let params = if let Some(options) = options {
            SendPoll {
                chat_id: ChatUId::from(chat_id),
                kind: Some(poll.kind),
                options: poll_options,
                question: poll.question,
                // TODO: #[remote(option, map, into)]
                question_entities: poll
                    .question_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                question_parse_mode: parse_mode.clone(),
                explanation: poll.explanation,
                // TODO: #[remote(option, map, into)]
                explanation_entities: poll
                    .explanation_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                explanation_parse_mode: parse_mode.clone(),
                allows_multiple_answers: Some(poll.allows_multiple_answers),
                is_anonymous: Some(poll.is_anonymous),
                is_closed: Some(poll.is_closed),
                correct_option_id: poll.correct_option_id,
                open_period: poll.open_period,
                close_date: poll.close_date,
                business_connection_id: options.business_connection_id,
                disable_notification: options.disable_notification,
                protect_content: options.protect_content,
                message_effect_id: options.message_effect_id,
                message_thread_id: options.message_thread_id,
                reply_parameters: options.reply_parameters,
                reply_markup: options.reply_markup,
            }
        } else {
            SendPoll {
                chat_id: ChatUId::from(chat_id),
                kind: Some(poll.kind),
                options: poll_options,
                question: poll.question,
                // TODO: #[remote(option, map, into)]
                question_entities: poll
                    .question_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                question_parse_mode: parse_mode.clone(),
                explanation: poll.explanation,
                // TODO: #[remote(option, map, into)]
                explanation_entities: poll
                    .explanation_entities
                    .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
                explanation_parse_mode: parse_mode.clone(),
                allows_multiple_answers: Some(poll.allows_multiple_answers),
                is_anonymous: Some(poll.is_anonymous),
                is_closed: Some(poll.is_closed),
                correct_option_id: poll.correct_option_id,
                open_period: poll.open_period,
                close_date: poll.close_date,
                ..Default::default()
            }
        };

        Ok(self.client.send_poll(&params).await?.into())
    }
}
