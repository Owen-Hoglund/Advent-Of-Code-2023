// FINAL SOLUTION FOR DAY SIX

pub fn day_six(_file: String) {
    // Used in part one
    // let time: [u32; 4] = [40, 92, 97, 90];
    // let record_distance: [u32; 4] = [215, 1064, 1505, 1100];

    let (min, max) = calculate_min_max_button_time(&40929790, &215106415051100);
    println!("{}", margin_of_error(min, max));
}

fn calculate_min_max_button_time(race_time: &u64, record_distance: &u64) -> (u64, u64) {
    let race_time = *race_time as f64;
    let record_distance = (record_distance + 1) as f64;
    let a = (race_time + f64::sqrt(race_time * race_time - 4.0 * record_distance)) / 2.0;
    let b = (race_time - f64::sqrt(race_time * race_time - 4.0 * record_distance)) / 2.0;
    (f64::ceil(f64::min(a, b)) as u64, f64::floor(f64::max(a, b)) as u64)
}

fn margin_of_error(min: u64, max:u64) -> u64 {
    max - min + 1
}

