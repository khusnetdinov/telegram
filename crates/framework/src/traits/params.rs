pub trait EnumParams {
    type Delete;
    type Get;
    type Set;

    fn config() -> (Self::Delete, Self::Get, Self::Set);

    fn delete_params() -> Self::Delete;

    fn get_params() -> Self::Get;

    fn set_params() -> Self::Set;
}

pub trait Params {
    type Delete;
    type Get;
    type Set;

    fn delete_params(&self) -> Self::Delete;

    fn get_params(&self) -> Self::Get;

    fn set_params(&self) -> Self::Set;
}
