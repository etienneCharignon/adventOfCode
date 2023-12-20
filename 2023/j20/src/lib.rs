mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref MAPRE: Regex = Regex::new(r"(.)(\w+) -> (.+)").unwrap();
    static ref MODULES: Regex = Regex::new(r", ").unwrap();
}

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

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Brodcaster(Brodcaster),
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

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pulse
{
    pub from: &'static str,
    pub to: &'static str,
    pub level: bool
}

pub fn count_pulses(mut modules: &mut HashMap<&str, Module>, memories: &mut HashMap<&str, HashMap<&str, bool>>) -> usize {
    let mut live_pulses = Vec::<Pulse>::new();
    live_pulses.push(Pulse { from: "button", to: "roadcaster", level: false });
    println!("{live_pulses:?}");
    let mut count_high = 0;
    let mut count_low = 1;
    while ! live_pulses.is_empty() {
        let mut next_pulses = vec![];
        for pulse in live_pulses {
            match modules.get_mut(pulse.to) {
                Some(module) => {
                    for p in module.output(pulse, memories) {
                        next_pulses.push(p);
                    }
                }
                None => {}
            };
        }
        live_pulses = next_pulses.clone();
        println!("{live_pulses:?}");
        count_high += live_pulses.iter().filter(|p| p.level).count();
        count_low += live_pulses.iter().filter(|p| !p.level).count();
        /*
        if count > 3 { break; }
        */
    }
    count_high * count_low
}

pub fn solve(input: &'static str) -> usize {
    let mut modules = HashMap::<&str, Module>::new();
    let mut memories = HashMap::<&str, HashMap<&str, bool>>::new();
    for line in input.lines() {
        let (_, [module_type, name, next]): (&str, [&str; 3]) = MAPRE.captures(line).unwrap().extract();
        let  module = match module_type {
            "%" => Module::FlipFlop(FlipFlop { state: false, next: next }),
            "&" => Module::Conjunction(Conjunction { name: name, next: next }),
            "b" => {
                Module::Brodcaster(Brodcaster{ next: next })
            },
            _ => panic!("Unknown module type"),
        };
        memories.insert(name, HashMap::<&str, bool>::new());
        modules.insert(name, module);
    }
    modules.insert("button", Module::Brodcaster(Brodcaster{ next: "roadcaster" }));
    println!("{modules:?}");
    count_pulses(&mut modules, &mut memories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_brocast() {
        let mut memories = HashMap::<&str, HashMap<&str, bool>>::new();
        let mut button = Module::Brodcaster(Brodcaster{ next: "rocaster" });
        assert_eq!(button.output(Pulse { from:"", to: "button", level: false }, &mut memories), vec![Pulse { from: "button", to: "rocaster", level: false }]);
        let mut brodcaster = Module::Brodcaster(Brodcaster{ next: "a, b" });
        assert_eq!(brodcaster.output(Pulse { from:"", to: "roadcaster", level: false }, &mut memories),
                   vec![Pulse { from: "roadcaster", to: "a", level: false },
                        Pulse { from: "roadcaster", to: "b", level: false }]);
    }

    #[test]
    fn it_flipflop() {
        let mut memories = HashMap::<&str, HashMap<&str, bool>>::new();
        let mut a = Module::FlipFlop(FlipFlop{ state: false, next: "b" });
        assert_eq!(a.output(Pulse { from: "", to: "a", level: true }, &mut memories), vec![]);
        assert_eq!(a.output(Pulse { from: "", to: "a", level: false }, &mut memories), vec![Pulse { from:"a", to: "b", level: true }]);
        assert_eq!(a.output(Pulse { from: "", to: "a", level: false }, &mut memories), vec![Pulse { from:"a", to: "b", level: false }]);
    }

    #[test]
    fn it_conjonct() {
        let mut memories = HashMap::<&str, HashMap<&str, bool>>::new();
        memories.insert("conj", HashMap::<&str, bool>::new());
        let mut conj = Module::Conjunction(Conjunction{ name: "conj", next: "c" });
        assert_eq!(conj.output(Pulse { from:"b", to: "conj", level: true }, &mut memories), vec![Pulse { from:"conj", to: "c", level: false }]);
        assert_eq!(conj.output(Pulse { from:"a", to: "conj", level: false }, &mut memories), vec![Pulse { from:"conj", to: "c", level: true }]);
        assert_eq!(conj.output(Pulse { from:"b", to: "conj", level: true }, &mut memories), vec![Pulse { from:"conj", to: "c", level: true }]);
        assert_eq!(conj.output(Pulse { from:"a", to: "conj", level: true }, &mut memories), vec![Pulse { from:"conj", to: "c", level: false }]);
    }

    #[test]
    fn it_solve() {
        assert_eq!(solve(inputs::EXAMPLE), 32000000);
        // assert_eq!(solve(inputs::EXAMPLE2), 11687500);
    }
}
