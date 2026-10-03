//! Scott & Burgan surface fire model, as used by C2F-W.
//!
//! C2F-W does not run the physical Rothermel equations. For every Scott & Burgan
//! fuel model and fuel moisture scenario it uses two curves fitted against wind
//! speed: one for the head rate of spread and one for the flame length.
//! Everything else is derived from those two values and the wind speed.

mod coefficients;

/// Fitted curve coefficients for one fuel model.
#[derive(Debug)]
pub struct FuelCoefficients {
    /// Fuel model code, as found in the fuel rasters (101 = GR1, 102 = GR2, ...).
    pub code: u16,
    /// Head rate of spread curve: `1 / (s1 * exp(-s2 * wind) + s3)`.
    pub spread: [f32; 3],
    /// Flame length curve: `(f1 * exp(-f2 * wind) + f3)^2`.
    pub flame: [f32; 3],
}

/// Surface fire behaviour of a burning cell.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceFire {
    /// Rate of spread with the wind, in m/min.
    pub head_ros: f32,
    /// Rate of spread against the wind, in m/min.
    pub back_ros: f32,
    /// Length of the fire shape divided by its width.
    pub length_to_breadth: f32,
    /// Flame length, in m.
    pub flame_length: f32,
    /// Byram fireline intensity, in kW/m.
    pub intensity: f32,
}

/// Looks up a fuel model for a fuel moisture scenario.
///
/// Scenarios go from 1 (driest, D1L1) to 4 (wettest, D4L4).
pub fn coefficients(scenario: u8, fuel_code: u16) -> Option<&'static FuelCoefficients> {
    let table = match scenario {
        1 => &coefficients::SCENARIO_1,
        2 => &coefficients::SCENARIO_2,
        3 => &coefficients::SCENARIO_3,
        4 => &coefficients::SCENARIO_4,
        _ => return None,
    };
    table.iter().find(|fuel| fuel.code == fuel_code)
}

/// Computes the surface fire behaviour for a wind speed in km/h.
pub fn surface_fire(fuel: &FuelCoefficients, wind_speed: f32) -> SurfaceFire {
    let head_ros = head_ros(fuel, wind_speed);
    let length_to_breadth = length_to_breadth(wind_speed);
    let back_ros = back_ros(head_ros, length_to_breadth);
    let flame_length = flame_length(fuel, wind_speed);
    let intensity = intensity(flame_length);

    SurfaceFire {
        head_ros,
        back_ros,
        length_to_breadth,
        flame_length,
        intensity,
    }
}

fn head_ros(fuel: &FuelCoefficients, wind_speed: f32) -> f32 {
    let [s1, s2, s3] = fuel.spread;
    let ros = 1.0 / (s1 * (-s2 * wind_speed).exp() + s3);
    // Fuels that do not burn in a scenario have coefficients that make this negative.
    ros.max(0.0)
}

/// Anderson (1983) length-to-breadth ratio, raised to the power 0.45 as C2F-W does.
fn length_to_breadth(wind_speed: f32) -> f32 {
    let kmh_to_ms: f32 = 1000.0 / 3600.0;
    let fast_term = 0.936 * (0.2566 * kmh_to_ms * wind_speed).exp();
    let slow_term = 0.461 * (-0.1548 * kmh_to_ms * wind_speed).exp();
    (fast_term + slow_term - 0.397).powf(0.45)
}

/// Back rate of spread from the head rate and the fire shape (Forestry Canada, 1992).
fn back_ros(head_ros: f32, length_to_breadth: f32) -> f32 {
    let root = (length_to_breadth * length_to_breadth - 1.0).sqrt();
    let head_to_back_ratio = (length_to_breadth + root) / (length_to_breadth - root);
    head_ros / head_to_back_ratio
}

fn flame_length(fuel: &FuelCoefficients, wind_speed: f32) -> f32 {
    let [f1, f2, f3] = fuel.flame;
    let root = f1 * (-f2 * wind_speed).exp() + f3;
    root * root
}

/// Byram's relation between flame length and intensity, solved for intensity.
fn intensity(flame_length: f32) -> f32 {
    let intensity = 259.833 * flame_length.powf(2.174);
    // C2F-W rounds the intensity up to two decimals.
    (intensity * 100.0).ceil() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuels_that_do_not_burn_in_a_scenario_have_no_spread() {
        // GR1 is too wet to burn in the wettest scenario.
        let grass = coefficients(4, 101).unwrap();
        let fire = surface_fire(grass, 30.0);
        assert_eq!(fire.head_ros, 0.0);
        assert_eq!(fire.intensity, 0.0);
    }
}
