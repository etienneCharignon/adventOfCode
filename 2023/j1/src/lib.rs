mod inputs;

use regex::Regex;

pub fn convert_match(m: regex::Match) -> &str {
    match m.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => m.as_str()
    }
}

pub fn extract_value(line: &str) -> u32 {
    let goodline = line
        .replace("nine", "9")
        .replace("eight", "8")
        .replace("two", "2")
        .replace("one", "1")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7");
    println!("{}", goodline);
    let lindex = goodline.find(|c: char| c.is_ascii_digit()).unwrap();
    let rindex = goodline.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let lcalibration = goodline.chars().nth(lindex).unwrap().to_string();
    let rcalibration = goodline.chars().nth(rindex).unwrap().to_string();
    (lcalibration + &rcalibration).parse().unwrap()
//    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").unwrap();
//    let numbers: Vec<&str> = re.find_iter(line).map(convert_match).collect();
//    println!("{:?}", numbers);
//    (numbers.first().unwrap().to_string() + &numbers.last().unwrap()).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_value() {
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
    #[ignore]
    fn it_solve() {
        let lines = inputs::INPUT.split('\n');
        assert_eq!(lines.clone().count(), 1000);
        let calibration_values: Vec<u32> = lines.map(extract_value).collect();
        println!("{:?}", calibration_values);
        assert_eq!(calibration_values.len(), 1000);
        assert_eq!(calibration_values.iter().sum::<u32>(), 55607);
    }
}
