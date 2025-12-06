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
        .map(|opération| if *opération == "*" { 1 } else { 0 })
        .collect();
    nombres
        .iter()
        .fold(problèmes, |problèmes, ligne_de_nombres| {
            problèmes
                .into_iter()
                .enumerate()
                .map(|(i, resultat)| {
                    if opérations[i] == "*" {
                        return resultat * ligne_de_nombres[i];
                    }
                    resultat + ligne_de_nombres[i]
                })
                .collect()
        })
        .iter()
        .sum()
}

pub fn somme_problèmes_2(cahier: &str) -> i64 {
    let lignes: Vec<_> = cahier.lines().collect();
    let opérations: Vec<_> = lignes[lignes.len() - 1].split_whitespace().collect();
    let acc = lignes[0].chars().map(|_| String::new()).collect();
    let chaines: Vec<_> = lignes[..lignes.len() - 1]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .fold(acc, |acc, chars| {
            acc.iter()
                .enumerate()
                .map(|(i, colonne)| {
                    let mut nouvelle_chaine = colonne.clone();
                    nouvelle_chaine.push(chars[i]);
                    nouvelle_chaine
                })
                .collect()
        });
    println!("{opérations:?}, {chaines:?}");

    let mut total = 0;
    let mut index_opération = 0;
    let mut resultat = match opérations[index_opération] {
        "*" => 1,
        _ => 0,
    };
    for chaine in chaines {
        if chaine.trim() == "" {
            index_opération += 1;
            total += resultat;
            resultat = match opérations[index_opération] {
                "*" => 1,
                _ => 0,
            };
            continue;
        }
        let nombre = chaine.trim().parse::<i64>().unwrap();
        match opérations[index_opération] {
            "*" => resultat *= nombre,
            _ => resultat += nombre,
        }
    }
    total += resultat;
    total
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
