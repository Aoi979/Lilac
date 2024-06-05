mod core;

#[derive(Debug, PartialEq)]
pub enum Error {
    KeyNotFound,
    DecodeFailed(String),
    TypeMissMatch,
    DataInvalid,
    InstanceClosed,
    EncodeFailed(String),
    IOError(String),
    LockError(String),
    #[cfg(feature = "encryption")]
    DecryptFailed(String),
    #[cfg(feature = "encryption")]
    EncryptFailed(String),
}