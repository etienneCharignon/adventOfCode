mod inputs;


use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?ms)Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d*), Y=(\d*)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Coordinate {
   x: i64,
   y: i64
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Machine {
   a: Coordinate,
   b: Coordinate,
   p: Coordinate
}

pub fn read(input: &str) -> Vec<Machine> {
    let mut machines = vec![];

    for captures in RE.captures_iter(input) {
        let numbers: Vec<i64> = captures.iter()
            .skip(1)
            .filter_map(|m| m.map(|m| m.as_str().parse().unwrap()))
            .collect();
        println!("{:?}", numbers);
        
        machines.push(Machine{
            a: Coordinate { x: numbers[0] ,y:numbers[1] },
            b: Coordinate { x: numbers[2] ,y:numbers[3] },
            p: Coordinate { x: numbers[4] + 10000000000000 ,y:numbers[5] + 10000000000000 }
        })
    }
    machines
}

fn is_integer(f: f64) -> bool {
    println!("is integer {f} {}: {}", f.round(), f64::abs(f - f.round()) < 0.1);
    f64::abs(f - f.round()) < 1e-3
}

pub fn minimum_cost(machine: &Machine) -> i64 {
    let xa = machine.a.x as f64;
    let ya = machine.a.y as f64;
    let xb = machine.b.x as f64;
    let yb = machine.b.y as f64;
    let xp = machine.p.x as f64;
    let yp = machine.p.y as f64;
    let nb = (yp/ya - xp/xa) / (yb/ya - xb/xa);
    let na = (yp/yb - xp/xb) / (ya/yb - xa/xb);
    println!("{na}, {nb}");
    if is_integer(nb) && is_integer(na) {
        na.round() as i64 * 3 + nb.round() as i64
    }
    else {
        0
    }
}

pub fn total_cost(machines: Vec<Machine>) -> i64 {
    machines.iter().map(|m| minimum_cost(m)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_integer() {
        assert_eq!(is_integer(132643157579.00002), true);
    }

    #[test]
    fn it_works() {
        assert_eq!(total_cost(read(inputs::EXAMPLE)), 875318608908);
        assert_eq!(total_cost(read(inputs::INPUT)), 72587986598368);
    }
}
