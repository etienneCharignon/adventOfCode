mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MAPRE: Regex = Regex::new(r"(.)(\w+) -> (.+)").unwrap();
    static ref MODULES: Regex = Regex::new(r", ").unwrap();
}

/*
trait Pulsable {
    fn output(&mut self, pulse: Pulse, memories: &mut HashMap<&'static str, HashMap<&'static str, bool>>) -> Vec<Pulse>;
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct FlipFlop {
    pub state: bool,
    pub next: &'static str,
}

impl Pulsable for FlipFlop {
    fn output(&mut self, pulse: Pulse, _memories: &mut HashMap<&'static str, HashMap<&'static str, bool>>) -> Vec<Pulse> {
        if ! pulse.level {
            self.state = !self.state;
            vec![Pulse{level: self.state, from: pulse.to, to: self.next}]
        }
        else {
            vec![]
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub struct Conjunction {
    pub name: &'static str,
    pub next: &'static str,
}

impl Pulsable for Conjunction {
    fn output(&mut self, pulse: Pulse, memories: &mut HashMap<&'static str, HashMap<&'static str, bool>>) -> Vec<Pulse> {
        let memorie = memories.get_mut(self.name).unwrap();
        memorie.insert(pulse.from, pulse.level);
        println!("{memorie:?}");
        vec![Pulse{from: self.name, to: self.next, level: !memorie.iter().all(|m| *m.1)}]
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Brodcaster {
    pub next: &'static str,
}

impl Pulsable for Brodcaster {
    fn output(&mut self, pulse: Pulse, _memories: &mut HashMap<&'static str, HashMap<&'static str, bool>>) -> Vec<Pulse> {
        MODULES.split(self.next)
               .map(|name| Pulse { from: pulse.to , to: name, level: pulse.level })
               .collect()
    }
}

impl Pulsable for Module {
    fn output(&mut self, pulse: Pulse, memories: &mut HashMap<&'static str, HashMap<&'static str, bool>>) -> Vec<Pulse> {
        match self {
            Module::FlipFlop(module) => {
                module.output(pulse, memories)
            },
            Module::Conjunction(module) => {
                module.output(pulse, memories)
            },
            Module::Brodcaster(module) => {
                module.output(pulse, memories)
            }
        }
    }
}
*/

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Module {
    pub name: String,
    pub next: String,
    pub t: char,
 }

 impl Module {
    fn new(name: &str, next: &str, t: char) -> Module {
        Module { name: String::from(name), next: String::from(next), t: t }
    }
 }

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Pulse
{
    pub from: String,
    pub level: bool,
    pub to: String,
}

impl Pulse {
    fn new(from: &str, to: &str, level: bool) -> Pulse {
        Pulse { from: String::from(from), to: String::from(to), level: level }
    }
}

fn output_flipflop(module: &Module, pulse: Pulse, memories: &mut HashMap<String, HashMap<String, bool>>) -> Vec<Pulse> {
    if ! pulse.level {
        let memorie = memories.get_mut(&module.name).unwrap();
        let state: bool = match memorie.get("state") {
            None => false,
            Some(state) => *state,
        };
        let new_state = !state;
        memorie.insert(String::from("state"), new_state); 
        MODULES.split(module.next.as_str())
                .map(|next_name| Pulse::new(&pulse.to, next_name, new_state))
                .collect()
    }
    else {
        vec![]
    }
}

fn output_conjonction(module: &Module, pulse: Pulse, memories: &mut HashMap<String, HashMap<String, bool>>) -> Vec<Pulse> {
    let memorie = memories.get_mut(&module.name).unwrap();
    memorie.insert(pulse.from, pulse.level);
    MODULES.split(module.next.as_str())
            .map(|next_name| Pulse::new(&module.name, &next_name, !memorie.iter().all(|m| *m.1)))
            .collect()
}

fn output_broadcast(module: &Module, pulse: Pulse, _memories: &mut HashMap<String, HashMap<String, bool>>) -> Vec<Pulse> {
    MODULES.split(module.next.as_str())
            .map(|next_name| Pulse::new(&module.name, next_name, pulse.level))
            .collect()
}

pub fn count_pulses(modules: &HashMap<&str, Module>, initial_memories: &mut HashMap<String, HashMap<String, bool>>) -> usize {
    let mut count_high = 0;
    let mut count_low = 0;

    let mut memories = initial_memories.clone();
    for _i in 0..1000 {
        let mut live_pulses = Vec::<Pulse>::new();
        let p = Pulse::new("button", "roadcaster", false);
        // println!("pulse : {p:?}");
        live_pulses.push(p);
        count_low += 1;
        while ! live_pulses.is_empty() {
            let mut next_pulses = vec![];
            for pulse in live_pulses {
                match modules.get(pulse.to.as_str()) {
                    Some(module) => {
                        let ps = match module.t {
                            '%' => output_flipflop(module, pulse, &mut memories),
                            '&' => output_conjonction(module, pulse, &mut memories),
                            'b' => output_broadcast(module, pulse, &mut memories),
                            _ => panic!("Module type unknown"),
                        };
                        for p in ps {
                            // println!("pulse : {p:?}");
                            next_pulses.push(p);
                        }
                    }
                    None => {}
                };
            }
            live_pulses = next_pulses.clone();
            // println!("{live_pulses:?}");
            count_high += live_pulses.iter().filter(|p| p.level).count();
            count_low += live_pulses.iter().filter(|p| !p.level).count();
            /*
            if count > 3 { break; }
            */
        }
    }
    println!("{count_high} {count_low}");
    count_high * count_low
}

pub fn solve(input: &'static str) -> usize {
    let mut modules = HashMap::<&str, Module>::new();
    let mut memories = HashMap::<String, HashMap<String, bool>>::new();
    for line in input.lines() {
        let (_, [module_type, name, next]): (&str, [&str; 3]) = MAPRE.captures(line).unwrap().extract();
        let  module = match module_type {
            "%" => Module::new(name, next, '%'),
            "&" => Module::new(name, next, '&'),
            "b" => Module::new(name, next, 'b'),
            _ => panic!("Unknown module type"),
        };
        modules.insert(name, module);
        memories.insert(String::from(name), HashMap::<String, bool>::new());
    }
    println!("{memories:?}");
    for module in modules.values() {
        for next_name in MODULES.split(module.next.as_str()) {
            println!("{next_name}");
            match memories.get_mut(&String::from(next_name)) {
                None => {
                    memories.insert(String::from(next_name), HashMap::from([(module.name.clone(), false)]));
                },
                Some(memorie) => { memorie.insert(module.name.clone(), false); }
            };
        }
    }
    modules.insert("button", Module::new("button", "roadcaster", 'b'));
    println!("{modules:?}");
    println!("{memories:?}");
    count_pulses(&mut modules, &mut memories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_brocast() {
        let mut memories = HashMap::<String, HashMap<String, bool>>::new();
        let button = Module::new("button", "roadcaster", 'b');
        assert_eq!(output_broadcast(&button, Pulse::new("", "button", false), &mut memories), vec![Pulse::new( "button", "roadcaster", false)]);
        let broadcaster = Module::new("roadcaster", "a, b", 'b');
        assert_eq!(output_broadcast(&broadcaster, Pulse::new("", "roadcaster", false), &mut memories),
                   vec![Pulse::new( "roadcaster", "a", false),
                        Pulse::new( "roadcaster", "b", false)]);
    }

    #[test]
    fn it_flipflop() {
        let mut memories = HashMap::<String, HashMap<String, bool>>::new();
        memories.insert(String::from("a"), HashMap::<String, bool>::new());
        let a = Module::new("a", "b", '%');
        assert_eq!(output_flipflop(&a, Pulse::new( "", "a", true), &mut memories), vec![]);
        assert_eq!(output_flipflop(&a, Pulse::new( "", "a", false), &mut memories), vec![Pulse::new("a", "b", true)]);
        assert_eq!(output_flipflop(&a, Pulse::new( "", "a", false), &mut memories), vec![Pulse::new("a", "b", false)]);
    }

    #[test]
    fn it_conjonct() {
        let mut memories = HashMap::<String, HashMap<String, bool>>::new();
        memories.insert(String::from("conj"), HashMap::from([(String::from("a"), false), (String::from("b"), false)]));
        let conj = Module::new("conj", "c", '&');
        assert_eq!(output_conjonction(&conj, Pulse::new("b", "conj", true), &mut memories), vec![Pulse::new("conj", "c", true)]);
        assert_eq!(output_conjonction(&conj, Pulse::new("a", "conj", true), &mut memories), vec![Pulse::new("conj", "c", false)]);
        assert_eq!(output_conjonction(&conj, Pulse::new("b", "conj", false), &mut memories), vec![Pulse::new("conj", "c", true)]);
    }

    #[test]
    fn it_solve() {
        assert_eq!(solve(inputs::EXAMPLE), 32000000);
        assert_eq!(solve(inputs::EXAMPLE2), 11687500);
        assert_eq!(solve(inputs::INPUT), 818723272);
    }
}
