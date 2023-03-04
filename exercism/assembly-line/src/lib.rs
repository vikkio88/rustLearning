const PRODUCE_PER_HOUR: f64 = 221.0;
const MINUTES_IN_HOUR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base = f64::from(speed) * PRODUCE_PER_HOUR;

    base * match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32) / MINUTES_IN_HOUR
}
