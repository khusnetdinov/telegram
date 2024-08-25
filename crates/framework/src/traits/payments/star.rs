#[async_trait::async_trait]
pub trait Star {
    async fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
