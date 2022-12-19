use std::{str::Utf8Error, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsarError {
    #[error("early end of file encountered")]
    Eof,

    #[cfg(feature = "write")]
    #[error("unable to index into zip, possibly corrupt")]
    ZipIndex(#[from] zip::result::ZipError),

    #[cfg(feature = "write")]
    #[error("unable to inflate zip entry: {msg}")]
    ZipBadData { msg: String },

    #[error("unable to serialize header")]
    HeaderSerialization(#[from] serde_json::Error),

    #[error("compression failure")]
    Compression(#[source] io::Error),

    #[error("unable to convert header of file to utf8")]
    HeaderEncoding(#[from] Utf8Error),

    #[error("expected header line")]
    HeaderLine,

    #[error("invalid asar header encountered")]
    AsarHeader,
}
