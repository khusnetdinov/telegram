use crate::enums::update_kind::UpdateKind;
use crate::traits::kind_dispatcher::KindDispatcher;
use telegram_bots_api::api::structs::update::Update as Remote;

#[derive(Debug)]
pub struct Update {
    pub update_id: i64,
    pub kind: UpdateKind,
}

impl From<Remote> for Update {
    fn from(remote: Remote) -> Self {
        Self {
            update_id: remote.update_id,
            kind: UpdateKind::from(remote),
        }
    }
}

impl KindDispatcher for Update {
    type Kind = UpdateKind;

    fn dispatch(&self) -> &Self::Kind {
        &self.kind
    }
}
