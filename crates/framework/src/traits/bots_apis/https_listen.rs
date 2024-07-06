#[async_trait::async_trait]
pub trait HttpsListen {
    fn https_listen(&self);
}
