mod inputs;
use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
    x: i64,
    y: i64,
}

pub fn surface(p1: Pos, p2: Pos) -> i64 {
    let l = (p2.x - p1.x).abs() + 1;
    let h = (p2.y - p1.y).abs() + 1;
    println!("{l} * {h}");
    l * h
}

pub fn trouve_retangle_max(entrée: &str) -> i64 {
    let surfaces: Vec<_> = entrée
        .lines()
        .map(|l| {
            let pos: Vec<_> = l.split(',').map(|n| n.parse().unwrap()).collect();
            Pos {
                x: pos[0],
                y: pos[1],
            }
        })
        .combinations(2)
        .map(|pair| surface(pair[0], pair[1]))
        .collect();
    println!("{surfaces:?}");
    *surfaces.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_calcule_une_surface() {
        assert_eq!(surface(Pos { x: 2, y: 5 }, Pos { x: 9, y: 7 }), 24);
        assert_eq!(surface(Pos { x: 2, y: 5 }, Pos { x: 11, y: 1 }), 50);
    }

    #[test]
    fn il_trouve_la_surface_max() {
        assert_eq!(trouve_retangle_max(inputs::EXEMPLE), 50);
        assert_eq!(trouve_retangle_max(inputs::INPUT), 50);
    }
}
