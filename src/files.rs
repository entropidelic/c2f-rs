//! Reading text files, with errors that say which file and which value.

use crate::error::InputError;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Reads a whole text file.
pub(crate) fn read_text(path: &Path) -> Result<String, InputError> {
    fs::read_to_string(path).map_err(|source| InputError::CannotRead {
        path: path.to_path_buf(),
        source,
    })
}

/// Parses a value read from the file at `path`, such as a number.
pub(crate) fn parse_value<T: FromStr>(value: &str, path: &Path) -> Result<T, InputError> {
    value.parse().map_err(|_| InputError::InvalidValue {
        path: path.to_path_buf(),
        value: value.to_string(),
    })
}
