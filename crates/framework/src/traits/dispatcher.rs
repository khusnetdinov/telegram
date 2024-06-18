pub trait Dispatcher {
    type Kind;

    fn dispatch(&self) -> &Self::Kind;
}
