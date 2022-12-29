mod inputs;
#[allow(unused_imports)]
use std::cmp;

fn decode(number: &str) -> i64 {
    number.chars().rev().enumerate().map(|(i, c)| {
        match c {
            '2' => { 2 * 5_i64.pow(i as u32) },
            '1' => { 1 * 5_i64.pow(i as u32) },
            '0' => { 0 },
            '-' => { -1 * 5_i64.pow(i as u32) },
            '=' => { -2 * 5_i64.pow(i as u32) },
            _ => panic!("unkwnow char {}", c)
        }
    } ).sum()
}

#[allow(dead_code)]
fn sum(input: &str) -> i64 {
    input.lines().map(|l| decode(l)).sum()
}

fn nums(a: &str) -> Vec<i32> {
    a.chars().rev().map(|c| {
        match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unkwnow char {}", c)
        }
    }).collect()
}

fn sum_snafu(a: &str, b: &str) -> String {
    let num_b = nums(b);
    let mut result = Vec::from_iter(nums(a));
    while result.len() <= b.len() {
        result.push(0);
    }
    for (i, b) in num_b.iter().enumerate() {
        result[i] = result[i] + b;
        if result[i] >= 3 {
            result[i] -= 5;
            result[i + 1] += 1;
        }
        if result[i] <= -3 {
            result[i] += 5;
            result[i + 1] -= 1;
        }
    }
    for i in num_b.len()..result.len() {
        if result[i] >= 3 {
            result[i] -= 5;
            result[i + 1] += 1;
        }
        if result[i] <= -3 {
            result[i] += 5;
            result[i + 1] -= 1;
        } 
    }
    
    result.iter().rev().map(|n| {
        match n {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => panic!("unkwnow char {}, {} + {}\n{:?}", n, a, b, result)
        }
    }).collect::<Vec<&str>>().join("")
}

#[allow(dead_code)]
fn sum_snafus(input: &str) -> String {
    let mut sum: String = String::from("");
    for l in input.lines() {
        sum = sum_snafu(&sum, l);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decode() {
        assert_eq!(decode("0"), 0);
        assert_eq!(decode("1"), 1);
        assert_eq!(decode("2"), 2);
        assert_eq!(decode("-"), -1);
        assert_eq!(decode("="), -2);
        assert_eq!(decode("20"), 10);
    }

    #[test]
    fn it_sum() {
        assert_eq!(sum(inputs::EXAMPLE), 4890);
        assert_eq!(sum(inputs::INPUT), 30638862852576);
    }

    #[test]
    fn it_sum_snafu() {
        assert_eq!(sum_snafu("1", "0"), "01");
        assert_eq!(sum_snafu("12", "00"), "012");
        assert_eq!(sum_snafu("1", "-"), "00");
        assert_eq!(sum_snafu("1", "="), "0-");
        assert_eq!(sum_snafu("1", "2"), "1=");
        assert_eq!(sum_snafu("2", "2"), "1-");
        assert_eq!(sum_snafu("-", "-"), "0=");
        assert_eq!(sum_snafu("-", "="), "-2");
        assert_eq!(sum_snafu("11", "="), "1-");
        assert_eq!(sum_snafu("1", "11"), "012");
        assert_eq!(sum_snafu("01==02=21212022=021-", "22-1-1=0--0="), "01==02-=--1=0=0=-102");
    }

    #[test]
    fn it_sum_snafus() {
        assert_eq!(sum_snafus(inputs::EXAMPLE), "02=-1=0");
        assert_eq!(sum_snafus(inputs::INPUT), "02=01-0-2-0=-0==-1=01");
    }
}
