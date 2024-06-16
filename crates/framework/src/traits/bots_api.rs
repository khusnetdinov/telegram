use crate::bots_api::BotsApi;
use crate::structs::update::Update;

pub trait Pooler {
    fn pooling(&self, drop_pending_updates: bool, callback: impl Fn(&BotsApi, Update));
}

pub trait HttpListener {
    fn http_listen(&self);
}

pub trait HttpsListener {
    fn https_listen(&self);
}
