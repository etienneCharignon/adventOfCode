mod inputs;
use std::collections::HashMap;

pub fn ppcm(a: usize, b:usize) -> usize {
    let mut c = a;
    let mut d = b;

    while c != d {
        if c > d {
            d = d+b;
        }
        else {
            c = c+a;
        }
    }
    c
}

pub fn count_step(map: &str, network: HashMap<&str, Vec<&'static str>>) -> usize {
    let mut current_node = "AAA";
    let directions: Vec<_> = map.chars().collect();
    let directions_length = directions.iter().count();
    for i in 0..usize::MAX {
        if current_node == "ZZZ" {
            return i
        }
        let direction = match directions[i % directions_length] {
            'L' => 0,
            _ => 1
        };
        current_node = network[current_node][direction];
    }
    0
}


pub fn count_node_step2(start_node: &str, map: &str, network: &HashMap<&str, Vec<&'static str>>) -> usize {
    let mut current_node = start_node;
    let directions: Vec<_> = map.chars().collect();
    let directions_length = directions.iter().count();
    for i in 0..usize::MAX {
        if current_node.chars().nth(2).unwrap() == 'Z' {
            return i
        }
        let direction = match directions[i % directions_length] {
            'L' => 0,
            _ => 1
        };
        current_node = network[current_node][direction];
    }
    0
}

pub fn count_step2(map: &str, network: HashMap<&str, Vec<&'static str>>) -> usize {
    let mut current_nodes: Vec<&str> = network.keys().filter(|node| node.chars().nth(2).unwrap() == 'A').map(|node| *node).collect();
    println!("{:?}", current_nodes);
    let distances: Vec<usize> = current_nodes.iter().map(|node| count_node_step2(node, map, &network)).collect();
    println!("{:?}", distances);
    distances.iter().fold(1, |solus, v| ppcm(solus, *v))
    /*
    let directions: Vec<_> = map.chars().collect();
    let directions_length = directions.iter().count();
    for i in 0..usize::MAX {
        if current_nodes.iter().all(|node| node.chars().nth(2).unwrap() == 'Z') {
            return i;
        }
        let direction = match directions[i % directions_length] {
            'L' => 0,
            _ => 1
        };
        let mut new_nodes = Vec::<&str>::new(); 
        for node in current_nodes {
            new_nodes.push(network[node][direction]);
        }
        current_nodes = new_nodes;
    }
    0
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example() {
        assert_eq!(count_step(inputs::EXAMPLE_MAP, inputs::example_network()), 2);
    }

    #[test]
    fn it_solve_example3() {
        assert_eq!(count_step2("LR", inputs::example3_network()), 6);
    }

    #[test]
    fn it_solve_example2() {
        assert_eq!(count_step(inputs::EXAMPLE2_MAP, inputs::example2_network()), 6);
    }

    #[test]
    fn it_solve_input() {
        assert_eq!(count_step(inputs::INPUT_MAP, inputs::input_network()), 18113);
    }

    #[test]
    fn it_solve_input_p2() {
        assert_eq!(count_step2(inputs::INPUT_MAP, inputs::input_network()), 12315788159977);
    }
}