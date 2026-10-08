//! An instance folder: the inputs of a simulation, in the same layout C2F-W uses.

use crate::error::{InputError, read_text};
use crate::landscape::Landscape;
use crate::weather::{Weather, read_weather};
use std::fs;
use std::path::Path;

pub struct Instance {
    pub landscape: Landscape,
    pub weather: Vec<Weather>,
    /// Ignition cell from `Ignitions.csv`, numbered from 0, if the file exists.
    pub ignition: Option<usize>,
    /// Coordinate system of the landscape (the contents of `fuels.prj`), if given.
    pub projection: Option<String>,
}

impl Instance {
    /// Reads `fuels.asc`, `spain_lookup_table.csv` and `Weather.csv`, and also
    /// `Ignitions.csv` and `fuels.prj` if they exist.
    pub fn read(folder: &Path) -> Result<Instance, InputError> {
        let landscape = Landscape::read(
            &folder.join("fuels.asc"),
            &folder.join("spain_lookup_table.csv"),
        )?;
        let weather = read_weather(&folder.join("Weather.csv"))?;

        let ignitions_path = folder.join("Ignitions.csv");
        let ignition = if ignitions_path.exists() {
            Some(read_ignition(&ignitions_path)?)
        } else {
            None
        };

        // A missing projection is fine: the outputs will just have none either.
        let projection = fs::read_to_string(folder.join("fuels.prj")).ok();

        Ok(Instance {
            landscape,
            weather,
            ignition,
            projection,
        })
    }
}

/// Reads the ignition cell of the first row of `Ignitions.csv`, whose columns are a year
/// and a cell numbered from 1.
fn read_ignition(path: &Path) -> Result<usize, InputError> {
    let text = read_text(path)?;
    let first_row = text.lines().nth(1).ok_or_else(|| InputError::NoRows {
        path: path.to_path_buf(),
    })?;
    let cell = first_row.split(',').nth(1).unwrap_or_default().trim();
    match cell.parse::<usize>() {
        Ok(number) if number >= 1 => Ok(number - 1),
        _ => Err(InputError::InvalidValue {
            path: path.to_path_buf(),
            value: cell.to_string(),
        }),
    }
}
