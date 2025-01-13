pub fn get_harmonic(value: &str, harmonic: f64) -> String {
    // if its a css variable
    if value.starts_with("--") {
        return format!("var({})", value);
    }

    if value == "none" {
        return "0.0".to_string();
    }

    if let Ok(x) = value.parse::<f64>() {
        let value =  harmonic.powf(x).to_string();
        return format!("{value:.5}rem").to_string();
    };

    // if the value is not a unit less number
    // so not a harmonic number
    // we return it as it is
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the function into scope for the tests

    // Use 1.618 as the harmonic value for all tests
    const HARMONIC: f64 = 1.618;

    #[test]
    fn test_numeric_values() {
        // Test harmonic calculation for numeric values with harmonic = 1.618
        assert_eq!(get_harmonic("2", HARMONIC), "2.617rem"); // 1.618^2
        assert_eq!(get_harmonic("2.5", HARMONIC), "3.330rem"); // 1.618^2.5
    }

    #[test]
    fn test_none_value() {
        // Test for the "none" value, which should return "0.0"
        assert_eq!(get_harmonic("none", HARMONIC), "0.0");
    }

    #[test]
    fn test_css_variable() {
        // Test for CSS variable values, they should be returned wrapped in var()
        assert_eq!(get_harmonic("--main-color", HARMONIC), "var(--main-color)");
        assert_eq!(get_harmonic("--font-size", HARMONIC), "var(--font-size)");
    }

    #[test]
    fn test_non_numeric_values() {
        // Test for non-numeric values (with units or other characters)
        assert_eq!(get_harmonic("16px", HARMONIC), "16px");
        assert_eq!(get_harmonic("1rem", HARMONIC), "1rem");
        assert_eq!(get_harmonic("invalid", HARMONIC), "invalid");
    }

    #[test]
    fn test_empty_string() {
        // Test for empty string, should return an empty string as is
        assert_eq!(get_harmonic("", HARMONIC), "");
    }
}

