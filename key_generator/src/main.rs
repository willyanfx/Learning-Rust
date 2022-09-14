pub mod constant;

fn main() {
    results(26.0, 13.0, 1, Unit::Hour);
}

#[derive(Debug)]
enum Unit {
    Hour,
    Second,
}

fn results(string_len: f32, length: f32, speed: i32, unit: Unit) {
    let probability = 0.01; // 1/100
    let speed_per_second = if let Unit::Hour = unit {
        speed as f32 / 3600.0
    } else {
        speed as f32
    };

    let random_bits = get_random_bits(string_len, length);
    let generate_for_collision = get_generate_for_collision(random_bits, probability);
    let time_to_collision = get_time_to_collision(generate_for_collision, speed_per_second);
    format_duration(time_to_collision);
}

fn get_random_bits(string_length: f32, size: f32) -> f32 {
    //  natural logarithm of 2, approximately 0.693
    size * (string_length.ln() / 0.6931471805599453)
}

fn get_generate_for_collision(random_bits: f32, probability: f32) -> f32 {
    // 0.010050335853501506
    f32::sqrt((2_f32 * f32::powf(2_f32, random_bits)) * (1_f32 / (1_f32 - probability)).ln())
}

fn get_time_to_collision(generate_for_collision: f32, speed_per_second: f32) -> f32 {
    (generate_for_collision / speed_per_second).floor()
}

fn format_duration(seconds: f32) {
    let current = seconds;
    // loop through the array
    // get the object
    // divide igual `/=` current for Object[num]
    // if not Object[index + 1] or current divided by  Object[index + 1].num < 1
    // // round current
    // // return round value + ending + if plurized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bits() {
        assert_eq!(get_random_bits(26.0, 13.0), 61.105713);
        assert_eq!(get_random_bits(46.0, 16.0), 88.37699);
    }
    #[test]
    fn test_generate_for_collision() {
        let probability = 0.01;
        assert_eq!(
            get_generate_for_collision(61.105713, probability),
            223321470.0
        );
        assert_eq!(
            get_generate_for_collision(88.37699, probability),
            2842280000000.0
        );
    }

    #[test]
    fn test_time_to_collision() {
        let an_hour = 0.0002777777778;
        let sec = 360_f32;
        assert_eq!(get_time_to_collision(223321470.0, an_hour), 803957291935.0);
        assert_eq!(get_time_to_collision(2842280000000.0, sec), 7895222222.0);
    }
}
