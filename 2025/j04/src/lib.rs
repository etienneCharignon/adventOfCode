mod inputs;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
    x: i64,
    y: i64,
}

pub fn déplacable(
    x: i64,
    y: i64,
    champ: &[Vec<i64>],
    l: i64,
    h: i64,
    supprimés: &HashSet<Pos>,
) -> bool {
    if champ[y as usize][x as usize] == 0 || supprimés.contains(&Pos { x, y }) {
        return false;
    }
    let mut nombre_rouleau = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let px = x + i;
            let py = y + j;
            if px >= 0 && py >= 0 && px < l && py < h && !supprimés.contains(&Pos { x: px, y: py })
            {
                nombre_rouleau += champ[py as usize][px as usize];
            }
        }
    }
    nombre_rouleau <= 4
}

pub fn compte_deplacable(entree: &str) -> usize {
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
    let déplacées = HashSet::<Pos>::new();
    let mut déplacables = HashSet::<Pos>::new();
    for y in 0..h {
        for x in 0..l {
            if déplacable(x, y, &champ, l, h, &déplacées) {
                déplacables.insert(Pos { x, y });
            }
        }
    }
    déplacables.len()
}

pub fn compte_déplacées(entree: &str) -> usize {
    let champ: Vec<_> = entree
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect::<Vec<i64>>()
        })
        .collect();
    let h = champ.len() as i64;
    let l = champ[0].len() as i64;
    let mut déplacées = HashSet::<Pos>::new();
    let mut déplacables = HashSet::from([Pos { x: 0, y: 0 }]);
    while !déplacables.is_empty() {
        déplacables.clear();
        for y in 0..h {
            for x in 0..l {
                if déplacable(x, y, &champ, l, h, &déplacées) {
                    déplacables.insert(Pos { x, y });
                }
            }
        }
        déplacées.extend(&déplacables);
    }
    déplacées.len()
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
    fn il_compte_déplacées() {
        assert_eq!(compte_déplacées(inputs::EXEMPLE), 43);
        assert_eq!(compte_déplacées(inputs::INPUT), 8538);
    }
}
