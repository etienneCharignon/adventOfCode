mod inputs;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;

lazy_static! {
    static ref CONDITIONRE: Regex = Regex::new(r"(.)(<|>)(\d*)").unwrap();
    static ref WORKFLOWRE: Regex = Regex::new(r"(\S*)\{(\S*)\}").unwrap();
    static ref PARTRE: Regex = Regex::new(r"(\d+)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Part([usize; 4]);
/*
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct PartRange([Range<Idx>; 4]);
*/

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
        if condition.greaterthan && part.0[condition.categorie % 4] > condition.rate ||
           !condition.greaterthan && part.0[condition.categorie % 4] < condition.rate {
            match rull.dest.as_str() {
                "A" => return true,
                "R" => return false,
                part_name => return is_accepted(part_name, workflows, part),
            };
        }
    }
    true
}

pub fn intersection(i1: (usize, usize), i2: (usize, usize)) -> Option<(usize, usize)> { 
    if i2.0 > i1.1 || i1.0 > i2.1 {
        None
    }
    else {
        Some((cmp::max(i1.0, i2.0), cmp::min(i1.1, i2.1)))
    }
}

/*
pub fn compute_accepted_range(rull_name: &str, range: [(usize, usize); 4], workflows: &HashMap<&str, Vec<Rull>>) -> [(usize, usize); 4] {
    let rulls = workflows.get(rull_name).unwrap();
    let mut accepted_range = [(1, 0), (1, 0), (1, 0), (1, 0)];
    for rull in rulls {
        let condition = rull.condition;

        let mut new_range = range;
        if condition.greaterthan {
            left_range[contidion.categorie] = intersection(new_range[condition.categorie], (1, condition.rate))
            match rull.dest.as_str() {
                "A" => union_catetories(accepted_range, left_range),
                "R" => intersection_catetories(range, left_range),
                part_name => return is_accepted(part_name, workflows, part),
            };
            right_range[contidion.categorie] = intersection(new_range[condition.categorie], (condition.rate + 1, 4000))
        }
        else {
            new_range[contidion.categorie] = intersection(new_range[condition.categorie], (1, condition.rate - 1))
        }
        match rull.dest.as_str() {
            "A" => merge(new_range, accepted_range),
            "R" => return false,
            part_name => return is_accepted(part_name, workflows, part),
        };
    }
    accepted_range
}

pub fn count_accepted_categorie(categorie: usize, workflow_name: &str, range: (usize, usize), workflows: &HashMap<&str, Vec<Rull>>) -> usize {
    // println!("{workflows:?}");
    let wf = workflows.get(workflow_name).unwrap();
    let mut count = 0;
    let mut living_range = Some(range);
    for rull in wf {
        // println!("{rull:?} {living_range:?}");
        if living_range == None {
            break;
        }
        let mut new_range = living_range.unwrap();
        let condition = rull.condition;
        if categorie == condition.categorie {
            let new_intersection = if condition.greaterthan { intersection(living_range.unwrap(), (condition.rate + 1, 4000)) }
                  else { intersection(living_range.unwrap(), (1, condition.rate - 1)) };
            living_range = if condition.greaterthan { intersection(living_range.unwrap(), (1, condition.rate)) }
                           else { intersection(living_range.unwrap(), (condition.rate, 4000)) };
            match new_intersection {
                    None => continue,
                    Some(intersection) => new_range = intersection,
                  }
        }
        match rull.dest.as_str() {
            "A" => count += new_range.1 - new_range.0 + 1,
            "R" => continue,
            dest => count += count_accepted_categorie(categorie, dest, new_range, workflows),
        };
    }
    count
}
*/

pub fn size(interval: &(usize, usize)) -> usize {
    if interval.1 < interval.0 { 0 } else { interval.1 - interval.0 + 1 }
}

pub fn count_accepted(workflow_name: &str, range: [(usize, usize);4], workflows: &HashMap<&str, Vec<Rull>>) -> usize {
    let wf = workflows.get(workflow_name).unwrap();
    let mut count = 0;
    let mut living_range = range;
    for rull in wf {
        let condition = rull.condition;

        let match_intersection = if condition.greaterthan { intersection(living_range[condition.categorie], (condition.rate + 1, 4000)) }
                else { intersection(living_range[condition.categorie], (1, condition.rate - 1)) };
        let mut new_range = living_range;
        match match_intersection {
            None => continue,
            Some(intersection) => new_range[condition.categorie] = intersection,
        };
        count += match rull.dest.as_str() {
            "A" => new_range.iter().map(|interval| size(interval)).product::<usize>(),
            "R" => 0,
            dest => count_accepted(dest, new_range, workflows),
        };

        let left_intersection = if condition.greaterthan { intersection(living_range[condition.categorie], (1, condition.rate)) }
                        else { intersection(living_range[condition.categorie], (condition.rate, 4000)) };
        match left_intersection {
            None => break,
            Some(intersection) => living_range[condition.categorie] = intersection
        }
    }
    count
}

pub fn solve(input_rulls: &str, parts: &str) -> usize {
    let rulls = read_rulls(input_rulls);
    read_parts(parts).iter().filter(|p| is_accepted("in", &rulls, *p)).map(|p| p.0.iter().sum::<usize>()).sum()
}

pub fn solvep2(input: &str) -> usize {
    let rulls = read_rulls(input);
    // (0..4).into_iter().map(|categorie| count_accepted_categorie(categorie, "in", (1, 4000), &rulls)).product()
    count_accepted("in", [(1, 4000), (1, 4000), (1, 4000), (1, 4000)], &rulls) 
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_count_accepted() {
        assert_eq!(count_accepted("in", [(11, 1000), (1, 2000), (1, 1), (1, 1)], &read_rulls("in{x<3:px,m<3:A}\npx{A}")), 2*990);
        assert_eq!(count_accepted("in", [(1, 1000), (1, 2000), (1, 1), (1, 1)], &read_rulls("in{x<3:px,R}\npx{m<3:A,A}")), 2*2 + 2*1998);
        assert_eq!(count_accepted("in", [(1, 1000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:px,R}\npx{A}")), 1);
        assert_eq!(count_accepted("in", [(1, 1000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:px,A}\npx{R}")), 999);
        assert_eq!(count_accepted("in", [(1, 1000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:px}\npx{A}")), 1);
        assert_eq!(count_accepted("in", [(1, 1000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:A,A}")), 1000);
        assert_eq!(count_accepted("in", [(10, 2000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:A}")), 0);
        assert_eq!(count_accepted("in", [(1, 2000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x<2:A}")), 1);
        assert_eq!(count_accepted("in", [(1, 1), (1, 2000), (1, 1), (1, 1)], &read_rulls("in{m>1:A}")), 1999);
        assert_eq!(count_accepted("in", [(1, 2000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x>1:A,R}")), 1999);
        assert_eq!(count_accepted("in", [(1, 1), (1, 2000), (1, 1), (1, 1)], &read_rulls("in{m>1:A,A}")), 2000);
        assert_eq!(count_accepted("in", [(1, 2000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{x>1:A}")), 1999);
        assert_eq!(count_accepted("in", [(1, 4000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{A}")), 4000);
        assert_eq!(count_accepted("in", [(1, 4000), (1, 1), (1, 1), (1, 1)], &read_rulls("in{R}")), 0);
        assert_eq!(count_accepted("in", [(1, 1), (1, 4000), (1, 1), (1, 1)], &read_rulls("in{A}")), 4000);
    }

    #[test]
    fn it_solve_part1() {
        assert_eq!(solve(inputs::EXAMPLE_RULLS, inputs::EXAMPLE_PARTS), 19114);
        assert_eq!(solve(inputs::INPUT_RULLS, inputs::INPUT_PARTS), 348378);
    }

    #[test]
    fn it_solve_part2() {
        assert_eq!(solvep2(inputs::EXAMPLE_RULLS), 167409079868000);
        assert_eq!(solvep2(inputs::INPUT_RULLS), 121158073425385);
    }
}