use thiserror::Error;
use uefi::Status;

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum UefiDisplayError {
    #[error("Unsupported Color Format")]
    UnsupportedFormat,
    #[error("Invalid Resolution")]
    InvalidResolution,
    #[error("Out of Bounds")]
    OutOfBounds,
}

impl From<UefiDisplayError> for uefi::Error {
    fn from(value: UefiDisplayError) -> Self {
        match value {
            UefiDisplayError::UnsupportedFormat => uefi::Error::new(Status::UNSUPPORTED, ()),
            UefiDisplayError::InvalidResolution => uefi::Error::new(Status::INVALID_PARAMETER, ()),
            UefiDisplayError::OutOfBounds => uefi::Error::new(Status::BUFFER_TOO_SMALL, ()),
        }
    }
}
