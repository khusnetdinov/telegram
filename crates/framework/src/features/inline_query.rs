use crate::bots_api::BotsApi;
use crate::enums::inline_query_result::InlineQueryResult;
use crate::structs::inline_query_results::inline_query_results_button::InlineQueryResultsButton;
use crate::traits::features::inline_query::InlineQuery;
use telegram_bots_api::api::params::answer_inline_query::AnswerInlineQuery;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl InlineQuery for BotsApi {
    async fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
        cache_time: Option<i64>,
        is_personal: Option<bool>,
        next_offset: Option<String>,
        button: Option<InlineQueryResultsButton>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = AnswerInlineQuery {
            inline_query_id,
            results: results
                .iter()
                .map(|inner| inner.to_owned().into())
                .collect(),
            cache_time,
            is_personal,
            next_offset,
            button: button.map(|inner| inner.into()),
        };

        Ok(self.client.answer_inline_query(&params).await?)
    }
}
