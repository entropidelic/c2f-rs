mod colours;
mod viewer;

use c2f_rs::instance::Instance;
use eframe::egui;
use std::path::Path;
use std::process::ExitCode;
use viewer::Viewer;

fn main() -> ExitCode {
    let Some(folder) = std::env::args().nth(1) else {
        eprintln!("usage: c2f-viewer <instance folder>");
        return ExitCode::FAILURE;
    };
    let instance = match Instance::read(Path::new(&folder)) {
        Ok(instance) => instance,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    let result = eframe::run_native(
        &format!("c2f-rs viewer: {folder}"),
        options,
        Box::new(|creation| {
            let viewer = Viewer::new(&creation.egui_ctx, instance)?;
            Ok(Box::new(viewer))
        }),
    );

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
