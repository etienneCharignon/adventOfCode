mod inputs;
use inputs::calculate;

fn calculate_example(yelling: i64, monkey: &str) -> i64 {
    match monkey {
        "root" => { calculate_example(yelling, "pppw") + calculate_example(yelling, "sjmn") },
        "dbpl" => { 5 },
        "cczh" => { calculate_example(yelling, "sllz") + calculate_example(yelling, "lgvd") },
        "zczc" => { 2 },
        "ptdq" => { calculate_example(yelling, "humn") - calculate_example(yelling, "dvpt") },
        "dvpt" => { 3 },
        "lfqf" => { 4 },
        "humn" => { yelling },
        "ljgn" => { 2 },
        "sjmn" => { calculate_example(yelling, "drzm") * calculate_example(yelling, "dbpl") },
        "sllz" => { 4 },
        "pppw" => { calculate_example(yelling, "cczh") / calculate_example(yelling, "lfqf") },
        "lgvd" => { calculate_example(yelling, "ljgn") * calculate_example(yelling, "ptdq") },
        "drzm" => { calculate_example(yelling, "hmdt") - calculate_example(yelling, "zczc") },
        "hmdt" => { 32 },
        _ => panic!("monkey unknown {}", monkey)
    }
}

pub fn find_yell(m1: &str, m2: &str) -> i64 {
    let m2_score = calculate(1, m2);
    let mut a = 550000000000;
    println!("a : {}", calculate(a, m1));
    let mut b = 5550000000000;
    println!("b : {}", calculate(b, m1));
    println!("b - a : {}", b - a);
    let mut m = 0;
    while b - a >= 1  {
        m = (a + b) / 2;
        let m1_score = calculate(m, m1);
        println!("m {}, {}, {}, {}", m, m1_score, m2_score, m1_score > m2_score);
        if  m1_score == m2_score { return m; }
        if  m1_score > m2_score {
            a = m;
        }
        else {
            b = m;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve() {
        // assert_eq!(find_yell("pppw", "sjmn"), 301);
        //assert_eq!(calculate(1000000000000, "lvvf"), 23440423968672);
        assert_eq!(find_yell("lvvf", "rqgq"), 3296135418821);
        assert_eq!(calculate(3296135418821 - 1, "root"), 0);
        //assert_eq!(calculate("root"), 78342931359552);
    }
}
