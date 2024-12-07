mod inputs;


pub fn check_equation(test_value: i64, terms: &[i64]) -> bool {
    if terms.len() == 2 {
        test_value == terms[0] * terms[1] || test_value == terms[0] + terms[1]
    }
    else {
        let previous = &terms[..terms.len() -1];
        check_equation(test_value - terms[terms.len()-1], previous) || 
            test_value % terms[terms.len()-1] == 0 && check_equation(test_value / terms[terms.len() - 1], previous)
    }
}

pub fn total_calibration(input: &str) -> i64 {
    let mut total = 0;
    for line in input.split('\n') {
        let mut elements: Vec<&str> = line.split_whitespace().collect();
        let mut test_value_str = String::from(elements.remove(0));
        test_value_str.pop();
        println!("{test_value_str}: {:?}", elements);
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
    fn it_works() {
        assert_eq!(total_calibration(inputs::EXAMPLE), 3749);
        assert_eq!(total_calibration(inputs::INPUT), 3749);
    }
}
