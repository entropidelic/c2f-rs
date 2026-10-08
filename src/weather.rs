//! Weather during the fire, one row per weather period (an hour by default).

use crate::error::{InputError, read_text};
use std::path::Path;

pub struct Weather {
    /// Wind speed, in km/h.
    pub wind_speed: f32,
    /// Compass bearing the wind blows from, in degrees (0 = north, clockwise).
    pub wind_direction: f64,
}

/// Reads a weather file such as `Weather.csv`. Columns are found by name: `WS` for wind
/// speed and `WD` for wind direction.
pub fn read_weather(path: &Path) -> Result<Vec<Weather>, InputError> {
    let text = read_text(path)?;
    let mut lines = text.lines();

    let header: Vec<&str> = lines
        .next()
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .collect();
    let column_of = |name: &str| {
        header
            .iter()
            .position(|column| *column == name)
            .ok_or_else(|| InputError::MissingColumn {
                path: path.to_path_buf(),
                column: name.to_string(),
            })
    };
    let wind_speed_column = column_of("WS")?;
    let wind_direction_column = column_of("WD")?;

    let mut rows = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<&str> = line.split(',').map(str::trim).collect();
        rows.push(Weather {
            wind_speed: parse_field(&fields, wind_speed_column, path)?,
            wind_direction: parse_field(&fields, wind_direction_column, path)?,
        });
    }
    Ok(rows)
}

fn parse_field<T: std::str::FromStr>(
    fields: &[&str],
    column: usize,
    path: &Path,
) -> Result<T, InputError> {
    let value = fields.get(column).copied().unwrap_or_default();
    value.parse().map_err(|_| InputError::InvalidValue {
        path: path.to_path_buf(),
        value: value.to_string(),
    })
}
