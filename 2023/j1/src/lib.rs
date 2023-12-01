mod inputs;

use regex::Regex;

pub fn convert_match(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s
    }
}

pub fn extract_value(line: &str) -> u32 {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").unwrap();
    let er = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
    let first: &str = convert_match(re.find(line).unwrap().as_str());
    let last_capture = er.captures(line).unwrap();
    let last: &str = convert_match(&last_capture[1]);

    (first.to_string() + last).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_value() {
        assert_eq!(extract_value("five61oneightr"), 58);
        assert_eq!(extract_value("1abc2"), 12);
        assert_eq!(extract_value("pqr3stu8vwx"), 38);
        assert_eq!(extract_value("treb7uchet"), 77);
        assert_eq!(extract_value("two1nine"), 29);
    }

    #[test]
    fn it_solve_example1() {
        let lines = inputs::EXAMPLE1.split('\n');
        let calibration_values: Vec<u32> = lines.map(extract_value).collect();
        assert_eq!(calibration_values.iter().sum::<u32>(), 142);
    }

    #[test]
    fn it_solve_example() {
        let lines = inputs::EXAMPLE.split('\n');
        let calibration_values: Vec<u32> = lines.map(extract_value).collect();
        assert_eq!(calibration_values.iter().sum::<u32>(), 281);
    }

    #[test]
    fn it_solve() {
        let lines = inputs::INPUT.split('\n');
        assert_eq!(lines.clone().count(), 1000);
        let calibration_values: Vec<u32> = lines.map(extract_value).collect();
        assert_eq!(calibration_values.len(), 1000);
        assert_eq!(calibration_values.iter().sum::<u32>(), 55614);
    }
}
