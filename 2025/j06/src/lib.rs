mod inputs;

pub fn somme_problèmes_1(cahier: &str) -> i64 {
    let lignes: Vec<_> = cahier.lines().collect();
    let opérations: Vec<_> = lignes[lignes.len() - 1].split_whitespace().collect();
    let nombres: Vec<_> = lignes[..lignes.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    println!("{opérations:?}, {nombres:?}");
    let problèmes: Vec<_> = opérations
        .iter()
        .map(|opération| valeur_initiale(opération))
        .collect();
    nombres
        .iter()
        .fold(problèmes, |problèmes, ligne_de_nombres| {
            problèmes
                .into_iter()
                .enumerate()
                .map(|(i, resultat)| match opérations[i] {
                    "*" => resultat * ligne_de_nombres[i],
                    _ => resultat + ligne_de_nombres[i],
                })
                .collect()
        })
        .iter()
        .sum()
}

pub fn valeur_initiale(opération: &str) -> i64 {
    match opération {
        "*" => 1,
        _ => 0,
    }
}

pub fn somme_problèmes_2(cahier: &str) -> i64 {
    let lignes: Vec<_> = cahier.lines().collect();
    let opérations: Vec<_> = lignes[lignes.len() - 1].split_whitespace().collect();
    let acc = vec![String::new(); lignes[0].len()];
    let chaines: Vec<_> = lignes[..lignes.len() - 1]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .fold(acc, |acc, chars| {
            acc.iter()
                .enumerate()
                .map(|(i, colonne)| {
                    let char = chars[i];
                    format!("{colonne}{char}")
                })
                .collect()
        });
    println!("{opérations:?}, {chaines:?}");

    let (t, _, r) = chaines.iter().map(|s| s.trim()).fold(
        (0, 0, valeur_initiale(opérations[0])),
        |(total, index_opération, resultat), chaine| {
            if chaine.is_empty() {
                return (
                    total + resultat,
                    index_opération + 1,
                    valeur_initiale(opérations[index_opération + 1]),
                );
            }
            let nombre = chaine.parse::<i64>().unwrap();
            (
                total,
                index_opération,
                match opérations[index_opération] {
                    "*" => resultat * nombre,
                    _ => resultat + nombre,
                },
            )
        },
    );
    t + r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_fait_les_maths_partie1() {
        assert_eq!(somme_problèmes_1(inputs::EXEMPLE), 4277556);
        assert_eq!(somme_problèmes_1(inputs::INPUT), 4076006202939);
    }

    #[test]
    fn il_fait_les_maths_partie2() {
        assert_eq!(somme_problèmes_2(inputs::EXEMPLE), 3263827);
        assert_eq!(somme_problèmes_2(inputs::INPUT), 7903168391557);
    }
}
