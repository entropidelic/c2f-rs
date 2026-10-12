//! Muted fuel colours and bright fire colours.

use eframe::egui::Color32;

const NO_FUEL: Color32 = Color32::from_rgb(214, 219, 224);

/// A muted colour per group of Scott & Burgan fuel models.
pub fn fuel_colour(fuel_code: Option<u16>) -> Color32 {
    let Some(code) = fuel_code else {
        return NO_FUEL;
    };
    match code {
        101..=109 => Color32::from_rgb(214, 214, 150), // grass
        121..=124 => Color32::from_rgb(186, 204, 140), // grass and shrubs
        141..=149 => Color32::from_rgb(150, 180, 120), // shrubs
        161..=165 => Color32::from_rgb(112, 150, 104), // timber with understory
        181..=189 => Color32::from_rgb(134, 140, 98),  // timber litter
        _ => Color32::from_rgb(180, 180, 160),
    }
}

/// Colour by time since ignition: yellow at the fire front, then orange, red
/// and dark ash after a few hours.
pub fn fire_colour(minutes_since_ignition: f32) -> Color32 {
    let stops = [
        (0.0, Color32::from_rgb(255, 245, 140)),
        (15.0, Color32::from_rgb(255, 150, 20)),
        (60.0, Color32::from_rgb(210, 50, 25)),
        (180.0, Color32::from_rgb(90, 25, 25)),
        (360.0, Color32::from_rgb(55, 45, 45)),
    ];

    for pair in stops.windows(2) {
        let (start_minutes, start_colour) = pair[0];
        let (end_minutes, end_colour) = pair[1];
        if minutes_since_ignition <= end_minutes {
            let fraction = (minutes_since_ignition - start_minutes) / (end_minutes - start_minutes);
            return mix(start_colour, end_colour, fraction.max(0.0));
        }
    }
    stops[stops.len() - 1].1
}

/// A colour between `from` and `to`, `fraction` of the way.
fn mix(from: Color32, to: Color32, fraction: f32) -> Color32 {
    let channel = |from: u8, to: u8| {
        let value = from as f32 + (to as f32 - from as f32) * fraction;
        value.round() as u8
    };
    Color32::from_rgb(
        channel(from.r(), to.r()),
        channel(from.g(), to.g()),
        channel(from.b(), to.b()),
    )
}
