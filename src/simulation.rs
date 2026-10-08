//! Fire spread over the landscape, one fire period (a minute by default) at a time.
//!
//! Every burning cell spreads towards its neighbours that have fuel and are not on fire
//! yet. The fire advances towards each of them at the rate given by the fire ellipse.
//! When it reaches the centre of a neighbour, a message is sent, and the neighbour
//! ignites if its own fire would be strong enough. A burning cell burns out when its
//! fire is too weak or there is nothing left around it to burn.
//!
//! A [`Simulation`] is a running fire that advances one step at a time, so a program can
//! look at it, or change the weather, between steps. [`simulate`] runs a whole fire with
//! a table of hourly weather.

use crate::ellipse::{FireEllipse, heading_from_wind};
use crate::error::SimulationError;
use crate::landscape::Landscape;
use crate::scott_burgan::{self, FuelCoefficients, SurfaceFire};
use crate::weather::Weather;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// Fuel moisture scenario, from 1 (driest) to 4 (wettest).
    pub moisture_scenario: u8,
    /// Length of a simulation step, in minutes.
    pub fire_period_minutes: f32,
    /// How long each weather row lasts, in minutes.
    pub weather_period_minutes: f32,
    /// A cell only ignites and spreads if its head rate of spread is above this, in m/min.
    pub ros_threshold: f32,
}

impl Default for Settings {
    /// The same defaults as C2F-W.
    fn default() -> Self {
        Settings {
            moisture_scenario: 3,
            fire_period_minutes: 1.0,
            weather_period_minutes: 60.0,
            ros_threshold: 0.1,
        }
    }
}

/// The fire spread from one cell to a neighbour: it reached the neighbour's centre and
/// set it on fire.
#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    /// Minutes after ignition when `to` caught fire.
    pub time: f32,
    /// Rate of spread from `from` towards `to`, in m/min.
    pub rate: f32,
}

/// A cell that caught fire, with its fire behaviour at that moment.
#[derive(Debug, Clone, Copy)]
pub struct BurntCell {
    pub cell: usize,
    /// Minutes after ignition when the cell caught fire.
    pub time: f32,
    /// Byram fireline intensity, in kW/m.
    pub intensity: f32,
    /// Flame length, in m.
    pub flame_length: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Fire {
    pub messages: Vec<Message>,
    /// Every cell that caught fire, in the order they caught fire.
    pub burnt_cells: Vec<BurntCell>,
}

/// What happened during one step.
pub struct Changes {
    /// Cells that caught fire.
    pub burnt_cells: Vec<BurntCell>,
    /// The spreads that set them on fire.
    pub messages: Vec<Message>,
}

#[derive(Clone)]
struct BurningCell {
    cell: usize,
    /// How far the fire has travelled towards each neighbour, in m, by neighbour position.
    progress: [f64; 8],
}

/// A running fire. Copying it gives an independent fire that can continue differently.
#[derive(Clone)]
pub struct Simulation {
    landscape: Landscape,
    settings: Settings,
    /// The fuel model coefficients of every cell, or `None` for cells without fuel.
    fuels: Vec<Option<&'static FuelCoefficients>>,
    /// Cells with fuel that have not caught fire yet.
    available: Vec<bool>,
    available_count: usize,
    burning: Vec<BurningCell>,
    steps_done: u32,
    fire: Fire,
}

impl Simulation {
    /// A landscape with no fire yet.
    pub fn new(landscape: Landscape, settings: &Settings) -> Result<Self, SimulationError> {
        let fuels = fuel_coefficients(&landscape, settings.moisture_scenario)?;
        // At the start, every cell with fuel is available.
        let available: Vec<bool> = fuels.iter().map(|fuel| fuel.is_some()).collect();
        let available_count = fuels.iter().filter(|fuel| fuel.is_some()).count();

        Ok(Simulation {
            landscape,
            settings: *settings,
            fuels,
            available,
            available_count,
            burning: Vec::new(),
            steps_done: 0,
            fire: Fire::default(),
        })
    }

    /// Starts a fire at `cell`, under the current weather. Nothing happens if the cell has
    /// no fuel or is already on fire.
    pub fn ignite(&mut self, cell: usize, weather: &Weather) -> Result<(), SimulationError> {
        if cell >= self.available.len() {
            return Err(SimulationError::CellOutsideLandscape {
                cell,
                cell_count: self.available.len(),
            });
        }
        // Cells without fuel, and cells already on fire, are not available.
        if !self.available[cell] {
            return Ok(());
        }
        let fuel = self.fuels[cell].expect("available cells have fuel");
        let cell_fire = surface_fire(fuel, weather);
        self.set_on_fire(cell, self.time(), &cell_fire);
        Ok(())
    }

    /// Advances the fire by one fire period, under `weather`.
    pub fn step(&mut self, weather: &Weather) -> Changes {
        let end_time = self.time() + self.settings.fire_period_minutes;
        let first_new_burnt_cell = self.fire.burnt_cells.len();
        let first_new_message = self.fire.messages.len();

        // The burning cells are taken out of the simulation while they spread, so that
        // `spread` can read the rest of it.
        let mut arrivals = Vec::new();
        let mut still_burning = Vec::new();
        for mut burning_cell in std::mem::take(&mut self.burning) {
            if self.spread(&mut burning_cell, weather, end_time, &mut arrivals) {
                still_burning.push(burning_cell);
            }
        }
        self.burning = still_burning;

        let mut reached: Vec<usize> = arrivals.iter().map(|arrival| arrival.to).collect();
        reached.sort_unstable();
        reached.dedup();
        for cell in reached {
            let fuel = self.fuels[cell].expect("the fire only reaches cells with fuel");
            let cell_fire = surface_fire(fuel, weather);
            if cell_fire.head_ros > self.settings.ros_threshold {
                self.set_on_fire(cell, end_time, &cell_fire);
            }
        }

        // Only arrivals that set a cell on fire become messages. A cell that could not
        // ignite stays available, and will be reached again next step.
        let ignitions = arrivals
            .into_iter()
            .filter(|arrival| !self.available[arrival.to]);
        self.fire.messages.extend(ignitions);

        self.steps_done += 1;
        Changes {
            burnt_cells: self.fire.burnt_cells[first_new_burnt_cell..].to_vec(),
            messages: self.fire.messages[first_new_message..].to_vec(),
        }
    }

    /// The fire is over when no cell is burning, or there is nothing left to burn.
    pub fn is_over(&self) -> bool {
        self.burning.is_empty() || self.available_count == 0
    }

    /// Minutes since the simulation started.
    pub fn time(&self) -> f32 {
        self.steps_done as f32 * self.settings.fire_period_minutes
    }

    pub fn landscape(&self) -> &Landscape {
        &self.landscape
    }

    /// The fire so far.
    pub fn fire(&self) -> &Fire {
        &self.fire
    }

    pub fn into_fire(self) -> Fire {
        self.fire
    }

    /// Advances the fire of a burning cell for one fire period, towards its available
    /// neighbours, and records an arrival for every neighbour whose centre has been
    /// reached by `end_time`. A neighbour that could not ignite is reached again every
    /// period, until it catches fire.
    ///
    /// Returns false when the cell has burnt out.
    fn spread(
        &self,
        burning_cell: &mut BurningCell,
        weather: &Weather,
        end_time: f32,
        arrivals: &mut Vec<Message>,
    ) -> bool {
        let fuel = self.fuels[burning_cell.cell].expect("only cells with fuel burn");
        let fire = surface_fire(fuel, weather);
        if fire.head_ros <= self.settings.ros_threshold {
            return false;
        }

        let heading = heading_from_wind(weather.wind_direction);
        let ellipse = FireEllipse::new(
            fire.head_ros,
            fire.back_ros,
            fire.length_to_breadth,
            heading,
        );
        let mut has_something_to_burn = false;

        for neighbour in self.landscape.neighbours(burning_cell.cell) {
            if !self.available[neighbour.cell] {
                continue;
            }
            has_something_to_burn = true;

            let rate = ellipse.spread_rate_towards(neighbour.direction) as f32;
            let progress = &mut burning_cell.progress[neighbour.position];
            *progress += f64::from(rate * self.settings.fire_period_minutes);
            if *progress >= neighbour.distance {
                arrivals.push(Message {
                    from: burning_cell.cell,
                    to: neighbour.cell,
                    time: end_time,
                    rate,
                });
            }
        }

        has_something_to_burn
    }

    fn set_on_fire(&mut self, cell: usize, time: f32, cell_fire: &SurfaceFire) {
        self.available[cell] = false;
        self.available_count -= 1;
        self.fire.burnt_cells.push(BurntCell {
            cell,
            time,
            intensity: cell_fire.intensity,
            flame_length: cell_fire.flame_length,
        });
        self.burning.push(BurningCell {
            cell,
            progress: [0.0; 8],
        });
    }
}

/// Simulates one fire, started at `ignition`, until it dies out, there is nothing
/// left to burn, or the weather runs out. Each weather row lasts
/// `settings.weather_period_minutes`.
pub fn simulate(
    landscape: &Landscape,
    weather: &[Weather],
    ignition: usize,
    settings: &Settings,
) -> Result<Fire, SimulationError> {
    if weather.is_empty() {
        return Err(SimulationError::NoWeather);
    }
    let mut simulation = Simulation::new(landscape.clone(), settings)?;
    simulation.ignite(ignition, &weather[0])?;

    // The fire can last as long as the weather rows cover.
    let step_count = (settings.weather_period_minutes / settings.fire_period_minutes
        * weather.len() as f32) as u32;
    for _ in 0..step_count {
        if simulation.is_over() {
            break;
        }
        let weather_row = (simulation.time() / settings.weather_period_minutes) as usize;
        simulation.step(&weather[weather_row]);
    }

    Ok(simulation.into_fire())
}

fn surface_fire(fuel: &FuelCoefficients, weather: &Weather) -> SurfaceFire {
    scott_burgan::surface_fire(fuel, weather.wind_speed)
}

/// The fuel model coefficients of every cell, or `None` for cells without fuel.
fn fuel_coefficients(
    landscape: &Landscape,
    scenario: u8,
) -> Result<Vec<Option<&'static FuelCoefficients>>, SimulationError> {
    let mut fuels = Vec::with_capacity(landscape.fuels.len());
    for fuel_code in &landscape.fuels {
        let coefficients = match fuel_code {
            None => None,
            Some(code) => {
                let found = scott_burgan::coefficients(scenario, *code).ok_or(
                    SimulationError::UnknownFuel {
                        code: *code,
                        scenario,
                    },
                )?;
                Some(found)
            }
        };
        fuels.push(coefficients);
    }
    Ok(fuels)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GR4 grass: spreads fast even without wind.
    const GRASS: u16 = 104;
    /// TL1 timber litter: too slow to ignite without wind in the driest scenario.
    const LITTER: u16 = 181;

    const CALM: Weather = Weather {
        wind_speed: 0.0,
        wind_direction: 270.0,
    };
    const WINDY: Weather = Weather {
        wind_speed: 20.0,
        wind_direction: 270.0,
    };

    fn grass_next_to_litter() -> Landscape {
        Landscape {
            rows: 1,
            columns: 2,
            cell_size: 100.0,
            lower_left_x: 0.0,
            lower_left_y: 0.0,
            fuels: vec![Some(GRASS), Some(LITTER)],
        }
    }

    fn driest_scenario() -> Settings {
        Settings {
            moisture_scenario: 1,
            ..Settings::default()
        }
    }

    #[test]
    fn a_cell_that_cannot_ignite_yet_gets_a_message_only_when_it_catches_fire() {
        let landscape = grass_next_to_litter();
        let fire = simulate(&landscape, &[CALM, WINDY], 0, &driest_scenario()).unwrap();

        // The grass fire reaches the litter within minutes, but the litter only ignites
        // when the wind rises, in the first minute of the second hour.
        let burnt: Vec<usize> = fire.burnt_cells.iter().map(|burnt| burnt.cell).collect();
        assert_eq!(burnt, vec![0, 1]);
        assert_eq!(fire.messages.len(), 1);
        assert_eq!(fire.messages[0].to, 1);
        assert_eq!(fire.messages[0].time, 61.0);
    }

    #[test]
    fn each_step_reports_what_changed_under_the_weather_it_is_given() {
        let mut simulation = Simulation::new(grass_next_to_litter(), &driest_scenario()).unwrap();
        simulation.ignite(0, &CALM).unwrap();

        for _ in 0..10 {
            let changes = simulation.step(&CALM);
            assert!(changes.burnt_cells.is_empty());
        }

        // The wind rises after 10 minutes, and the litter catches fire straight away.
        let changes = simulation.step(&WINDY);
        assert_eq!(changes.burnt_cells.len(), 1);
        assert_eq!(changes.burnt_cells[0].cell, 1);
        assert_eq!(changes.burnt_cells[0].time, 11.0);
        assert_eq!(changes.messages.len(), 1);
        assert!(simulation.is_over());
    }
}
