mod inputs;
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example() {
        assert_eq!(count_step(inputs::EXAMPLE_MAP, inputs::example_network()), 2);
    }

    #[test]
    fn it_solve_example2() {
        assert_eq!(count_step(inputs::EXAMPLE2_MAP, inputs::example2_network()), 6);
    }

    #[test]
    fn it_solve_input() {
        assert_eq!(count_step(inputs::INPUT_MAP, inputs::input_network()), 18113);
    }
}