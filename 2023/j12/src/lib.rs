mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;



lazy_static! {
    static ref RE: Regex = Regex::new(r"(\S+) (\S+)").unwrap();
}

/*
pub fn generate_combinations(springs: Vec<char>) -> Vec<&'static str> {
    vec!["#.#.###"]
}
*/

pub fn match_controle(line: &str, controls: &Vec<usize>) -> bool {
    let mut elements = Vec::<String>::new();
    for n in controls {
        let mut element = String::from(r"#{");
        element.push_str(&(n.to_string()));
        element.push_str(r"}");
        elements.push(element);
    }
    let mut re = String::from(r"^\.*");
    re.push_str(&elements.join(r"\.+"));
    re.push_str(r"\.*$");
    Regex::new(&re).unwrap().is_match(line) 
}

pub fn start_with(line: &[char], group: &String) -> bool {
    if line.len() < group.len() {
        return false;
    }
    for i in 0..group.len() {
        if line[i] != '?' && line[i] != '#' {
            return false;
        }
    }
    return true;
}

pub fn count(springs: &[char], groups: &[String], cache: &mut HashMap<String, usize>) -> usize {
    let mut key = String::from_iter(springs);
    key.push_str(&groups.join(","));
    let c = match cache.get(&key) {
        Some(c) => *c,
        None => {
            let mut c = 0;
            for i in 0..springs.len() {
                if start_with(&springs[i..], &groups[0]) {
                    let remain = &springs[i+groups[0].len()..];
                    if remain.len() == 0 {
                        if groups.len() == 1 {
                            c += 1;
                        }
                    }
                    else if remain[0] == '#' {
                    }
                    else if groups.len() == 1 {
                        if remain.iter().all(|c| *c != '#') {
                            c += 1;
                        }
                    }
                    else {
                        let remain = &springs[i+groups[0].len()+1..];
                        if remain.len() == 0 {
                            if groups.len() == 1 {
                                c += 1;
                            }
                        }
                        else {
                            c += count(remain, &groups[1..], cache);
                        }
                    }
                }
                if springs[i] == '#' {
                    break;
                }
            }
            cache.insert(key.clone(), c);
            c
        }
    };
    println!("{key} {c}");
    c
}
                    /*
                    let mut key = String::from_iter(remain);
                    key.push_str(&groups[1..].join(","));
                    c += match cache.get(&key) {
                        Some(c) => *c,
                        None => {
                            let sub_c = count(remain, &groups[1..], cache);
                            cache.insert(key, sub_c);
                            let mut keyp = String::from_iter(remain);
                            keyp.push_str(&groups[1..].join(","));
                            println!("{keyp:?} {sub_c}");
                            sub_c
                        }
                    };
                    */

pub fn read_groups(string: &str) -> Vec<String> {
    let controls: Vec<usize> = string.split(",").map(|n| n.parse().unwrap()).collect();
    controls.iter().map(|n| {
        let mut string = String::from("#");
        for _n in 1..*n {
            string.push_str("#");
        }
        string
    }).collect()
}

pub fn count_combination(line: &str) -> usize {
    let data: Vec<&str> = line.split_whitespace().collect();
    let springs: Vec<char> = data[0].chars().collect();
    let groups: Vec<String> = read_groups(data[1]);
    let mut cache: HashMap<String, usize> = HashMap::new();
    count(&springs, &groups, &mut cache)
}

pub fn solve(input: &str) -> usize {
    input.lines().map(|l| count_combination(l)).sum()
}

pub fn solve2(input: &str) -> usize {
    input.lines().enumerate().map(|(i, l)| {
        let c = count_combination(&expand(l));
        println!("{i} {l} {c}");
        c
    }).sum()
}

pub fn expand(input: &str) -> String {
    RE.replace(input, "$1?$1?$1?$1?$1 $2,$2,$2,$2,$2").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_match() {
        assert_eq!(match_controle("#.#.###", &vec![1,1,3]), true);
        assert_eq!(match_controle(".#..###", &vec![1,1,3]), false);
    }

    #[test]
    fn it_is_read_groups() {
        assert_eq!(read_groups("3"), vec![String::from("###")]);
        assert_eq!(read_groups("1,3"), vec![String::from("#"), String::from("###")]);
    }

    #[test]
    fn it_count_with_question_marks() {
        let mut cache: HashMap<String, usize> = HashMap::new();
        assert_eq!(count(&("??##".chars().collect::<Vec<char>>()), &read_groups("1,2"), &mut cache), 1);
        assert_eq!(count(&("???##".chars().collect::<Vec<char>>()), &read_groups("1,2"), &mut cache), 2);
        assert_eq!(count(&("?##".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 1);
        assert_eq!(count(&("?#.".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 0);
        assert_eq!(count(&("??##".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 1);
    }

    #[test]
    fn it_count() {
        let mut cache: HashMap<String, usize> = HashMap::new();
        assert_eq!(count(&("###".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 1);
        assert_eq!(count(&("###..".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 1);
        assert_eq!(count(&(".###".chars().collect::<Vec<char>>()), &read_groups("3"), &mut cache), 1);
        assert_eq!(count(&("#.###".chars().collect::<Vec<char>>()), &read_groups("1,3"), &mut cache), 1);
        assert_eq!(count(&("#.#.###".chars().collect::<Vec<char>>()), &read_groups("1,1,3"), &mut cache), 1);
        assert_eq!(count(&("..###".chars().collect::<Vec<char>>()), &read_groups("1,3"), &mut cache), 0);
        assert_eq!(count(&("#..##".chars().collect::<Vec<char>>()), &read_groups("1,3"), &mut cache), 0);
        assert_eq!(count(&("#..###".chars().collect::<Vec<char>>()), &read_groups("1,3"), &mut cache), 1);
        assert_eq!(count(&("##.###".chars().collect::<Vec<char>>()), &read_groups("1,3"), &mut cache), 0);
        assert_eq!(count(&("#.#..##".chars().collect::<Vec<char>>()), &read_groups("1,1,3"), &mut cache), 0);
    }

    #[test]
    fn it_count_combinations() {
        assert_eq!(count_combination(&expand("????.######..#####. 1,6,5")), 2500);
        assert_eq!(count_combination("????.#...#...# 4,1,1"), 0);
        assert_eq!(count_combination("?###???????? 3,2,1"), 10);
        assert_eq!(count_combination("???.### 1,1,3"), 1);
        assert_eq!(count_combination(".??..??...?##. 1,1,3"), 4);
        assert_eq!(count_combination("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(count_combination("????.#...#... 4,1,1"), 1);
        assert_eq!(count_combination("????.######..#####. 1,6,5"), 4);
    }

    #[test]
    fn it_expand() {
        assert_eq!(expand(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[test]
    fn it_works() {
        assert_eq!(solve(inputs::EXAMPLE), 21);
        assert_eq!(solve(inputs::INPUT), 7716);
        assert_eq!(solve2(inputs::EXAMPLE), 525152);
        assert_eq!(solve2(inputs::INPUT), 18716325559999);
    }
}
