use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("unable to encode request")]
    EncodeError(#[from] EncodeError),

    #[error("unable to decode response")]
    DecodeError(#[from] DecodeError),

    #[error("{}", .msg)]
    AppError { msg: String },
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid utf8 bytes")]
    Utf8Error(#[from] Utf8Error),

    #[error("invalid protobuf")]
    ProtoDecodeError(#[from] prost::DecodeError),

    #[error("invalid json")]
    JsonDecodeError(#[from] serde_json::Error),

    #[error("invalid base64")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("invalid signing key")]
    SigningKeyDecodeError { msg: String },
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("invalid protobuf")]
    ProtoEncodeError(#[from] prost::EncodeError),

    #[error("unable to encode json")]
    JsonEncodeError(#[from] serde_json::Error),
}
