//! Weather during the fire, one row per weather period (an hour by default).

use std::fs;
use std::path::Path;

pub struct Weather {
    /// Wind speed, in km/h.
    pub wind_speed: f32,
    /// Compass bearing the wind blows from, in degrees (0 = north, clockwise).
    pub wind_direction: f64,
}

/// Reads a weather file such as `Weather.csv`. Columns are found by name: `WS` for wind
/// speed and `WD` for wind direction.
pub fn read_weather(path: &Path) -> Result<Vec<Weather>, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
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
            .ok_or(format!("missing column '{name}' in {}", path.display()))
    };
    let wind_speed_column = column_of("WS")?;
    let wind_direction_column = column_of("WD")?;

    let mut rows = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<&str> = line.split(',').map(str::trim).collect();
        let wind_speed: f32 = parse_field(&fields, wind_speed_column, path)?;
        let wind_direction: f64 = parse_field(&fields, wind_direction_column, path)?;
        rows.push(Weather {
            wind_speed,
            // C2F-W keeps only whole degrees.
            wind_direction: wind_direction.trunc(),
        });
    }
    Ok(rows)
}

fn parse_field<T: std::str::FromStr>(
    fields: &[&str],
    column: usize,
    path: &Path,
) -> Result<T, String> {
    let value = fields.get(column).copied().unwrap_or_default();
    value
        .parse()
        .map_err(|_| format!("invalid value '{value}' in {}", path.display()))
}
