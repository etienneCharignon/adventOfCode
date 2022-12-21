mod inputs;
use inputs::calculate;

fn calculate_example(monkey: &str) -> i64 {
    match monkey {
        "root" => { calculate_example("pppw") + calculate_example("sjmn") },
        "dbpl" => { 5 },
        "cczh" => { calculate_example("sllz") + calculate_example("lgvd") },
        "zczc" => { 2 },
        "ptdq" => { calculate_example("humn") - calculate_example("dvpt") },
        "dvpt" => { 3 },
        "lfqf" => { 4 },
        "humn" => { 5 },
        "ljgn" => { 2 },
        "sjmn" => { calculate_example("drzm") * calculate_example("dbpl") },
        "sllz" => { 4 },
        "pppw" => { calculate_example("cczh") / calculate_example("lfqf") },
        "lgvd" => { calculate_example("ljgn") * calculate_example("ptdq") },
        "drzm" => { calculate_example("hmdt") - calculate_example("zczc") },
        "hmdt" => { 32 },
        _ => panic!("monkey unknown {}", monkey)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve() {
        assert_eq!(calculate_example("root"), 152);
        assert_eq!(calculate("root"), 78342931359552);
    }
}
