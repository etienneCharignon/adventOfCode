use std::collections::HashSet;
use z3::ast::Int;
use z3::*;

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

pub fn appuie_joltage(etat: &Vec<i64>, bouton: &[i64]) -> Vec<i64> {
    etat.iter()
        .enumerate()
        .map(|(i, etat)| {
            if bouton.contains(&(i as i64)) {
                etat + 1
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
            let (retour_min, succès) = trouve_état(&état, cible, boutons, n + 1, nouveau_min);
            if succès {
                println!("appuie {bouton:?}, {}", n + 1);
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

pub fn trouve_état_joltage(
    état_initial: &Vec<i64>,
    cible: &Vec<i64>,
    boutons: &[Vec<i64>],
    n: i64,
    min: i64,
) -> (i64, bool) {
    if n >= min || état_initial.iter().enumerate().any(|(i, j)| j > &cible[i]) {
        return (n, false);
    }
    if état_initial == cible {
        println!("====> Succés : n: {n}");
        (n, true)
    } else {
        let mut trouvé = false;
        let mut nouveau_min = min;
        for bouton in boutons {
            let état = appuie_joltage(état_initial, bouton);
            let (retour_min, succès) =
                trouve_état_joltage(&état, cible, boutons, n + 1, nouveau_min);
            if succès {
                println!("appuie {bouton:?}, {}", n + 1);
                nouveau_min = retour_min;
                trouvé = true;
            }
        }
        (nouveau_min, trouvé)
    }
}

pub fn compte_appuis_machine_joltage(machine: &str) -> i64 {
    let jetons: Vec<_> = machine.split_whitespace().collect();
    let boutons: Vec<_> = jetons[1..jetons.len() - 1]
        .iter()
        .map(|b| b.trim_matches(&['(', ')'][..]))
        .map(|b| {
            b.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<HashSet<_>>()
        })
        .collect();
    let cible: Vec<_> = jetons[jetons.len() - 1]
        .trim_matches(&['{', '}'][..])
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    println!("Entrée : {boutons:?} {cible:?}");
    /*
    let état_initial = vec![0; cible.len()];
    let (min, succes) = trouve_état_joltage(&état_initial, &cible, &boutons, 0, 15);
    if !succes {
        panic!("min initial issufisant.")
    }
    min*/
    let variables: Vec<_> = jetons[1..jetons.len() - 1]
        .iter()
        .map(|nom| Int::fresh_const(nom))
        .collect();
    let solver = Optimize::new();
    let somme = z3::ast::Int::add(&variables);
    solver.minimize(&somme);
    for variable in &variables {
        solver.assert(&variable.ge(0));
    }
    for (i, voltage) in cible.iter().enumerate() {
        let variables_concernées: Vec<_> = boutons
            .iter()
            .enumerate()
            .filter(|(_, b)| b.contains(&i))
            .map(|(ib, _)| &variables[ib])
            .collect();
        let somme = z3::ast::Int::add(&variables_concernées);

        solver.assert(&somme.eq(*voltage));
    }
    if solver.check(&[]) == SatResult::Sat {
        let model = solver.get_model().unwrap();

        let values: Vec<i64> = variables
            .iter()
            .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
            .collect();
        println!("Solutions: {:?}", values);

        model.eval(&somme, true).unwrap().as_i64().unwrap()
    } else {
        panic!("pas de solution")
    }
}

pub fn compte_appuis_joltage(entrées: &str) -> i64 {
    entrées.lines().map(compte_appuis_machine_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "travaille sur la partie 2"]
    fn il_appuie() {
        assert_eq!(appuie(&vec![false], &[0]), vec![true]);
        assert_eq!(appuie(&vec![false, false], &[1]), vec![false, true]);
        assert_eq!(appuie(&vec![false, false], &[0, 1]), vec![true, true]);
        assert_eq!(appuie(&vec![true, false], &[0, 1]), vec![false, true]);
    }

    #[test]
    #[ignore = "travaille sur la partie 2"]
    fn il_compt_les_appuis_pour_les_lampes() {
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

    #[test]
    #[ignore = "travaille sur la partie 2"]
    fn il_compt_les_appuis_pour_les_lampes_2() {
        assert_eq!(
            compte_appuis_machine("[##.#] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
        assert_eq!(
            compte_appuis_machine("[#..#] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
        assert_eq!(
            compte_appuis_machine("[.###] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            2
        );
    }

    #[test]
    fn il_compt_les_appuis_machine_joltage() {
        assert_eq!(
            compte_appuis_machine_joltage("[##.#] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            10
        );
        assert_eq!(
            compte_appuis_machine_joltage(
                "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            ),
            12
        );
    }

    #[test]
    fn il_compt_les_appuis_pour_joltage() {
        assert_eq!(compte_appuis_joltage(inputs::EXEMPLE), 33);
        assert_eq!(compte_appuis_joltage(inputs::INPUT), 21021);
    }
}
