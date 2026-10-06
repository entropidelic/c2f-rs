//! The landscape: a grid of square cells, each with a fuel or without one.
//!
//! Cells are numbered from 0, row by row, starting at the top-left (north-west) corner.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Landscape {
    pub rows: usize,
    pub columns: usize,
    /// Side of a cell, in m.
    pub cell_size: f64,
    /// Map coordinates of the lower-left corner of the grid.
    pub lower_left_x: f64,
    pub lower_left_y: f64,
    /// Fuel code of every cell, or `None` if the cell cannot burn.
    pub fuels: Vec<Option<u16>>,
}

/// A cell next to another one, including diagonals.
pub struct Neighbour {
    pub cell: usize,
    /// Position around the cell, from 0 to 7, counterclockwise starting at east.
    pub position: usize,
    /// Direction towards the neighbour, in degrees (0 = east, counterclockwise).
    pub direction: f64,
    /// Distance between the two cell centres, in m.
    pub distance: f64,
}

/// Row and column offsets of the 8 neighbours, counterclockwise starting at east.
/// Rows grow towards the south.
const NEIGHBOUR_OFFSETS: [(isize, isize); 8] = [
    (0, 1),   // east
    (-1, 1),  // north-east
    (-1, 0),  // north
    (-1, -1), // north-west
    (0, -1),  // west
    (1, -1),  // south-west
    (1, 0),   // south
    (1, 1),   // south-east
];

impl Landscape {
    /// Reads a fuel raster in ESRI ASCII format (`fuels.asc`) and the lookup table that
    /// says which raster values are fuels. Values missing from the table, or named
    /// "Non-fuel" in it, cannot burn.
    pub fn read(fuels_path: &Path, lookup_table_path: &Path) -> Result<Landscape, String> {
        let fuel_names = read_lookup_table(lookup_table_path)?;
        let text = fs::read_to_string(fuels_path)
            .map_err(|error| format!("cannot read {}: {error}", fuels_path.display()))?;

        let mut header = HashMap::new();
        let mut fuels = Vec::new();
        for line in text.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }
            // Header lines are a name followed by a value, like "ncols 7".
            if tokens[0].parse::<f64>().is_err() {
                let value = tokens.get(1).copied().unwrap_or_default();
                header.insert(tokens[0].to_lowercase(), value.to_string());
                continue;
            }
            for token in tokens {
                let value: i32 = token.parse().map_err(|_| {
                    format!("invalid fuel value '{token}' in {}", fuels_path.display())
                })?;
                let is_fuel = fuel_names
                    .get(&value)
                    .is_some_and(|name| !name.starts_with("No"));
                fuels.push(if is_fuel { Some(value as u16) } else { None });
            }
        }

        let rows: usize = header_value(&header, "nrows")?;
        let columns: usize = header_value(&header, "ncols")?;
        let cell_size: f64 = header_value(&header, "cellsize")?;
        if fuels.len() != rows * columns {
            return Err(format!(
                "{} has {} values, expected {rows} rows x {columns} columns",
                fuels_path.display(),
                fuels.len()
            ));
        }

        // Some rasters give the centre of the lower-left cell instead of its corner.
        let (lower_left_x, lower_left_y) = if header.contains_key("xllcenter") {
            let center_x: f64 = header_value(&header, "xllcenter")?;
            let center_y: f64 = header_value(&header, "yllcenter")?;
            (center_x - cell_size / 2.0, center_y - cell_size / 2.0)
        } else {
            (
                header_value(&header, "xllcorner")?,
                header_value(&header, "yllcorner")?,
            )
        };

        Ok(Landscape {
            rows,
            columns,
            cell_size,
            lower_left_x,
            lower_left_y,
            fuels,
        })
    }

    /// The neighbours of a cell that are inside the grid.
    pub fn neighbours(&self, cell: usize) -> Vec<Neighbour> {
        let row = (cell / self.columns) as isize;
        let column = (cell % self.columns) as isize;
        let mut neighbours = Vec::with_capacity(8);

        for (position, (row_offset, column_offset)) in NEIGHBOUR_OFFSETS.into_iter().enumerate() {
            let neighbour_row = row + row_offset;
            let neighbour_column = column + column_offset;
            let inside_grid = neighbour_row >= 0
                && neighbour_column >= 0
                && neighbour_row < self.rows as isize
                && neighbour_column < self.columns as isize;
            if !inside_grid {
                continue;
            }

            let is_diagonal = row_offset != 0 && column_offset != 0;
            let distance = if is_diagonal {
                std::f64::consts::SQRT_2 * self.cell_size
            } else {
                self.cell_size
            };
            neighbours.push(Neighbour {
                cell: neighbour_row as usize * self.columns + neighbour_column as usize,
                position,
                direction: position as f64 * 45.0,
                distance,
            });
        }
        neighbours
    }
}

/// Reads the lookup table: raster value in the first column, fuel name in the fourth.
fn read_lookup_table(path: &Path) -> Result<HashMap<i32, String>, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let mut fuel_names = HashMap::new();
    for line in text.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() < 4 {
            continue;
        }
        let value: i32 = fields[0]
            .trim()
            .parse()
            .map_err(|_| format!("invalid raster value '{}' in {}", fields[0], path.display()))?;
        fuel_names.insert(value, fields[3].trim().to_string());
    }
    Ok(fuel_names)
}

fn header_value<T: std::str::FromStr>(
    header: &HashMap<String, String>,
    key: &str,
) -> Result<T, String> {
    let value = header
        .get(key)
        .ok_or(format!("missing '{key}' in the raster header"))?;
    value
        .parse()
        .map_err(|_| format!("invalid '{key}' in the raster header: {value}"))
}
