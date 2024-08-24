use crate::bots_api::BotsApi;
use crate::traits::star::Star;
use std::error::Error;
use telegram_bots_api::api::params::refund_star_payment::RefundStarPayment;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Star for BotsApi {
    async fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: String,
    ) -> Result<bool, Box<dyn Error>> {
        let params = RefundStarPayment {
            user_id,
            telegram_payment_charge_id
        };

        Ok(self.client.refund_star_payment(&params).await?)
    }
}
