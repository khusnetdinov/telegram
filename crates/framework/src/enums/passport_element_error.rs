use crate::structs::passports::passport_element_error::passport_element_error_data_field::PassportElementErrorDataField;
use crate::structs::passports::passport_element_error::passport_element_error_file::PassportElementErrorFile;
use crate::structs::passports::passport_element_error::passport_element_error_files::PassportElementErrorFiles;
use crate::structs::passports::passport_element_error::passport_element_error_front_side::PassportElementErrorFrontSide;
use crate::structs::passports::passport_element_error::passport_element_error_reverse_side::PassportElementErrorReverseSide;
use crate::structs::passports::passport_element_error::passport_element_error_selfie::PassportElementErrorSelfie;
use crate::structs::passports::passport_element_error::passport_element_error_translation_file::PassportElementErrorTranslationFile;
use crate::structs::passports::passport_element_error::passport_element_error_translation_files::PassportElementErrorTranslationFiles;
use crate::structs::passports::passport_element_error::passport_element_error_unspecified::PassportElementErrorUnspecified;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::passport_element_error::PassportElementError as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
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
