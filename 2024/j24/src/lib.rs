mod inputs;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(.{3}) (.{2,3}) (.{3}) -> (.{3})").unwrap();
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Gate {
   input1: String,
   input2: String,
   operator: String,
   operate: fn(u64, u64) -> u64,
}

pub fn read_wires(input: &str) -> HashMap<String, u64> {
    let mut wires = HashMap::new();
    for wire in input.split('\n') {
        let parts: Vec<&str> = wire.split(": ").collect();
        wires.insert(parts[0].to_string(), parts[1].parse().expect("failed to parse number"));
    }
    wires
}

pub fn read_gates(input: &str) -> HashMap<String, Gate> {
    let mut gates = HashMap::new();
    for groups in RE.captures_iter(input) {
        let operator_string: &str = &groups[2];
        let gate = Gate {
            input1: groups[1].to_string(),
            operator: operator_string.to_string(),
            operate: match operator_string {
                "OR" => |input1, input2| input1 | input2,
                "AND" => |input1, input2| input1 & input2,
                "XOR" => |input1, input2| input1 ^ input2,
                _ => panic!("Unknown operator"),
            },
            input2: groups[3].to_string()
        };
        gates.insert(groups[4].to_string(), gate);
    }
    gates
}

pub fn read(wire: &String, gates: &HashMap<String, Gate>, states: &HashMap<String, u64>) -> u64 {
    if let Some(&value) = states.get(wire) {
        return value;
    }
    match gates.get(wire) {
        Some(gate) => {
            println!("{wire} : {gate:?}");
            let input1 = read(&gate.input1, gates, states);
            let input2 = read(&gate.input2, gates, states);
            (gate.operate)(input1, input2)
        },
        _ => panic!("wire unknown {wire}")
    }
}

pub fn read_zs(initial_sates: HashMap<String, u64>, gates: HashMap<String, Gate>) -> u64 {
    let mut zs = 0;
    let mut digit = 0;
    loop {
        let z_id = format!("z{digit:02}");
        if ! gates.contains_key(&z_id) {
            break;
        }
        let z_value = read(&z_id, &gates, &initial_sates);
        zs += z_value * 2_u64.pow(digit);
        digit += 1;
    }
    zs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_zs(read_wires(inputs::EXAMPLE_INTIAL_STATE), read_gates(inputs::EXAMPLE_GATES)), 2024);
        assert_eq!(read_zs(read_wires(inputs::INPUT_INTIAL_STATE), read_gates(inputs::INPUT_GATES)), 53755311654662);
    }
}
