#[async_trait::async_trait]
pub trait HttpListen {
    fn http_listen(&self);
}
