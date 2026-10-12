//! Writes spread messages and georeferenced ESRI ASCII maps.
//!
//! Cells are numbered from 1 in the output files, as in C2F-W instances.

use crate::error::OutputError;
use crate::landscape::Landscape;
use crate::simulation::Fire;
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// Value of the map cells that did not burn.
const NO_DATA: i32 = -9999;

/// Writes into `folder`:
///
/// - `messages.csv`: spread events, with the target cell's ignition time and rate of spread.
/// - `arrival_time.asc`: ignition time in minutes for each burnt cell.
/// - `intensity.asc` and `flame_length.asc`: intensity and flame length at ignition.
///
/// If `projection` is given, a `.prj` file is written next to each map.
pub fn write_results(
    folder: &Path,
    landscape: &Landscape,
    fire: &Fire,
    projection: Option<&str>,
) -> Result<(), OutputError> {
    create_folder(folder)?;

    let mut messages = String::from("from,to,time_minutes,rate_of_spread\n");
    for message in &fire.messages {
        let from = message.from + 1;
        let to = message.to + 1;
        writeln!(messages, "{from},{to},{},{}", message.time, message.rate).unwrap();
    }
    write_text(&folder.join("messages.csv"), &messages)?;

    let cell_count = landscape.rows * landscape.columns;
    let mut arrival_time = vec![None; cell_count];
    let mut intensity = vec![None; cell_count];
    let mut flame_length = vec![None; cell_count];
    for burnt in &fire.burnt_cells {
        arrival_time[burnt.cell] = Some(burnt.time);
        intensity[burnt.cell] = Some(burnt.intensity);
        flame_length[burnt.cell] = Some(burnt.flame_length);
    }

    for (name, values) in [
        ("arrival_time", arrival_time),
        ("intensity", intensity),
        ("flame_length", flame_length),
    ] {
        write_text(
            &folder.join(format!("{name}.asc")),
            &ascii_grid(landscape, &values),
        )?;
        if let Some(projection) = projection {
            write_text(&folder.join(format!("{name}.prj")), projection)?;
        }
    }
    Ok(())
}

/// A map in ESRI ASCII format, with the same position and cell size as the landscape.
fn ascii_grid(landscape: &Landscape, values: &[Option<f32>]) -> String {
    let mut text = String::new();
    writeln!(text, "ncols {}", landscape.columns).unwrap();
    writeln!(text, "nrows {}", landscape.rows).unwrap();
    writeln!(text, "xllcorner {}", landscape.lower_left_x).unwrap();
    writeln!(text, "yllcorner {}", landscape.lower_left_y).unwrap();
    writeln!(text, "cellsize {}", landscape.cell_size).unwrap();
    writeln!(text, "NODATA_value {NO_DATA}").unwrap();

    for row in values.chunks(landscape.columns) {
        let row_text: Vec<String> = row
            .iter()
            .map(|value| match value {
                Some(value) => value.to_string(),
                None => NO_DATA.to_string(),
            })
            .collect();
        writeln!(text, "{}", row_text.join(" ")).unwrap();
    }
    text
}

/// Creates a folder and any missing parent folders.
fn create_folder(path: &Path) -> Result<(), OutputError> {
    fs::create_dir_all(path).map_err(|source| OutputError {
        path: path.to_path_buf(),
        source,
    })
}

/// Writes a whole text file.
fn write_text(path: &Path, contents: &str) -> Result<(), OutputError> {
    fs::write(path, contents).map_err(|source| OutputError {
        path: path.to_path_buf(),
        source,
    })
}
