pub fn convert_celsius_to_fahrenheit(temperature: f32) -> f32 {
    return format_with_two_decimal_places(temperature * 1.8 + 32.0);
}

pub fn convert_fahrenheit_to_celsius(temperature: f32) -> f32 {
    return format_with_two_decimal_places((temperature - 32.0) / 1.8);
}

pub fn format_with_two_decimal_places(number: f32) -> f32 {
    return f32::trunc(number * 100.0) / 100.0;
}

#[cfg(test)]
mod temperature_converter_tests {
    use super::*;

    #[test]
    fn zero_celsius_equals_thirty_two_fahrenheit() {
        assert_eq!(32.0, convert_celsius_to_fahrenheit(0.0));
    }

    #[test]
    fn twelve_celsius_equals_fifty_three_point_six_fahrenheit() {
        assert_eq!(53.6, convert_celsius_to_fahrenheit(12.0));
    }

    #[test]
    fn zero_fahrenheit_equals_minus_seventeen_point_seventy_seven_celsius() {
        assert_eq!(-17.77, convert_fahrenheit_to_celsius(0.0));
    }

    #[test]
    fn twelve_fahrenheit_equals_minus_eleven_point_eleven_celsius() {
        assert_eq!(-11.11, convert_fahrenheit_to_celsius(12.0));
    }
}
