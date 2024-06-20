use futures::future::BoxFuture;
use std::sync::Arc;

pub trait Storage<ST> {
    type Error;

    fn get(self: Arc<Self>, chat_id: i64) -> BoxFuture<'static, Result<Option<ST>, Self::Error>>;

    fn set(self: Arc<Self>, chat_id: i64, state: ST)
        -> BoxFuture<'static, Result<(), Self::Error>>;
}
