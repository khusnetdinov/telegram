use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
