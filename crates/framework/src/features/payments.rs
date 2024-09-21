use crate::bots_api::BotsApi;
use crate::structs::payments::labeled_price::LabeledPrice;
use crate::structs::payments::options::Options;
use crate::structs::payments::shipping_option::ShippingOption;
use crate::traits::features::payments::invoice::Invoice;
use crate::traits::features::payments::order::Order;
use crate::traits::features::payments::star::Star;
use telegram_bots_api::api::enums::chat_uid::ChatUId;
use telegram_bots_api::api::params::answer_pre_checkout_query::AnswerPreCheckoutQuery;
use telegram_bots_api::api::params::answer_shipping_query::AnswerShippingQuery;
use telegram_bots_api::api::params::create_invoice_link::CreateInvoiceLink;
use telegram_bots_api::api::params::refund_star_payment::RefundStarPayment;
use telegram_bots_api::api::params::send_invoice::SendInvoice;
use telegram_bots_api::api::requests::r#async::Requests;
use telegram_bots_api::api::structs::message::Message;

#[async_trait::async_trait]
impl Invoice for BotsApi {
    async fn create_invoice_link(
        &self,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
        options: Options,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let Options {
            provider_token,
            max_tip_amount,
            suggested_tip_amounts,
            provider_data,
            photo_url,
            photo_size,
            photo_width,
            photo_height,
            need_name,
            need_phone_number,
            need_email,
            need_shipping_address,
            send_phone_number_to_provider,
            send_email_to_provider,
            is_flexible,
            ..
        } = options;

        let params = CreateInvoiceLink {
            title,
            description,
            payload,
            currency,
            prices: prices.iter().map(|coll| coll.to_owned().into()).collect(),
            provider_token,
            max_tip_amount,
            suggested_tip_amounts,
            provider_data,
            photo_url,
            photo_size,
            photo_width,
            photo_height,
            need_name,
            need_phone_number,
            need_email,
            need_shipping_address,
            send_phone_number_to_provider,
            send_email_to_provider,
            is_flexible,
        };

        Ok(self.client.create_invoice_link(&params).await?)
    }

    async fn send_invoice(
        &self,
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        currency: String,
        prices: Vec<LabeledPrice>,
        options: Options,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let Options {
            provider_token,
            max_tip_amount,
            suggested_tip_amounts,
            provider_data,
            photo_url,
            photo_size,
            photo_width,
            photo_height,
            need_name,
            need_phone_number,
            need_email,
            need_shipping_address,
            send_phone_number_to_provider,
            send_email_to_provider,
            is_flexible,
            message_thread_id,
            start_parameter,
            disable_notification,
            protect_content,
            reply_parameters,
            reply_markup,
            ..
        } = options;

        let params = SendInvoice {
            chat_id: ChatUId::from(chat_id),
            title,
            description,
            payload,
            currency,
            prices: prices.iter().map(|coll| coll.to_owned().into()).collect(),
            provider_token,
            max_tip_amount,
            suggested_tip_amounts,
            provider_data,
            photo_url,
            photo_size,
            photo_width,
            photo_height,
            need_name,
            need_phone_number,
            need_email,
            need_shipping_address,
            send_phone_number_to_provider,
            send_email_to_provider,
            is_flexible,
            message_thread_id,
            start_parameter,
            disable_notification,
            protect_content,
            reply_parameters,
            reply_markup,
        };

        Ok(self.client.send_invoice(&params).await?)
    }
}

#[async_trait::async_trait]
impl Order for BotsApi {
    async fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
        error_message: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = AnswerPreCheckoutQuery {
            ok,
            pre_checkout_query_id,
            error_message,
        };

        Ok(self.client.answer_pre_checkout_query(&params).await?)
    }

    async fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
        shipping_options: Option<Vec<ShippingOption>>,
        error_message: Option<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = AnswerShippingQuery {
            ok,
            shipping_query_id,
            shipping_options: shipping_options
                .map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()),
            error_message,
        };

        Ok(self.client.answer_shipping_query(&params).await?)
    }
}

#[async_trait::async_trait]
impl Star for BotsApi {
    async fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let params = RefundStarPayment {
            user_id,
            telegram_payment_charge_id,
        };

        Ok(self.client.refund_star_payment(&params).await?)
    }
}
