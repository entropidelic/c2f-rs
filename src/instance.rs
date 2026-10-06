//! An instance folder: the inputs of a simulation, in the same layout C2F-W uses.

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
    pub fn read(folder: &Path) -> Result<Instance, String> {
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
fn read_ignition(path: &Path) -> Result<usize, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let first_row = text
        .lines()
        .nth(1)
        .ok_or(format!("{} has no ignition rows", path.display()))?;
    let cell = first_row.split(',').nth(1).unwrap_or_default().trim();
    match cell.parse::<usize>() {
        Ok(number) if number >= 1 => Ok(number - 1),
        _ => Err(format!(
            "invalid ignition cell '{cell}' in {}",
            path.display()
        )),
    }
}
