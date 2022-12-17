mod inputs;
use regex::Regex;
use std::collections::HashMap;


#[allow(dead_code)]
fn read_input<'a>(input: &'a str, network: &mut HashMap<&'a str, Vec<&'a str>>, valves: &mut HashMap<&'a str, usize>) {
    let re = Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    for line in input.lines() {
        re.captures(line).and_then(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3));
            match groups {
                (Some(node), Some(rate), Some(nodes)) => {
                    let not_null_rate = rate.as_str().parse::<usize>().unwrap();
                    if not_null_rate > 0 { valves.insert(node.as_str(), not_null_rate); }
                    network.insert(node.as_str(), nodes.as_str().split(", ").collect::<Vec<&str>>())
                },
                _ => panic!("{} unparsable", line),
            }
        });
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_input() {

        let mut network: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut valves: HashMap<&str, usize> = HashMap::new();
        read_input(inputs::EXAMPLE, &mut network, &mut valves);
        assert_eq!(network, HashMap::from([ 
            ("AA", vec!["DD", "II", "BB"]),
            ("BB", vec!["CC", "AA"]),
            ("CC", vec!["DD", "BB"]),
            ("DD", vec!["CC", "AA", "EE"]),
            ("EE", vec!["FF", "DD"]),
            ("FF", vec!["EE", "GG"]),
            ("GG", vec!["FF", "HH"]),
            ("HH", vec!["GG"]),
            ("II", vec!["AA", "JJ"]),
            ("JJ", vec!["II"]),
        ]));
        assert_eq!(valves, HashMap::from([ 
            ("BB", 13),
            ("CC", 2),
            ("DD", 20),
            ("EE", 3),
            ("HH", 22),
            ("JJ", 21),
        ]));
    }
}
