// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    let rate: f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    221.0 * speed as f64 * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    (production_rate_per_hour(speed) / 60.0) as u32
}
