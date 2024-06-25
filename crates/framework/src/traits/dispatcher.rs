pub trait KindDispatcher {
    type Kind;

    fn dispatch(&self) -> &Self::Kind;
}
