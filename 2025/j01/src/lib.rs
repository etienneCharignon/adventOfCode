mod inputs;

pub fn calcul(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut count = 0;
    let mut curseur = 50;
    for line in lines {
        let nombre: i32 = line[1..].parse().unwrap();
        let direction = if line.starts_with('R') { 1 } else { -1 };
        curseur = (curseur + nombre * direction) % 100;
        if curseur == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcul_secret_exemple() {
        assert_eq!(calcul(inputs::EXEMPLE), 3);
        assert_eq!(calcul(inputs::INPUT), 1141);
    }
}
