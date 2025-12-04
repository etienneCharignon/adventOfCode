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
    déplacés: &HashSet<Pos>,
) -> bool {
    if champ[y as usize][x as usize] == 0 || déplacés.contains(&Pos { x, y }) {
        return false;
    }
    let mut nombre_rouleaux = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let p = Pos { x: x + i, y: y + j };
            if p.x >= 0 && p.y >= 0 && p.x < l && p.y < h && !déplacés.contains(&p) {
                nombre_rouleaux += champ[p.y as usize][p.x as usize];
            }
        }
    }
    nombre_rouleaux <= 4
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
    let déplacés = HashSet::<Pos>::new();
    let mut déplacables = HashSet::<Pos>::new();
    for y in 0..h {
        for x in 0..l {
            if déplacable(x, y, &champ, l, h, &déplacés) {
                déplacables.insert(Pos { x, y });
            }
        }
    }
    déplacables.len()
}

pub fn compte_déplacés(entree: &str) -> usize {
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
    let mut déplacés = HashSet::<Pos>::new();
    let mut déplacables = HashSet::from([Pos { x: 0, y: 0 }]);
    while !déplacables.is_empty() {
        déplacables.clear();
        for y in 0..h {
            for x in 0..l {
                if déplacable(x, y, &champ, l, h, &déplacés) {
                    déplacables.insert(Pos { x, y });
                }
            }
        }
        déplacés.extend(&déplacables);
    }
    déplacés.len()
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
    fn il_compte_déplacés() {
        assert_eq!(compte_déplacés(inputs::EXEMPLE), 43);
        assert_eq!(compte_déplacés(inputs::INPUT), 8538);
    }
}
