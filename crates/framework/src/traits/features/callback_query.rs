#[async_trait::async_trait]
pub trait CallbackQuery {
    async fn answer_callback_query(
        &self,
        callback_query_id: String,
        url: Option<String>,
        text: Option<String>,
        show_alert: Option<bool>,
        cache_time: Option<i64>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
