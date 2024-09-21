use crate::enums::inline_query_result::InlineQueryResult;
use crate::feature::inline::InlineQueryResultsButton;

#[async_trait::async_trait]
pub trait InlineQuery {
    async fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
        cache_time: Option<i64>,
        is_personal: Option<bool>,
        next_offset: Option<String>,
        button: Option<InlineQueryResultsButton>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
