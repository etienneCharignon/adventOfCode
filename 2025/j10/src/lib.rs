use std::i64;

mod inputs;

pub fn appuie(etat: &Vec<bool>, bouton: &[i64]) -> Vec<bool> {
    etat.iter()
        .enumerate()
        .map(|(i, etat)| {
            if bouton.contains(&(i as i64)) {
                !etat
            } else {
                *etat
            }
        })
        .collect()
}

pub fn affiche(état: &Vec<bool>) {
    let str: String = état.iter().map(|b| if *b { '#' } else { '.' }).collect();
    println!("{str}");
}

pub fn trouve_état(
    état_initial: &Vec<bool>,
    cible: &Vec<bool>,
    boutons: &[Vec<i64>],
    n: i64,
    min: i64,
) -> (i64, bool) {
    if n >= min || boutons.is_empty() {
        return (n, false);
    }
    if état_initial == cible {
        println!("====> Succés : n: {n}");
        (n, true)
    } else {
        let mut trouvé = false;
        let mut nouveau_min = min;
        for bouton in boutons {
            let état = appuie(état_initial, bouton);
            //println!("appuie {bouton:?}, {}", n + 1);
            let (retour_min, succès) = trouve_état(&état, cible, boutons, n + 1, nouveau_min);
            if succès {
                nouveau_min = retour_min;
                trouvé = true;
            }
        }
        (nouveau_min, trouvé)
    }
}
pub fn compte_appuis_machine(machine: &str) -> i64 {
    let jetons: Vec<_> = machine.split_whitespace().collect();
    let cible: Vec<bool> = jetons[0]
        .trim_matches(&['[', ']'][..])
        .chars()
        .map(|c| c == '#')
        .collect();
    let boutons: Vec<_> = jetons[1..jetons.len() - 1]
        .iter()
        .map(|b| b.trim_matches(&['(', ')'][..]))
        .map(|b| {
            b.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    println!("{cible:?} {boutons:?}");
    let état_initial = vec![false; cible.len()];
    let (min, succes) = trouve_état(&état_initial, &cible, &boutons, 0, 9);
    if !succes {
        panic!("min initial issufisant.")
    }
    min
}

pub fn compte_appuis(entrées: &str) -> i64 {
    entrées.lines().map(compte_appuis_machine).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_appuie() {
        assert_eq!(appuie(&vec![false], &[0]), vec![true]);
        assert_eq!(appuie(&vec![false, false], &[1]), vec![false, true]);
        assert_eq!(appuie(&vec![false, false], &[0, 1]), vec![true, true]);
        assert_eq!(appuie(&vec![true, false], &[0, 1]), vec![false, true]);
    }

    #[test]
    fn il_compt_les_appuis() {
        assert_eq!(
            compte_appuis_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
        assert_eq!(
            compte_appuis_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            3
        );
        assert_eq!(compte_appuis(inputs::EXEMPLE), 7);
        assert_eq!(compte_appuis(inputs::INPUT), 535);
    }
}
