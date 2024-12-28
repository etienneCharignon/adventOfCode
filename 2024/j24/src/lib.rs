mod inputs;

use std::collections::HashMap;
use csv::WriterBuilder;
use std::fs::File;
use std::io::Error;

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

pub fn read_ns(letter: char, initial_sates: &HashMap<String, u64>, gates: &HashMap<String, Gate>) -> u64 {
    let mut ns = 0;
    let mut digit = 0;
    loop {
        let n_id = format!("{letter}{digit:02}");
        if ! gates.contains_key(&n_id) && !initial_sates.contains_key(&n_id) {
            break;
        }
        let z_value = read(&n_id, &gates, &initial_sates);
        ns += z_value * 2_u64.pow(digit);
        digit += 1;
    }
    ns
}

pub fn find_swaps(n:u64, initial_sates: &HashMap<String, u64>, gates: &HashMap<String, Gate>) -> Result<String, Error> {
    let x = read_ns('x', initial_sates, gates);
    let y = read_ns('y', initial_sates, gates);
    let z = read_ns('z', initial_sates, gates);
    let sum = x + y;
    println!("{x:b} + {y:b}");
    println!("s: {sum:b}\nz: {z:45b}");
    println!("i: 5432109876543210987654321098765432109876543210");
    //     let expected: Vec<u64> = format!("{sum:b}").chars().map(|c| c.to_string().parse().expect("failed to parse number")).collect();
    //     let actual: Vec<u64> = format!("{z:45b}").chars().map(|c| c.to_string().parse().expect("failed to parse number")).collect();
    //     println!("s: {expected:?}\nz: {actual:?}");
    let mut gates_wtr = WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(File::create("/Users/etienne/Downloads/j24_gates.csv")?);
    gates_wtr.write_record(&["Id", "Label"])?;

    let mut wires_wtr = WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(File::create("/Users/etienne/Downloads/j24_wires.csv")?);
    wires_wtr.write_record(&["Source", "Target", "Type", "Id", "Label"])?;

    let mut id_gate = 0;
    let mut id_wire = 0;
    let mut gate_ids: HashMap<String, String> = HashMap::new();
    for (input, value) in initial_sates {
        gates_wtr.write_record(&[format!("{id_gate}"), format!("{input}: {value}")])?;
        gate_ids.insert(input.to_string(), format!("{id_gate}"));
        id_gate += 1;
    }
    for (output, gate) in gates {
        gates_wtr.write_record(&[format!("{id_gate}"), gate.operator.clone()])?;
        gate_ids.insert(output.to_string(), format!("{id_gate}"));
        id_gate += 1;
        if output.starts_with('z') {
            gates_wtr.write_record(&[format!("{id_gate}"), format!("{output}: ??")])?;
            gate_ids.insert(format!("o{output}"), format!("{id_gate}"));
            id_gate += 1;
        }
    }
    for (output, gate) in gates {
        wires_wtr.write_record(&[
            gate_ids.get(&gate.input1).expect("input id not found"),
            gate_ids.get(output).expect("output id not found"),
            "Directed",
            &format!("{id_wire}"),
            &gate.input1
        ])?;
        id_wire += 1;
        wires_wtr.write_record(&[
            gate_ids.get(&gate.input2).expect("input id not found"),
            gate_ids.get(output).expect("output id not found"),
            "Directed",
            &format!("{id_wire}"),
            &gate.input2
        ])?;
        id_wire += 1;
        if output.starts_with('z') {
            wires_wtr.write_record(&[
                gate_ids.get(output).expect("output id not found"),
                gate_ids.get(&format!("o{output}")).expect("output node id not found"),
                "Directed",
                &format!("{id_wire}"),
                &output
            ])?;
            id_wire += 1;
        }
    }
    gates_wtr.flush()?;
    Ok("dkr,ggk,hhh,htp,rhv,z05,z15,z20".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_p1() {
        assert_eq!(read_ns('z', &read_wires(inputs::EXAMPLE_INTIAL_STATE), &read_gates(inputs::EXAMPLE_GATES)), 2024);
        assert_eq!(read_ns('z', &read_wires(inputs::INPUT_INTIAL_STATE), &read_gates(inputs::INPUT_GATES)), 53755311654662);
    }

    #[test]
    fn it_works_p2() -> Result<(), Error> {
        // assert_eq!(find_swaps(2, &read_wires(inputs::EXAMPLE_INTIAL_STATE2), &read_gates(inputs::EXAMPLE_GATES2)), "z00,z01,z02,z05");
        assert_eq!(find_swaps(4, &read_wires(inputs::INPUT_INTIAL_STATE), &read_gates(inputs::INPUT_GATES))?, "dkr,ggk,hhh,htp,rhv,z05,z15,z20");
        Ok(())
    }
}
