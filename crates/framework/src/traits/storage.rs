use futures::future::BoxFuture;
use std::sync::Arc;

pub trait Storage<State> {
    type Error;

    fn get(self: Arc<Self>, chat_id: i64)
        -> BoxFuture<'static, Result<Option<State>, Self::Error>>;

    fn set(
        self: Arc<Self>,
        chat_id: i64,
        state: State,
    ) -> BoxFuture<'static, Result<(), Self::Error>>;
}
