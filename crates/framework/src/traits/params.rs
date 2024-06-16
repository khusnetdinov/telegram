pub trait Params {
    type Delete;
    type Get;
    type Set;

    fn config() -> (Self::Delete, Self::Get, Self::Set);

    fn delete() -> Self::Delete;

    fn get() -> Self::Get;

    fn set() -> Self::Set;
}
