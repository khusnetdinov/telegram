use crate::enums::inline_query_result::InlineQueryResult;
use crate::structs::web_apps::sent_web_app_message::SentWebAppMessage;

#[async_trait::async_trait]
pub trait WebApp {
    async fn answer_web_app_query(
        &self,
        web_app_query_id: String,
        result: InlineQueryResult,
    ) -> Result<SentWebAppMessage, Box<dyn std::error::Error>>;
}
