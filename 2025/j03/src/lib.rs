mod inputs;

pub fn voltage_max(banc: &str) -> i64 {
    let batteries: Vec<i64> = banc
        .chars()
        .into_iter()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    println!("{batteries:?}");
    let max = &batteries[..batteries.len() - 1].iter().max().unwrap();
    println!("{max}");
    let position_max = batteries.iter().position(|b| b == *max).unwrap();
    println!("{position_max}");
    let max_suivant = &batteries[position_max + 1..].iter().max().unwrap();
    println!("{max_suivant}");
    println!("{position_max}{max_suivant}");
    format!("{max}{max_suivant}").parse().unwrap()
}

pub fn somme_voltage(entree: &str) -> i64 {
    entree.lines().map(|banc| voltage_max(banc)).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trouve_voltage_maximum() {
        assert_eq!(voltage_max("987654321111111"), 98);
        assert_eq!(voltage_max("811111111111119"), 89);
        assert_eq!(voltage_max("234234234234278"), 78);
    }

    #[test]
    fn somme_voltage_bancs() {
        assert_eq!(somme_voltage(inputs::EXEMPLE), 357);
        assert_eq!(somme_voltage(inputs::INPUT), 17092);
    }
}
