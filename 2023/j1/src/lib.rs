mod inputs;

pub fn extract_value(line: &str) -> u32 {
    let lindex = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let rindex = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let lcalibration = line.chars().nth(lindex).unwrap().to_string();
    let rcalibration = line.chars().nth(rindex).unwrap().to_string();
    (lcalibration + &rcalibration).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_value() {
        assert_eq!(extract_value("1abc2"), 12);
        assert_eq!(extract_value("pqr3stu8vwx"), 38);
        assert_eq!(extract_value("treb7uchet"), 77);
    }

    #[test]
    fn it_solve() {
        let lines = inputs::INPUT.split('\n');
        let calibration_values: Vec<u32> = lines.map(extract_value).collect();
        assert_eq!(calibration_values.iter().sum::<u32>(), 0);
    }
}
