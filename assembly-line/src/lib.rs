pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as f64;
    if speed >= 9.0 && speed <= 10.0 {
        speed * 0.77 * 221.0
    } else if speed >= 5.0 && speed <= 8.0 {
        speed * 0.9 * 221.0
    } else if speed >= 1.0 && speed <=4.0 {
        speed * 1.0 * 221.0
    } else {
        0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
