//! Elliptical fire shape.
//!
//! A fire grows as a wind-oriented ellipse, with ignition at the rear focus.
//! Spread is fastest downwind (the head), slowest upwind (the back), and
//! between those rates in other directions.
//!
//! Directions are angles in degrees with 0 = east, 90 = north (counterclockwise).

/// Rates of spread in every direction around a burning point.
#[derive(Debug, Clone, Copy)]
pub struct FireEllipse {
    semi_major: f32,
    semi_minor: f32,
    heading: f64,
}

impl FireEllipse {
    /// Builds the ellipse from the head and back rates of spread (m/min), the
    /// length-to-breadth ratio, and the direction the head fire moves towards.
    pub fn new(head_ros: f32, back_ros: f32, length_to_breadth: f32, heading: f64) -> Self {
        FireEllipse {
            semi_major: (head_ros + back_ros) / 2.0,
            semi_minor: (head_ros + back_ros) / (2.0 * length_to_breadth),
            heading,
        }
    }

    /// Rate of spread towards `direction`, in m/min.
    pub fn spread_rate_towards(&self, direction: f64) -> f64 {
        let semi_major = f64::from(self.semi_major);
        let semi_minor = f64::from(self.semi_minor);
        let eccentricity = (semi_major * semi_major - semi_minor * semi_minor).sqrt() / semi_major;
        let angle_from_head = (direction - self.heading).to_radians();
        // Distance from the rear focus to the edge of the ellipse, in polar form.
        semi_major * (1.0 - eccentricity * eccentricity)
            / (1.0 - eccentricity * angle_from_head.cos())
    }
}

/// Converts the wind's source bearing to the fire's direction of travel.
///
/// Weather files give the compass bearing the wind blows from (0 = north,
/// clockwise). The returned direction points downwind, with 0 = east and
/// angles increasing counterclockwise.
pub fn heading_from_wind(wind_from: f64) -> f64 {
    let bearing_towards = (wind_from + 180.0) % 360.0;
    (90.0 - bearing_towards).rem_euclid(360.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scott_burgan;

    #[test]
    fn head_and_back_of_the_ellipse_match_the_fuel_model_rates() {
        let fuel = scott_burgan::coefficients(1, 123).unwrap();
        let fire = scott_burgan::surface_fire(fuel, 10.0);
        let ellipse = FireEllipse::new(fire.head_ros, fire.back_ros, fire.length_to_breadth, 90.0);

        let head = ellipse.spread_rate_towards(90.0);
        let back = ellipse.spread_rate_towards(270.0);
        assert!(
            (head - f64::from(fire.head_ros)).abs() < 1e-4,
            "head rate was {head}"
        );
        assert!(
            (back - f64::from(fire.back_ros)).abs() < 1e-4,
            "back rate was {back}"
        );
    }

    #[test]
    fn wind_from_the_south_pushes_the_fire_north() {
        assert_eq!(heading_from_wind(180.0), 90.0);
        assert_eq!(heading_from_wind(270.0), 0.0);
        assert_eq!(heading_from_wind(0.0), 270.0);
        assert_eq!(heading_from_wind(45.0), 225.0);
    }
}
