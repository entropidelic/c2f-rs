//! The errors the library can return.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;

/// A problem reading the input files.
#[derive(Debug, Error)]
pub enum InputError {
    #[error("cannot read {}: {source}", .path.display())]
    CannotRead { path: PathBuf, source: io::Error },

    #[error("invalid value '{value}' in {}", .path.display())]
    InvalidValue { path: PathBuf, value: String },

    #[error("missing '{key}' in the header of {}", .path.display())]
    MissingHeader { path: PathBuf, key: String },

    #[error("missing column '{column}' in {}", .path.display())]
    MissingColumn { path: PathBuf, column: String },

    #[error("{} has no rows", .path.display())]
    NoRows { path: PathBuf },

    #[error("{} has {values} values, expected {rows} rows x {columns} columns", .path.display())]
    WrongSize {
        path: PathBuf,
        values: usize,
        rows: usize,
        columns: usize,
    },
}

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

/// A problem starting or running a simulation.
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("no Scott & Burgan fuel model {code} for moisture scenario {scenario}")]
    UnknownFuel { code: u16, scenario: u8 },

    #[error("cell {cell} is outside the landscape, which has {cell_count} cells")]
    CellOutsideLandscape { cell: usize, cell_count: usize },

    #[error("the weather has no rows")]
    NoWeather,
}

/// A problem writing the results.
#[derive(Debug, Error)]
#[error("cannot write {}: {source}", .path.display())]
pub struct OutputError {
    pub path: PathBuf,
    pub source: io::Error,
}

/// Creates a folder, and the folders above it if needed.
pub(crate) fn create_folder(path: &Path) -> Result<(), OutputError> {
    fs::create_dir_all(path).map_err(|source| OutputError {
        path: path.to_path_buf(),
        source,
    })
}

/// Writes a whole text file.
pub(crate) fn write_text(path: &Path, contents: &str) -> Result<(), OutputError> {
    fs::write(path, contents).map_err(|source| OutputError {
        path: path.to_path_buf(),
        source,
    })
}
