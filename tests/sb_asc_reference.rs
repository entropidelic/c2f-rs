//! Compares the Scott & Burgan fire behaviour against C2F-W outputs for the
//! 7x7 `sb-asc` instance: moisture scenario 1, wind of 10 km/h from the south
//! during the whole fire, and no elevation data.

use c2f_rs::ellipse::{FireEllipse, heading_from_wind};
use c2f_rs::scott_burgan::{coefficients, surface_fire};
use std::fs;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/sb-asc");
const SCENARIO: u8 = 1;
const WIND_SPEED: f32 = 10.0;
const WIND_FROM: f64 = 180.0;
const COLUMNS: usize = 7;
const SIMULATIONS: usize = 113;

/// Fuel code of every cell, numbered row by row from the top-left corner.
fn read_fuel_codes() -> Vec<u16> {
    let text = fs::read_to_string(format!("{DATA}/input/fuels.asc")).unwrap();
    let header_lines = 6;
    text.lines()
        .skip(header_lines)
        .flat_map(|line| line.split_whitespace())
        .map(|value| value.parse().unwrap())
        .collect()
}

/// Direction from one cell to another, in degrees (0 = east, counterclockwise).
/// Cell ids start at 1, as in C2F-W.
fn direction_between(from_cell: usize, to_cell: usize) -> f64 {
    let from_row = (from_cell - 1) / COLUMNS;
    let from_column = (from_cell - 1) % COLUMNS;
    let to_row = (to_cell - 1) / COLUMNS;
    let to_column = (to_cell - 1) % COLUMNS;

    let east = to_column as f64 - from_column as f64;
    let north = from_row as f64 - to_row as f64;
    north.atan2(east).to_degrees().rem_euclid(360.0)
}

/// Each message line is `source cell, target cell, period, rate of spread`,
/// written when the fire from the source reaches the centre of the target.
/// The rate is the source's spread rate towards the target, rounded up to
/// two decimals.
#[test]
fn spread_rate_towards_each_neighbour_matches_every_message() {
    let fuel_codes = read_fuel_codes();
    let heading = heading_from_wind(WIND_FROM);
    let mut checked = 0;
    let mut mismatches = Vec::new();

    for simulation in 1..=SIMULATIONS {
        let path = format!("{DATA}/expected/Messages/MessagesFile{simulation:03}.csv");
        for line in fs::read_to_string(path).unwrap().lines() {
            let fields: Vec<&str> = line.split(',').collect();
            let source: usize = fields[0].parse().unwrap();
            let target: usize = fields[1].parse().unwrap();
            let expected_rate: f64 = fields[3].parse().unwrap();

            let fuel = coefficients(SCENARIO, fuel_codes[source - 1]).unwrap();
            let fire = surface_fire(fuel, WIND_SPEED);
            let ellipse = FireEllipse::new(
                fire.head_ros,
                fire.back_ros,
                fire.length_to_breadth,
                heading,
            );
            let rate = ellipse.spread_rate_towards(direction_between(source, target)) as f32;
            let rounded_rate = (f64::from(rate) * 100.0).ceil() / 100.0;

            if (rounded_rate - expected_rate).abs() > 1e-4 {
                mismatches.push(format!(
                    "sim {simulation}: {line} -> computed {rounded_rate}"
                ));
            }
            checked += 1;
        }
    }

    assert!(checked > 0);
    assert!(
        mismatches.is_empty(),
        "{} of {checked} messages differ:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

/// The intensity grid holds, for every burnt cell, the intensity of its own
/// fuel under the (constant) weather. Unburnt cells are 0.
#[test]
fn intensity_matches_every_burnt_cell() {
    let fuel_codes = read_fuel_codes();
    let mut checked = 0;
    let mut mismatches = Vec::new();

    for simulation in 1..=SIMULATIONS {
        let path = format!("{DATA}/expected/SurfaceIntensity/SurfaceIntensity{simulation:03}.asc");
        let text = fs::read_to_string(path).unwrap();
        let values = text
            .lines()
            .skip(6)
            .flat_map(|line| line.split_whitespace());

        for (cell_index, value) in values.enumerate() {
            let expected: f32 = value.parse().unwrap();
            if expected == 0.0 {
                continue;
            }
            let fuel = coefficients(SCENARIO, fuel_codes[cell_index]).unwrap();
            let computed = surface_fire(fuel, WIND_SPEED).intensity;

            // C2F-W prints six significant digits.
            if (computed - expected).abs() > expected * 1e-5 {
                mismatches.push(format!(
                    "sim {simulation}, cell {}: expected {expected}, computed {computed}",
                    cell_index + 1
                ));
            }
            checked += 1;
        }
    }

    assert!(checked > 0);
    assert!(
        mismatches.is_empty(),
        "{} of {checked} cells differ:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}
