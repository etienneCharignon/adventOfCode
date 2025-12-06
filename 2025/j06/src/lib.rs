mod inputs;

pub fn somme_problèmes(cahier: &str) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_fait_les_maths() {
        assert_eq!(somme_problèmes(inputs::EXEMPLE), 4277556);
        assert_eq!(somme_problèmes(inputs::INPUT), 4076006202939);
    }
}
