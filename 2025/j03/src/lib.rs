mod inputs;

pub fn max_suivant(batteries: &[i64], taille: usize) -> i64 {
    if taille == 1 {
        return *batteries.iter().max().unwrap();
    }
    let bs = &batteries[..=batteries.len() - taille];
    let max = bs.iter().max().unwrap();
    println!("{max} {bs:?} {batteries:?} {taille}");
    let position_max = batteries.iter().position(|b| *b == *max).unwrap();
    let max_suivant = max_suivant(&batteries[position_max + 1..], taille - 1);
    format!("{max}{max_suivant}").parse().unwrap()
}

pub fn voltage_max(banc: &str) -> i64 {
    let batteries: Vec<i64> = banc
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    max_suivant(&batteries, 12)
}

pub fn somme_voltage(entree: &str) -> i64 {
    entree.lines().map(voltage_max).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trouve_voltage_maximum() {
        assert_eq!(voltage_max("234234234234278"), 434234234278);
        assert_eq!(voltage_max("987654321111111"), 987654321111);
        assert_eq!(voltage_max("811111111111119"), 811111111119);
        assert_eq!(voltage_max("818181911112111"), 888911112111);
    }

    #[test]
    fn somme_voltage_bancs() {
        assert_eq!(somme_voltage(inputs::EXEMPLE), 3121910778619);
        assert_eq!(somme_voltage(inputs::INPUT), 170147128753455);
    }
}
