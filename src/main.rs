use c2f_rs::instance::Instance;
use c2f_rs::output::write_results;
use c2f_rs::simulation::{Settings, simulate};
use clap::Parser;
use std::error::Error;
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
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u8).range(1..=4))]
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

fn run(arguments: &Arguments) -> Result<(), Box<dyn Error>> {
    let instance = Instance::read(&arguments.instance)?;

    let cell_count = instance.landscape.rows * instance.landscape.columns;
    let ignition = match arguments.ignition {
        Some(cell) if cell == 0 || cell > cell_count => {
            let message = format!("--ignition must be a cell from 1 to {cell_count}");
            return Err(message.into());
        }
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
