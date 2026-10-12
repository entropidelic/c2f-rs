//! Compares CLI output with 113 C2F-W reference fires on the 7×7 `sb-asc` grid:
//! Scott & Burgan fuels, moisture scenario 1, a 10 km/h southerly wind for 5 hours,
//! and no elevation. Each run uses the ignition cell logged by C2F-W.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/sb-asc");
const SIMULATIONS: usize = 113;
const NO_DATA: f32 = -9999.0;

/// A message as (from, to, minute, rate), with the rate in hundredths of m/min.
/// C2F-W writes the rate rounded up to two decimals.
type MessageKey = (usize, usize, u32, i64);

/// Ignition cell for each reference simulation.
fn read_ignition_cells() -> Vec<usize> {
    let text = fs::read_to_string(format!("{DATA}/expected/ignition_and_weather_log.csv")).unwrap();
    text.lines()
        .skip(1)
        .map(|line| line.split(',').nth(1).unwrap().parse().unwrap())
        .collect()
}

/// Burnt cell counts from the results tables in C2F-W's log.
fn read_burnt_counts() -> Vec<usize> {
    let text = fs::read_to_string(format!("{DATA}/expected/log.txt")).unwrap();
    text.lines()
        .filter(|line| line.trim_start().starts_with("Burnt"))
        .map(|line| line.split_whitespace().nth(1).unwrap().parse().unwrap())
        .collect()
}

/// Sorted reference messages; C2F-W writes them in hash-map order.
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
            let period: u32 = fields[2].parse().unwrap();
            let rate: f64 = fields[3].parse().unwrap();
            (
                fields[0].parse().unwrap(),
                fields[1].parse().unwrap(),
                period + 1,
                (rate * 100.0).round() as i64,
            )
        })
        .collect();
    messages.sort();
    messages
}

/// Sorted c2f-rs messages, with rates rounded up as in C2F-W.
fn read_computed_messages(path: &Path) -> Vec<MessageKey> {
    let mut messages: Vec<MessageKey> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            let fields: Vec<&str> = line.split(',').collect();
            let minute: f32 = fields[2].parse().unwrap();
            let rate: f64 = fields[3].parse().unwrap();
            (
                fields[0].parse().unwrap(),
                fields[1].parse().unwrap(),
                minute.round() as u32,
                (rate * 100.0).ceil() as i64,
            )
        })
        .collect();
    messages.sort();
    messages
}

/// Values of a map in ESRI ASCII format, row by row.
fn read_map(path: &Path) -> Vec<f32> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|line| !line.starts_with(char::is_alphabetic))
        .flat_map(|line| line.split_whitespace())
        .map(|value| value.parse().unwrap())
        .collect()
}

fn run_c2f_rs(ignition: usize, output: &Path) {
    let status = Command::new(env!("CARGO_BIN_EXE_c2f-rs"))
        .arg(format!("{DATA}/input"))
        .args(["--output", output.to_str().unwrap()])
        .args(["--ignition", &ignition.to_string()])
        .args(["--scenario", "1"])
        .output()
        .unwrap();
    assert!(
        status.status.success(),
        "{}",
        String::from_utf8_lossy(&status.stderr)
    );
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

/// Compares the intensity map with C2F-W's, where unburnt cells are 0 instead of no data,
/// and checks that the flame length map matches the intensity map.
fn compare_maps(simulation: usize, output: &Path, failures: &mut Vec<String>) {
    let expected_path =
        format!("{DATA}/expected/SurfaceIntensity/SurfaceIntensity{simulation:03}.asc");
    let expected_intensity = read_map(Path::new(&expected_path));
    let intensity = read_map(&output.join("intensity.asc"));
    let flame_length = read_map(&output.join("flame_length.asc"));

    for cell in 0..expected_intensity.len() {
        let expected = expected_intensity[cell];
        let computed = intensity[cell];
        let burnt = computed != NO_DATA;
        // C2F-W rounds intensity up to two decimals, then prints six significant digits.
        let tolerance = 0.01 + expected * 1e-5;
        if burnt != (expected != 0.0) || (burnt && (computed - expected).abs() > tolerance) {
            failures.push(format!(
                "sim {simulation}, cell {}: intensity {computed}, expected {expected}",
                cell + 1
            ));
        }

        // Intensity comes from the flame length, through Byram's relation.
        let intensity_from_flame_length = 259.833 * flame_length[cell].powf(2.174);
        if burnt && (intensity_from_flame_length - computed).abs() > computed * 1e-4 {
            failures.push(format!(
                "sim {simulation}, cell {}: flame length {} does not match intensity {computed}",
                cell + 1,
                flame_length[cell]
            ));
        }
    }
}

#[test]
fn every_fire_matches_c2f_w_outputs() {
    let ignition_cells = read_ignition_cells();
    let burnt_counts = read_burnt_counts();
    assert_eq!(ignition_cells.len(), SIMULATIONS);
    assert_eq!(burnt_counts.len(), SIMULATIONS);

    let mut failures = Vec::new();
    for (index, &ignition) in ignition_cells.iter().enumerate() {
        let simulation = index + 1;
        let output =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("sb-asc-{simulation:03}"));
        run_c2f_rs(ignition, &output);

        let arrival_time = read_map(&output.join("arrival_time.asc"));
        let burnt_count = arrival_time
            .iter()
            .filter(|&&minute| minute != NO_DATA)
            .count();
        if burnt_count != burnt_counts[index] {
            failures.push(format!(
                "sim {simulation}: {burnt_count} burnt cells, expected {}",
                burnt_counts[index]
            ));
        }

        let messages = read_computed_messages(&output.join("messages.csv"));
        let expected_messages = read_expected_messages(simulation);
        if messages != expected_messages {
            failures.push(format!(
                "sim {simulation}: messages differ\n{}",
                describe_difference(&messages, &expected_messages)
            ));
        }

        compare_maps(simulation, &output, &mut failures);
    }

    assert!(
        failures.is_empty(),
        "{} problems:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
