use crate::enums::updates::Updates;
use crate::traits::kind_dispatcher::KindDispatcher;
use telegram_bots_api::api::structs::update::Update as Remote;

#[derive(Debug)]
pub struct Update {
    pub update_id: i64,
    pub kind: Updates,
}

impl From<Remote> for Update {
    fn from(remote: Remote) -> Self {
        Self {
            update_id: remote.update_id,
            kind: Updates::from(remote),
        }
    }
}

impl KindDispatcher for Update {
    type Kind = Updates;

    fn dispatch(&self) -> &Self::Kind {
        &self.kind
    }
}
