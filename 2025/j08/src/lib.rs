mod inputs;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::vec;

use itertools::Itertools;

pub fn distance(a: &[i64], b: &[i64]) -> i64 {
    (b[0] - a[0]).pow(2) + (b[1] - a[1]).pow(2) + (b[2] - a[2]).pow(2)
}

pub fn compte_circuits(entrée: &str, max_nombre_guirlands: usize) -> usize {
    let boites: Vec<_> = entrée
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .collect();
    let mut distances: Vec<_> = boites
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1], distance(pair[0], pair[1])))
        .filter(|distance| distance.2 > 0)
        .collect();
    distances.sort_by_key(|d| d.2);
    // println!("{distances:?}");
    let mut circuits: Vec<_> = vec![];
    let mut nombre_guirlands = 0;
    for distance in distances {
        if nombre_guirlands == max_nombre_guirlands {
            break;
        }
        println!("{circuits:?}");
        println!("D = {distance:?}");
        let (b1, b2, _) = distance;
        let mut trouvés: Vec<_> = circuits
            .iter_mut()
            .filter(|c: &&mut HashSet<&Vec<_>>| c.contains(b1) || c.contains(b2))
            .collect();
        if trouvés.is_empty() {
            nombre_guirlands += 1;
            circuits.push(HashSet::from([b1, b2]));
            continue;
        }
        if trouvés.len() == 1 {
            let circuit = &mut trouvés[0];
            if circuit.contains(b1) && circuit.contains(b2) {
                //continue;
            }

            nombre_guirlands += 1;
            circuit.insert(b1);
            circuit.insert(b2);
        } else {
            println!("{trouvés:?}");
            let (premier, reste) = trouvés.split_at_mut(1);
            let circuit = &mut premier[0];
            let circuit2 = &mut reste[0];

            circuit.extend(circuit2.iter());
            circuit2.clear();

            nombre_guirlands += 1;
            circuit.insert(b1);
            circuit.insert(b2);
        }
    }
    circuits.sort_by_key(|c| Reverse(c.len()));
    println!("{circuits:?}");
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_les_trois_plus_grand_circuits() {
        assert_eq!(compte_circuits(inputs::EXEMPLE, 10), 40);
        assert_eq!(compte_circuits(inputs::INPUT, 1000), 123420);
    }
}
