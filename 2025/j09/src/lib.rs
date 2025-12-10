mod inputs;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
    x: i64,
    y: i64,
}

pub fn surface(p1: Pos, p2: Pos) -> i64 {
    let l = (p2.x - p1.x).abs() + 1;
    let h = (p2.y - p1.y).abs() + 1;
    l * h
}

pub fn trouve_retangle_max(entrée: &str) -> i64 {
    entrée
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
        .max()
        .unwrap()
}

pub fn segments_intersect(segment1: &(Pos, Pos), segment2: &(Pos, Pos)) -> bool {
    let (p1, q1) = segment1;
    let (p2, q2) = segment2;

    // Détermine quel segment est horizontal et lequel est vertical
    let (horiz, vert) = if p1.y == q1.y {
        // segment1 horizontal, segment2 vertical
        ((p1, q1), (p2, q2))
    } else {
        // segment1 vertical, segment2 horizontal
        ((p2, q2), (p1, q1))
    };

    let y_horiz = horiz.0.y;
    let x_min_horiz = horiz.0.x.min(horiz.1.x);
    let x_max_horiz = horiz.0.x.max(horiz.1.x);

    let x_vert = vert.0.x;
    let y_min_vert = vert.0.y.min(vert.1.y);
    let y_max_vert = vert.0.y.max(vert.1.y);

    x_min_horiz < x_vert && x_vert < x_max_horiz && y_min_vert < y_horiz && y_horiz < y_max_vert
}

pub fn intersectionne(segment: (Pos, Pos), lignes: &HashSet<(Pos, Pos)>) -> bool {
    lignes.iter().any(|l| segments_intersect(&segment, l))
}

pub fn est_dans_lacet(
    pair: &Vec<Pos>,
    lacet: &Vec<Pos>,
    lignes_verticales: &HashSet<(Pos, Pos)>,
    lignes_horisontales: &HashSet<(Pos, Pos)>,
) -> bool {
    let p1 = pair[0];
    let p2 = pair[1];
    let intersection = intersectionne((p1, Pos { x: p1.x, y: p2.y }), lignes_verticales)
        || intersectionne((p2, Pos { x: p2.x, y: p1.y }), lignes_verticales)
        || intersectionne((p1, Pos { x: p2.x, y: p1.y }), lignes_horisontales)
        || intersectionne((p2, Pos { x: p1.x, y: p2.y }), lignes_horisontales);
    !intersection
}

pub fn trouve_retangle_max_inclus(entrée: &str) -> i64 {
    let points: Vec<_> = entrée
        .lines()
        .map(|l| {
            let pos: Vec<_> = l.split(',').map(|n| n.parse().unwrap()).collect();
            Pos {
                x: pos[0],
                y: pos[1],
            }
        })
        .collect();
    let mut lignes_verticales: HashSet<(Pos, Pos)> = HashSet::new();
    let mut lignes_horisontales: HashSet<(Pos, Pos)> = HashSet::new();
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if p1.x == p2.x {
            lignes_verticales.insert((p1, p2));
        } else {
            lignes_horisontales.insert((p1, p2));
        }
    }
    let surfaces: Vec<_> = points
        .clone()
        .into_iter()
        .combinations(2)
        .map(|pair| (pair.clone(), surface(pair[0], pair[1])))
        .filter(|(pair, _)| est_dans_lacet(pair, &points, &lignes_verticales, &lignes_horisontales))
        .collect();
    *surfaces.iter().map(|(_, s)| s).max().unwrap()
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
        assert_eq!(trouve_retangle_max(inputs::INPUT), 4765757080);
    }

    #[test]
    fn il_trouve_la_surface_max_incluse() {
        assert_eq!(trouve_retangle_max_inclus(inputs::EXEMPLE), 24);
        assert_eq!(trouve_retangle_max_inclus(inputs::INPUT), 4765757079);
    }
}
