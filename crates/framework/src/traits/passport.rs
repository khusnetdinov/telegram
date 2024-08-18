use crate::enums::passport_element_error::PassportElementError;

#[async_trait::async_trait]
pub trait Passport {
    async fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<PassportElementError>,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
