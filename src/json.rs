//! Loading data from json files.
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors that can occur when reading json data from a file.
pub enum ReadJsonError {
    #[error("{0} io error: {1}")]
    /// IO error
    IoError(PathBuf, #[source] std::io::Error),
    #[error("failed to parse {0}: {1}")]
    /// JSON parsing error
    ParseError(PathBuf, #[source] serde_json::Error),
}

/// Helper trait to report which file caused a problem when reading json
pub trait ReadJsonErrExt<T> {
    /// Attach path to this error
    fn err_path(self, path: impl AsRef<Path>) -> Result<T, ReadJsonError>;
}

impl<T> ReadJsonErrExt<T> for Result<T, std::io::Error> {
    fn err_path(self, path: impl AsRef<Path>) -> Result<T, ReadJsonError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(ReadJsonError::IoError(path.as_ref().to_owned(), e)),
        }
    }
}

impl<T> ReadJsonErrExt<T> for Result<T, serde_json::Error> {
    fn err_path(self, path: impl AsRef<Path>) -> Result<T, ReadJsonError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(ReadJsonError::ParseError(path.as_ref().to_owned(), e)),
        }
    }
}

/// Read a value of type `T` from a json file at `path`
pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, ReadJsonError> {
    let mut f = File::open(&path).err_path(&path)?;
    let mut data = String::with_capacity(4096);
    f.read_to_string(&mut data).err_path(&path)?;

    serde_json::from_str(&data).err_path(&path)
}
