use crate::bots_api::BotsApi;
use crate::enums::inline_query_result::InlineQueryResult;
use crate::structs::web_apps::sent_web_app_message::SentWebAppMessage;
use crate::traits::features::web_app::WebApp;
use telegram_bots_api::api::params::answer_web_app_query::AnswerWebAppQuery;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl WebApp for BotsApi {
    async fn answer_web_app_query(
        &self,
        web_app_query_id: String,
        result: InlineQueryResult,
    ) -> Result<SentWebAppMessage, Box<dyn std::error::Error>> {
        let params = AnswerWebAppQuery {
            web_app_query_id,
            result: result.into(),
        };

        Ok(self.client.answer_web_app_query(&params).await?.into())
    }
}
