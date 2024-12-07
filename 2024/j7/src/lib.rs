mod inputs;

pub fn terminate_by(value: i64, last_digit: i64) -> bool {
    let len = format!("{last_digit}").len() as u32;
    (value-last_digit) % (10_i64.pow(len)) == 0
}

pub fn remove_last(value: i64, last_digit: i64) -> i64 {
    let len = format!("{last_digit}").len() as u32;
    (value-last_digit) / (10_i64.pow(len)) 
}

pub fn check_equation(test_value: i64, terms: &[i64]) -> bool {
    if terms.len() == 2 {
        test_value == terms[0] * terms[1] || test_value == terms[0] + terms[1] ||
            format!("{test_value}") == format!("{}{}", terms[0], terms[1])
    }
    else {
        let previous = &terms[..terms.len() -1];
        let last_term = terms[terms.len()-1];
        check_equation(test_value - last_term, previous) || 
            test_value % last_term == 0 && check_equation(test_value / last_term, previous) ||
            terminate_by(test_value, last_term) && check_equation(remove_last(test_value, last_term), previous)
    }
}

pub fn total_calibration(input: &str) -> i64 {
    let mut total = 0;
    for line in input.split('\n') {
        let mut elements: Vec<&str> = line.split_whitespace().collect();
        let mut test_value_str = String::from(elements.remove(0));
        test_value_str.pop();
        let test_value: i64 = test_value_str.parse().expect("Failed to parse number");
        let terms: Vec<i64> = elements.iter().map(|s| s.parse().expect("Failed to parse number")).collect();
        if check_equation(test_value, &terms) {
            total += test_value;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_check_terminante_by() {
            assert_eq!(terminate_by(486, 6), true);
            assert_eq!(terminate_by(4866, 66), true);
            assert_eq!(terminate_by(4866, 65), false);
            assert_eq!(terminate_by(4856, 66), false);
    }

    #[test]
    fn it_check_remove_last() {
            assert_eq!(remove_last(486, 6), 48);
            assert_eq!(remove_last(4866, 66), 48);
    }

    #[test]
    fn it_works() {
        assert_eq!(total_calibration(inputs::EXAMPLE), 11387);
        assert_eq!(total_calibration(inputs::INPUT), 165278151522644);
    }
}
