use crate::bots_api::BotsApi;
use crate::enums::passport_element_error::PassportElementError;
use crate::traits::features::passport::Passport;
use telegram_bots_api::api::params::set_passport_data_errors::SetPassportDataErrors;
use telegram_bots_api::api::requests::r#async::Requests;

#[async_trait::async_trait]
impl Passport for BotsApi {
    async fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<PassportElementError>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = SetPassportDataErrors {
            user_id,
            errors: errors.iter().map(|inner| inner.clone().into()).collect(),
        };

        Ok(self.client.set_passport_data_errors(&params).await?)
    }
}
