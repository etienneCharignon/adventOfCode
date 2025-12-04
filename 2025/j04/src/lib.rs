mod inputs;

pub fn deplacable(x: i64, y: i64, champ: &Vec<Vec<i64>>, l: i64, h: i64) -> bool {
    if champ[y as usize][x as usize] == 0 {
        return false;
    }
    let mut nombre_rouleau = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let px = x + i;
            let py = y + j;
            if px >= 0 && py >= 0 && px < l && py < h {
                nombre_rouleau += champ[py as usize][px as usize];
            }
        }
    }
    if nombre_rouleau <= 4 {
        println!("{x},{y}: {nombre_rouleau}");
    }
    nombre_rouleau <= 4
}

pub fn compte_deplacable(entree: &str) -> u64 {
    let champ: Vec<_> = entree
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<i64>>()
        })
        .collect();
    println!("{champ:?}");
    let h = champ.len() as i64;
    let l = champ[0].len() as i64;
    let mut compte = 0;
    for y in 0..h {
        for x in 0..l {
            if deplacable(x, y, &champ, l, h) {
                compte += 1;
            }
        }
    }
    compte
}

pub fn compte_supprimables(entree: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_deplacable() {
        assert_eq!(compte_deplacable(inputs::EXEMPLE), 13);
        assert_eq!(compte_deplacable(inputs::INPUT), 1508);
    }

    #[test]
    fn il_compte_supprimables() {
        assert_eq!(compte_supprimables(inputs::EXEMPLE), 43);
        assert_eq!(compte_supprimables(inputs::INPUT), 13);
    }
}
