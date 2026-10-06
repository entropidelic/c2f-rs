use c2f_rs::instance::Instance;
use c2f_rs::output::write_results;
use c2f_rs::simulation::{Settings, simulate};
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

/// Simulates a forest fire on a landscape, with the Scott & Burgan fuel model.
#[derive(Parser)]
#[command(version)]
struct Arguments {
    /// Instance folder, as used by C2F-W: fuels.asc, spain_lookup_table.csv,
    /// Weather.csv, and optionally Ignitions.csv and fuels.prj
    instance: PathBuf,

    /// Folder where the results are written
    #[arg(long)]
    output: PathBuf,

    /// Cell where the fire starts, numbered from 1, row by row from the top-left corner.
    /// Defaults to the cell in Ignitions.csv
    #[arg(long)]
    ignition: Option<usize>,

    /// Fuel moisture scenario, from 1 (driest) to 4 (wettest)
    #[arg(long, default_value_t = 3)]
    scenario: u8,
}

fn main() -> ExitCode {
    let arguments = Arguments::parse();
    match run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: &Arguments) -> Result<(), String> {
    let instance = Instance::read(&arguments.instance)?;

    let ignition = match arguments.ignition {
        Some(0) => return Err("cells are numbered from 1".to_string()),
        Some(cell) => cell - 1,
        None => instance
            .ignition
            .ok_or("no --ignition given, and the instance has no Ignitions.csv")?,
    };
    let settings = Settings {
        moisture_scenario: arguments.scenario,
        ..Settings::default()
    };

    let fire = simulate(&instance.landscape, &instance.weather, ignition, &settings)?;
    write_results(
        &arguments.output,
        &instance.landscape,
        &fire,
        instance.projection.as_deref(),
    )?;

    let last_time = fire
        .burnt_cells
        .iter()
        .map(|burnt| burnt.time)
        .fold(0.0, f32::max);
    println!(
        "{} cells burnt; the last one caught fire after {last_time} minutes.",
        fire.burnt_cells.len()
    );
    println!("Results written to {}", arguments.output.display());
    Ok(())
}
