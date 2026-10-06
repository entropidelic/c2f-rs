//! Fire spread over the landscape, one fire period (a minute by default) at a time.
//!
//! Every burning cell spreads towards its neighbours that have fuel and are not on fire
//! yet. The fire advances towards each of them at the rate given by the fire ellipse.
//! When it reaches the centre of a neighbour, a message is sent, and the neighbour
//! ignites if its own fire would be strong enough. A burning cell burns out when its
//! fire is too weak or there is nothing left around it to burn.

use crate::ellipse::{FireEllipse, heading_from_wind};
use crate::landscape::Landscape;
use crate::scott_burgan::{self, FuelCoefficients, SurfaceFire};
use crate::weather::Weather;

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

#[derive(Debug, Default)]
pub struct Fire {
    pub messages: Vec<Message>,
    /// Every cell that caught fire, starting with the ignition cell.
    pub burnt_cells: Vec<BurntCell>,
}

struct BurningCell {
    cell: usize,
    /// How far the fire has travelled towards each neighbour, in m, by neighbour position.
    progress: [f64; 8],
}

/// Simulates one fire, started at `ignition`, until it dies out, there is nothing
/// left to burn, or the weather runs out.
pub fn simulate(
    landscape: &Landscape,
    weather: &[Weather],
    ignition: usize,
    settings: &Settings,
) -> Result<Fire, String> {
    let fuels = fuel_coefficients(landscape, settings.moisture_scenario)?;
    if ignition >= fuels.len() {
        return Err(format!(
            "the ignition cell is outside the landscape, which has {} cells",
            fuels.len()
        ));
    }
    if weather.is_empty() {
        return Err("the weather has no rows".to_string());
    }

    let mut fire = Fire::default();
    let Some(ignition_fuel) = fuels[ignition] else {
        return Ok(fire);
    };

    // Cells with fuel that have not caught fire yet.
    let mut available: Vec<bool> = fuels.iter().map(|fuel| fuel.is_some()).collect();
    let mut available_count = available
        .iter()
        .filter(|&&is_available| is_available)
        .count();

    available[ignition] = false;
    available_count -= 1;
    let ignition_fire = surface_fire(ignition_fuel, &weather[0]);
    fire.burnt_cells.push(BurntCell {
        cell: ignition,
        time: 0.0,
        intensity: ignition_fire.intensity,
        flame_length: ignition_fire.flame_length,
    });
    let mut burning = vec![BurningCell {
        cell: ignition,
        progress: [0.0; 8],
    }];

    // The fire can last as long as the weather rows cover.
    let period_count = (settings.weather_period_minutes / settings.fire_period_minutes
        * weather.len() as f32) as u32;
    for period in 0..period_count {
        let start_time = period as f32 * settings.fire_period_minutes;
        let end_time = start_time + settings.fire_period_minutes;
        let current_weather = &weather[(start_time / settings.weather_period_minutes) as usize];

        let mut arrivals = Vec::new();
        burning.retain_mut(|burning_cell| {
            spread(
                burning_cell,
                landscape,
                &fuels,
                &available,
                current_weather,
                settings,
                end_time,
                &mut arrivals,
            )
        });

        let fire_died_out = arrivals.is_empty() && burning.is_empty();
        if fire_died_out {
            break;
        }

        let mut reached: Vec<usize> = arrivals.iter().map(|arrival| arrival.to).collect();
        reached.sort_unstable();
        reached.dedup();
        for cell in reached {
            let fuel = fuels[cell].expect("the fire only reaches cells with fuel");
            let cell_fire = surface_fire(fuel, current_weather);
            if cell_fire.head_ros > settings.ros_threshold {
                available[cell] = false;
                available_count -= 1;
                fire.burnt_cells.push(BurntCell {
                    cell,
                    time: end_time,
                    intensity: cell_fire.intensity,
                    flame_length: cell_fire.flame_length,
                });
                burning.push(BurningCell {
                    cell,
                    progress: [0.0; 8],
                });
            }
        }

        // Only arrivals that set a cell on fire become messages. A cell that could not
        // ignite stays available, and will be reached again next period.
        let ignitions = arrivals
            .into_iter()
            .filter(|arrival| !available[arrival.to]);
        fire.messages.extend(ignitions);

        if available_count == 0 {
            break;
        }
    }

    Ok(fire)
}

/// Advances the fire of a burning cell for one fire period, towards its available
/// neighbours, and records an arrival for every neighbour whose centre has been reached
/// by `end_time`. A neighbour that could not ignite is reached again every period, until
/// it catches fire.
///
/// Returns false when the cell has burnt out.
#[allow(clippy::too_many_arguments)]
fn spread(
    burning_cell: &mut BurningCell,
    landscape: &Landscape,
    fuels: &[Option<&FuelCoefficients>],
    available: &[bool],
    weather: &Weather,
    settings: &Settings,
    end_time: f32,
    arrivals: &mut Vec<Message>,
) -> bool {
    let fuel = fuels[burning_cell.cell].expect("only cells with fuel burn");
    let fire = surface_fire(fuel, weather);
    if fire.head_ros <= settings.ros_threshold {
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

    for neighbour in landscape.neighbours(burning_cell.cell) {
        if !available[neighbour.cell] {
            continue;
        }
        has_something_to_burn = true;

        let rate = ellipse.spread_rate_towards(neighbour.direction) as f32;
        let progress = &mut burning_cell.progress[neighbour.position];
        *progress += f64::from(rate * settings.fire_period_minutes);
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

fn surface_fire(fuel: &FuelCoefficients, weather: &Weather) -> SurfaceFire {
    scott_burgan::surface_fire(fuel, weather.wind_speed)
}

/// The fuel model coefficients of every cell, or `None` for cells without fuel.
fn fuel_coefficients(
    landscape: &Landscape,
    scenario: u8,
) -> Result<Vec<Option<&'static FuelCoefficients>>, String> {
    landscape
        .fuels
        .iter()
        .map(|fuel_code| match fuel_code {
            None => Ok(None),
            Some(code) => scott_burgan::coefficients(scenario, *code)
                .map(Some)
                .ok_or(format!(
                    "no Scott & Burgan fuel model {code} for moisture scenario {scenario}"
                )),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GR4 grass: spreads fast even without wind.
    const GRASS: u16 = 104;
    /// TL1 timber litter: too slow to ignite without wind in the driest scenario.
    const LITTER: u16 = 181;

    #[test]
    fn a_cell_that_cannot_ignite_yet_gets_a_message_only_when_it_catches_fire() {
        let landscape = Landscape {
            rows: 1,
            columns: 2,
            cell_size: 100.0,
            lower_left_x: 0.0,
            lower_left_y: 0.0,
            fuels: vec![Some(GRASS), Some(LITTER)],
        };
        let calm = Weather {
            wind_speed: 0.0,
            wind_direction: 270.0,
        };
        let windy = Weather {
            wind_speed: 20.0,
            wind_direction: 270.0,
        };
        let settings = Settings {
            moisture_scenario: 1,
            ..Settings::default()
        };

        let fire = simulate(&landscape, &[calm, windy], 0, &settings).unwrap();

        // The grass fire reaches the litter within minutes, but the litter only ignites
        // when the wind rises, in the first minute of the second hour.
        let burnt: Vec<usize> = fire.burnt_cells.iter().map(|burnt| burnt.cell).collect();
        assert_eq!(burnt, vec![0, 1]);
        assert_eq!(fire.messages.len(), 1);
        assert_eq!(fire.messages[0].to, 1);
        assert_eq!(fire.messages[0].time, 61.0);
    }
}
