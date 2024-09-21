use crate::bots_api::BotsApi;
use crate::traits::features::callback_query::CallbackQuery;
use telegram_bots_api::api::params::answer_callback_query::AnswerCallbackQuery;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl CallbackQuery for BotsApi {
    async fn answer_callback_query(
        &self,
        callback_query_id: String,
        url: Option<String>,
        text: Option<String>,
        show_alert: Option<bool>,
        cache_time: Option<i64>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = AnswerCallbackQuery {
            callback_query_id,
            url,
            text,
            show_alert,
            cache_time,
        };

        Ok(self.client.answer_callback_query(&params).await?)
    }
}
