mod inputs;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CONDITIONRE: Regex = Regex::new(r"(.)(<|>)(\d*)").unwrap();
    static ref WORKFLOWRE: Regex = Regex::new(r"(\S*)\{(\S*)\}").unwrap();
    static ref PARTRE: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Part([usize; 4]);

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Condition {
    categorie: usize,
    greaterthan: bool,
    rate: usize,
}

 impl Condition {
    fn new(condition: &str) -> Condition {
        let (_, [categorie, condition, rate]): (&str, [&str; 3]) = CONDITIONRE.captures(condition).unwrap().extract();
        Condition {
            categorie: match categorie { "x" => 0, "m" => 1, "a" => 2, "s" => 3, _ => panic!("not a categorie") },
            greaterthan: match condition { ">" => true, "<" => false, _=> panic!("not a condition") },
            rate: rate.parse::<usize>().unwrap()
        }
    }
 }

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Rull {
    condition: Condition,
    dest: String,
}

 impl Rull {
    fn new(rull: &str) -> Rull {
        // println!("{rull}");
        let split: Vec<&str> = rull.split(":").collect();
        if split.len() == 2 {
            Rull { condition: Condition::new(split[0]) , dest: String::from(split[1]) }
        }
        else {
            Rull {
                condition: Condition { categorie: 0, greaterthan: true, rate: 0 }, 
                dest: String::from(rull)
            }
        }
    }
 }

pub fn read_rulls(rulls: &str) -> HashMap<&str, Vec<Rull>> {
    let mut workflows = HashMap::new();
    for workflow in rulls.lines() {
        let (_, [name, rull]): (&str, [&str; 2]) = WORKFLOWRE.captures(workflow).unwrap().extract();
        workflows.insert(name, rull.split(",").map(|r| Rull::new(r)).collect());
    }
    workflows
}

pub fn read_parts(parts: &str) -> Vec<Part> {
    parts.lines().map(|l| {
        let rates: Vec<usize> = PARTRE.find_iter(l).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
        Part([rates[0],rates[1],rates[2],rates[3]])
    }).collect()
}

pub fn is_accepted(rull_name: &str, workflows: &HashMap<&str, Vec<Rull>>, part: &Part) -> bool {
    // println!("{part:?} : {workflows:?}");
    let rulls = workflows.get(rull_name).unwrap();
    for rull in rulls {
        let condition = rull.condition;
        if condition.greaterthan && part.0[condition.categorie] > condition.rate ||
           !condition.greaterthan && part.0[condition.categorie] < condition.rate {
            match rull.dest.as_str() {
                "A" => return true,
                "R" => return false,
                part_name => return is_accepted(part_name, workflows, part),
            };
        }
    }
    true
}

pub fn solve(rulls: &str, parts: &str) -> usize {
    read_parts(parts).iter().filter(|p| is_accepted("in", &read_rulls(rulls), *p)).map(|p| p.0.iter().sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(inputs::EXAMPLE_RULLS, inputs::EXAMPLE_PARTS), 19114);
        assert_eq!(solve(inputs::INPUT_RULLS, inputs::INPUT_PARTS), 348378);
    }
}
