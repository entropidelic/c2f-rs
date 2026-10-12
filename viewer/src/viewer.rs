//! The viewer window: a side panel with the controls, and the map of the fire.

use crate::colours::{fire_colour, fuel_colour};
use c2f_rs::error::SimulationError;
use c2f_rs::instance::Instance;
use c2f_rs::landscape::Landscape;
use c2f_rs::simulation::{Settings, Simulation};
use c2f_rs::weather::Weather;
use eframe::egui::{
    self, Align2, Color32, ColorImage, FontId, Pos2, Rect, Sense, Stroke, TextureHandle,
    TextureOptions, Vec2, pos2, vec2,
};

pub struct Viewer {
    landscape: Landscape,
    /// The ignition cell of the instance's `Ignitions.csv`, lit at every reset.
    initial_ignition: Option<usize>,
    settings: Settings,
    simulation: Simulation,
    wind: Weather,
    playing: bool,
    minutes_per_frame: u32,
    /// The viewer pauses when the fire has run this long.
    stop_after_hours: u32,
    /// The colour of every cell before any fire: its fuel.
    fuel_colours: Vec<Color32>,
    map_texture: TextureHandle,
    /// 1 shows the whole landscape; larger values zoom in.
    zoom: f32,
    /// How far the map has been dragged from the centre, in screen points.
    pan: Vec2,
    error: Option<String>,
}

impl Viewer {
    pub fn new(context: &egui::Context, instance: Instance) -> Result<Viewer, SimulationError> {
        context.set_theme(egui::Theme::Dark);
        let landscape = instance.landscape;
        let settings = Settings::default();
        let wind = match instance.weather.first() {
            Some(first_row) => Weather {
                wind_speed: first_row.wind_speed,
                wind_direction: first_row.wind_direction,
            },
            None => Weather {
                wind_speed: 0.0,
                wind_direction: 0.0,
            },
        };

        let fuel_colours: Vec<Color32> = landscape
            .fuels
            .iter()
            .map(|fuel| fuel_colour(*fuel))
            .collect();
        let first_image =
            ColorImage::new([landscape.columns, landscape.rows], fuel_colours.clone());
        let map_texture = context.load_texture("map", first_image, TextureOptions::NEAREST);

        let mut simulation = Simulation::new(landscape.clone(), &settings)?;
        if let Some(cell) = instance.ignition {
            simulation.ignite(cell, &wind)?;
        }

        Ok(Viewer {
            landscape,
            initial_ignition: instance.ignition,
            settings,
            simulation,
            wind,
            playing: false,
            minutes_per_frame: 5,
            // As long as the weather file covers, at one row per hour, like C2F-W does.
            stop_after_hours: instance.weather.len().max(1) as u32,
            fuel_colours,
            map_texture,
            zoom: 1.0,
            pan: Vec2::ZERO,
            error: None,
        })
    }

    /// Starts again from an unburnt landscape, with the current settings.
    fn reset(&mut self) -> Result<(), SimulationError> {
        let mut simulation = Simulation::new(self.landscape.clone(), &self.settings)?;
        if let Some(cell) = self.initial_ignition {
            simulation.ignite(cell, &self.wind)?;
        }
        self.simulation = simulation;
        self.playing = false;
        Ok(())
    }

    fn advance(&mut self, minutes: u32) {
        for _ in 0..minutes {
            if self.simulation.is_over() || self.time_limit_reached() {
                self.playing = false;
                break;
            }
            self.simulation.step(&self.wind);
        }
    }

    fn time_limit_reached(&self) -> bool {
        self.simulation.time() >= self.stop_after_hours as f32 * 60.0
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("c2f-rs viewer");
        ui.label(format!(
            "{} × {} cells of {} m",
            self.landscape.columns, self.landscape.rows, self.landscape.cell_size
        ));
        ui.separator();

        ui.horizontal(|ui| {
            let play_text = if self.playing {
                "⏸ Pause"
            } else {
                "▶ Play"
            };
            if ui.button(play_text).clicked() {
                self.playing = !self.playing;
            }
            if ui.button("⏭ Step").clicked() {
                self.advance(1);
            }
            if ui.button("↺ Reset").clicked() {
                let result = self.reset();
                self.show_error(result);
            }
        });
        ui.label("Speed: simulated minutes per frame");
        ui.add(egui::Slider::new(&mut self.minutes_per_frame, 1..=60));
        ui.label("Stop after");
        ui.add(egui::Slider::new(&mut self.stop_after_hours, 1..=168).suffix(" h"));

        let burnt_count = self.simulation.fire().burnt_cells.len();
        ui.label(format!("Time: {}", format_minutes(self.simulation.time())));
        ui.label(format!("Burnt cells: {burnt_count}"));
        if burnt_count == 0 {
            ui.label("Click on the map to start a fire.");
        } else if self.simulation.is_over() {
            ui.label("The fire is over. Click on the map to start another one.");
        } else if self.time_limit_reached() {
            ui.label(format!(
                "Stopped after {} h. Raise the limit to continue.",
                self.stop_after_hours
            ));
        }
        ui.separator();

        ui.label("Wind");
        ui.add(egui::Slider::new(&mut self.wind.wind_speed, 0.0..=80.0).suffix(" km/h"));
        ui.add(egui::Slider::new(&mut self.wind.wind_direction, 0.0..=359.0).suffix("°"));
        ui.label(format!(
            "Blowing from the {}",
            compass_name(self.wind.wind_direction)
        ));
        self.wind_compass(ui);
        ui.separator();

        ui.label("Fuel moisture (resets the fire)");
        let previous_scenario = self.settings.moisture_scenario;
        let scenarios = [(1, "1 (driest)"), (2, "2"), (3, "3"), (4, "4 (wettest)")];
        for (scenario, name) in scenarios {
            if ui
                .radio_value(&mut self.settings.moisture_scenario, scenario, name)
                .changed()
            {
                let result = self.reset();
                if result.is_err() {
                    self.settings.moisture_scenario = previous_scenario;
                }
                self.show_error(result);
            }
        }

        if let Some(error) = &self.error {
            ui.separator();
            ui.colored_label(Color32::RED, error);
        }
    }

    /// A compass with an arrow pointing where the wind blows to, which is where it pushes
    /// the fire.
    fn wind_compass(&self, ui: &mut egui::Ui) {
        let size = 100.0;
        let (response, painter) = ui.allocate_painter(vec2(size, size), Sense::hover());
        let center = response.rect.center();
        let radius = size / 2.0 - 14.0;
        let grey = Color32::GRAY;

        painter.circle_stroke(center, radius, Stroke::new(1.0, grey));
        let letters = [
            ("N", vec2(0.0, -1.0)),
            ("E", vec2(1.0, 0.0)),
            ("S", vec2(0.0, 1.0)),
            ("W", vec2(-1.0, 0.0)),
        ];
        for (letter, direction) in letters {
            let position = center + direction * (radius + 8.0);
            painter.text(
                position,
                Align2::CENTER_CENTER,
                letter,
                FontId::proportional(11.0),
                grey,
            );
        }

        if self.wind.wind_speed == 0.0 {
            painter.text(
                center,
                Align2::CENTER_CENTER,
                "calm",
                FontId::proportional(12.0),
                grey,
            );
            return;
        }
        // The wind blows towards the opposite of where it comes from. On the screen,
        // north is up and y grows downwards.
        let towards = (self.wind.wind_direction + 180.0).to_radians() as f32;
        let direction = vec2(towards.sin(), -towards.cos());
        let tail = center - direction * radius * 0.8;
        let arrow_color = Color32::from_rgb(40, 110, 200);
        painter.arrow(
            tail,
            direction * radius * 1.6,
            Stroke::new(3.0, arrow_color),
        );
    }

    fn show_error(&mut self, result: Result<(), SimulationError>) {
        self.error = match result {
            Ok(()) => None,
            Err(error) => Some(error.to_string()),
        };
    }

    fn map(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        // Drag to move the map, scroll to zoom.
        if response.dragged() {
            self.pan += response.drag_delta();
        }
        if response.hovered() {
            let scroll = ui.input(|input| input.smooth_scroll_delta.y);
            self.zoom = (self.zoom * (1.0 + scroll * 0.002)).clamp(0.5, 50.0);
        }

        let map_rect = self.map_rect(response.rect);
        if response.clicked()
            && let Some(position) = response.interact_pointer_pos()
            && let Some(cell) = self.cell_at(position, map_rect)
        {
            let result = self.simulation.ignite(cell, &self.wind);
            self.show_error(result);
        }

        self.map_texture
            .set(self.map_image(), TextureOptions::NEAREST);
        let whole_texture = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
        painter.image(
            self.map_texture.id(),
            map_rect,
            whole_texture,
            Color32::WHITE,
        );
    }

    /// The fuel colours, with every burnt cell coloured by how long ago it caught fire.
    fn map_image(&self) -> ColorImage {
        let mut pixels = self.fuel_colours.clone();
        let now = self.simulation.time();
        for burnt in &self.simulation.fire().burnt_cells {
            pixels[burnt.cell] = fire_colour(now - burnt.time);
        }
        ColorImage::new([self.landscape.columns, self.landscape.rows], pixels)
    }

    /// Where the map is drawn: as large as fits in `area` at zoom 1, then zoomed and moved.
    fn map_rect(&self, area: Rect) -> Rect {
        let columns = self.landscape.columns as f32;
        let rows = self.landscape.rows as f32;
        let fitting_cell_size = (area.width() / columns).min(area.height() / rows);
        let cell_size = fitting_cell_size * self.zoom;
        Rect::from_center_size(
            area.center() + self.pan,
            vec2(columns * cell_size, rows * cell_size),
        )
    }

    /// The cell under a point of the screen, if any.
    fn cell_at(&self, position: Pos2, map_rect: Rect) -> Option<usize> {
        if !map_rect.contains(position) {
            return None;
        }
        let x_fraction = (position.x - map_rect.min.x) / map_rect.width();
        let y_fraction = (position.y - map_rect.min.y) / map_rect.height();
        let column =
            ((x_fraction * self.landscape.columns as f32) as usize).min(self.landscape.columns - 1);
        let row = ((y_fraction * self.landscape.rows as f32) as usize).min(self.landscape.rows - 1);
        Some(row * self.landscape.columns + column)
    }
}

impl eframe::App for Viewer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.playing {
            self.advance(self.minutes_per_frame);
            ui.ctx().request_repaint();
        }

        egui::Panel::left("controls")
            .resizable(false)
            .exact_size(240.0)
            .show(ui, |ui| self.controls(ui));
        egui::CentralPanel::default().show(ui, |ui| self.map(ui));
    }
}

/// For example "2 h 05 min".
fn format_minutes(minutes: f32) -> String {
    let whole_minutes = minutes.round() as u32;
    format!("{} h {:02} min", whole_minutes / 60, whole_minutes % 60)
}

/// The nearest of the 8 compass directions, for a bearing in degrees.
fn compass_name(degrees: f64) -> &'static str {
    let names = [
        "north",
        "north-east",
        "east",
        "south-east",
        "south",
        "south-west",
        "west",
        "north-west",
    ];
    let index = (degrees / 45.0).round() as usize % 8;
    names[index]
}
