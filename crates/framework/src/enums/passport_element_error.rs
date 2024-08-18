use crate::structs::passports::passport_element_error_data_field::PassportElementErrorDataField;
use crate::structs::passports::passport_element_error_file::PassportElementErrorFile;
use crate::structs::passports::passport_element_error_files::PassportElementErrorFiles;
use crate::structs::passports::passport_element_error_front_side::PassportElementErrorFrontSide;
use crate::structs::passports::passport_element_error_reverse_side::PassportElementErrorReverseSide;
use crate::structs::passports::passport_element_error_selfie::PassportElementErrorSelfie;
use crate::structs::passports::passport_element_error_translation_file::PassportElementErrorTranslationFile;
use crate::structs::passports::passport_element_error_translation_files::PassportElementErrorTranslationFiles;
use crate::structs::passports::passport_element_error_unspecified::PassportElementErrorUnspecified;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::passport_element_error::PassportElementError as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PassportElementError {
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}

impl From<Remote> for PassportElementError {
    fn from(remote: Remote) -> Self {
        match remote {
            Remote::DataField(error) => Self::DataField(error.into()),
            Remote::FrontSide(error) => Self::FrontSide(error.into()),
            Remote::ReverseSide(error) => Self::ReverseSide(error.into()),
            Remote::Selfie(error) => Self::Selfie(error.into()),
            Remote::File(error) => Self::File(error.into()),
            Remote::Files(error) => Self::Files(error.into()),
            Remote::TranslationFile(error) => Self::TranslationFile(error.into()),
            Remote::TranslationFiles(error) => Self::TranslationFiles(error.into()),
            Remote::Unspecified(error) => Self::Unspecified(error.into()),
        }
    }
}

impl From<PassportElementError> for Remote {
    fn from(value: PassportElementError) -> Self {
        match value {
            PassportElementError::DataField(error) => Self::DataField(error.into()),
            PassportElementError::FrontSide(error) => Self::FrontSide(error.into()),
            PassportElementError::ReverseSide(error) => Self::ReverseSide(error.into()),
            PassportElementError::Selfie(error) => Self::Selfie(error.into()),
            PassportElementError::File(error) => Self::File(error.into()),
            PassportElementError::Files(error) => Self::Files(error.into()),
            PassportElementError::TranslationFile(error) => Self::TranslationFile(error.into()),
            PassportElementError::TranslationFiles(error) => Self::TranslationFiles(error.into()),
            PassportElementError::Unspecified(error) => Self::Unspecified(error.into()),
        }
    }
}
