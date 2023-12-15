mod inputs;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Lense<'a>
{
    pub label: &'a str,
    pub focal: usize
}

use multimap::MultiMap;

pub fn hash(string: &str) -> usize {
    string.chars().fold(0, |hash, c| {
        let mut new_hash = hash + c as usize;
        new_hash *= 17;
        new_hash %= 256;
        new_hash
    })
}

pub fn solve_p1(input: &str) -> usize {
    input.split(",").map(|c| hash(c)).sum()
}

pub fn focusing_power(input: &str) -> usize {
    let mut boxes: MultiMap<usize, Lense> = MultiMap::new();
    for step in input.split(",") {
        let splitequal: Vec<&str> = step.split('=').collect();
        if splitequal.len() == 2 {
            println!("{splitequal:?}");
            let box_number = hash(splitequal[0]);
            let mut found = false;
            for (n, list) in boxes.iter_all_mut() {
                if box_number == *n {
                    for lense in list {
                        if lense.label == splitequal[0] {
                            lense.focal = splitequal[1].parse::<usize>().unwrap();
                            found = true;
                            break;
                        }
                    }
                    break;
                }
            }
            if ! found {
                boxes.insert(box_number, Lense { label: splitequal[0], focal: splitequal[1].parse::<usize>().unwrap() });
            }
        }
        else {
            let label = step.split('-').collect::<Vec<_>>()[0];
            println!("{label} => -");
            let box_number = hash(label);
            for (n, list) in boxes.iter_all_mut() {
                if box_number == *n {
                    list.retain(|l| label != l.label);
                    break;
                }
            }
        }
        //println!("{boxes:?}");
    }
    boxes.iter_all().map(|(n, lenses)| {
        lenses.iter().enumerate().map(|(i, lense)| (n + 1) * (i + 1) * lense.focal).sum::<usize>()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn it_solve_p1() {
        assert_eq!(solve_p1(inputs::EXAMPLE), 1320);
        assert_eq!(solve_p1(inputs::INPUT), 519041);
    }

    #[test]
    fn it_solve_p2() {
        assert_eq!(focusing_power(inputs::EXAMPLE), 145);
        assert_eq!(focusing_power(inputs::INPUT), 260530);
    }
}
