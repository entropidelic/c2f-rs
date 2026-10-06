//! Compares c2f-rs against C2F-W outputs for the 7x7 `sb-asc` instance: Scott & Burgan
//! fuels, moisture scenario 1, wind of 10 km/h from the south for 5 hours, no elevation.
//! C2F-W ran 113 fires there, each one from a random ignition cell that it logged.

use c2f_rs::landscape::Landscape;
use c2f_rs::scott_burgan::{coefficients, surface_fire};
use c2f_rs::simulation::{Settings, simulate};
use c2f_rs::weather::{Weather, read_weather};
use std::fs;
use std::path::Path;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/sb-asc");
const SCENARIO: u8 = 1;
const SIMULATIONS: usize = 113;

/// A message as (from, to, arrival minute, rate), with cells numbered from 0 and the
/// rate in hundredths of m/min. C2F-W writes the rate rounded up to two decimals.
type MessageKey = (usize, usize, u32, i64);

fn read_landscape() -> Landscape {
    let fuels = format!("{DATA}/input/fuels.asc");
    let lookup_table = format!("{DATA}/input/spain_lookup_table.csv");
    Landscape::read(Path::new(&fuels), Path::new(&lookup_table)).unwrap()
}

fn read_input_weather() -> Vec<Weather> {
    read_weather(Path::new(&format!("{DATA}/input/Weather.csv"))).unwrap()
}

/// Ignition cell of every simulation, numbered from 0.
fn read_ignition_cells() -> Vec<usize> {
    let text = fs::read_to_string(format!("{DATA}/expected/ignition_and_weather_log.csv")).unwrap();
    text.lines()
        .skip(1)
        .map(|line| {
            let cell: usize = line.split(',').nth(1).unwrap().parse().unwrap();
            cell - 1
        })
        .collect()
}

/// Number of burnt cells of every simulation, from the results tables in C2F-W's log.
fn read_burnt_counts() -> Vec<usize> {
    let text = fs::read_to_string(format!("{DATA}/expected/log.txt")).unwrap();
    text.lines()
        .filter(|line| line.trim_start().starts_with("Burnt"))
        .map(|line| line.split_whitespace().nth(1).unwrap().parse().unwrap())
        .collect()
}

/// Messages of a simulation, sorted. C2F-W writes them in hash map order.
///
/// C2F-W writes fire period numbers instead of times. Its period 0 already spreads the
/// fire for one minute, so a fire that arrives after 4 minutes is written as period 3.
fn read_expected_messages(simulation: usize) -> Vec<MessageKey> {
    let path = format!("{DATA}/expected/Messages/MessagesFile{simulation:03}.csv");
    let mut messages: Vec<MessageKey> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let from: usize = fields[0].parse().unwrap();
            let to: usize = fields[1].parse().unwrap();
            let period: u32 = fields[2].parse().unwrap();
            let rate: f64 = fields[3].parse().unwrap();
            let arrival_minute = period + 1;
            (
                from - 1,
                to - 1,
                arrival_minute,
                (rate * 100.0).round() as i64,
            )
        })
        .collect();
    messages.sort();
    messages
}

fn describe_difference(computed: &[MessageKey], expected: &[MessageKey]) -> String {
    let missing: Vec<_> = expected
        .iter()
        .filter(|message| !computed.contains(message))
        .take(5)
        .collect();
    let unexpected: Vec<_> = computed
        .iter()
        .filter(|message| !expected.contains(message))
        .take(5)
        .collect();
    format!(
        "  {} messages, expected {}\n  missing (first 5): {missing:?}\n  unexpected (first 5): {unexpected:?}",
        computed.len(),
        expected.len()
    )
}

#[test]
fn every_fire_burns_the_same_cells_and_sends_the_same_messages() {
    let landscape = read_landscape();
    let weather = read_input_weather();
    let settings = Settings {
        moisture_scenario: SCENARIO,
        ..Settings::default()
    };
    let ignition_cells = read_ignition_cells();
    let burnt_counts = read_burnt_counts();
    assert_eq!(ignition_cells.len(), SIMULATIONS);
    assert_eq!(burnt_counts.len(), SIMULATIONS);

    let mut failures = Vec::new();
    for (index, &ignition) in ignition_cells.iter().enumerate() {
        let simulation = index + 1;
        let fire = simulate(&landscape, &weather, ignition, &settings).unwrap();

        if fire.burnt_cells.len() != burnt_counts[index] {
            failures.push(format!(
                "sim {simulation}: {} burnt cells, expected {}",
                fire.burnt_cells.len(),
                burnt_counts[index]
            ));
        }

        let mut messages: Vec<MessageKey> = fire
            .messages
            .iter()
            .map(|message| {
                let arrival_minute = message.time.round() as u32;
                let rate_in_hundredths = (f64::from(message.rate) * 100.0).ceil() as i64;
                (message.from, message.to, arrival_minute, rate_in_hundredths)
            })
            .collect();
        messages.sort();
        let expected_messages = read_expected_messages(simulation);
        if messages != expected_messages {
            failures.push(format!(
                "sim {simulation}: messages differ\n{}",
                describe_difference(&messages, &expected_messages)
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} problems:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The intensity grid holds, for every burnt cell, the intensity of its own fuel under
/// the weather, which does not change in this instance. Unburnt cells are 0.
#[test]
fn intensity_matches_every_burnt_cell() {
    let landscape = read_landscape();
    let wind_speed = read_input_weather()[0].wind_speed;
    let mut checked = 0;
    let mut mismatches = Vec::new();

    for simulation in 1..=SIMULATIONS {
        let path = format!("{DATA}/expected/SurfaceIntensity/SurfaceIntensity{simulation:03}.asc");
        let text = fs::read_to_string(path).unwrap();
        let values = text
            .lines()
            .skip(6)
            .flat_map(|line| line.split_whitespace());

        for (cell, value) in values.enumerate() {
            let expected: f32 = value.parse().unwrap();
            if expected == 0.0 {
                continue;
            }
            let fuel = coefficients(SCENARIO, landscape.fuels[cell].unwrap()).unwrap();
            let computed = surface_fire(fuel, wind_speed).intensity;

            // C2F-W rounds intensity up to two decimals, then prints six significant digits.
            let tolerance = 0.01 + expected * 1e-5;
            if (computed - expected).abs() > tolerance {
                mismatches.push(format!(
                    "sim {simulation}, cell {cell}: expected {expected}, computed {computed}"
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
