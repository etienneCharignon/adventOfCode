mod inputs;

pub fn compte_division(collecteur: &str) -> i64 {
    let mut etat_final: Vec<Vec<char>> = vec![];
    let mut nombres_division = 0;
    for (i, ligne) in collecteur.lines().enumerate() {
        let nouvelle_ligne = if etat_final.is_empty() {
            ligne
                .chars()
                .map(|c| if c == 'S' { '|' } else { c })
                .collect()
        } else {
            let chars: Vec<_> = ligne.chars().collect();
            chars
                .clone()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    '^' => {
                        if etat_final[i - 1][x] == '|' {
                            nombres_division += 1;
                        }
                        *c
                    }
                    _ => {
                        if x < chars.len() - 1
                            && chars[x + 1] == '^'
                            && etat_final[i - 1][x + 1] == '|'
                            || x > 0 && chars[x - 1] == '^' && etat_final[i - 1][x - 1] == '|'
                            || etat_final[i - 1][x] == '|'
                        {
                            '|'
                        } else {
                            *c
                        }
                    }
                })
                .collect()
        };
        etat_final.push(nouvelle_ligne);
    }
    println!("{etat_final:?}");
    nombres_division
}

pub fn compte_lignes_temporelles(collecteur: &str) -> i64 {
    let mut compteurs_lignes_temporelles: Vec<Vec<i64>> = vec![];
    for (i, ligne) in collecteur.lines().enumerate() {
        let nouvelle_ligne = if compteurs_lignes_temporelles.is_empty() {
            ligne
                .chars()
                .map(|c| if c == 'S' { 1 } else { 0 })
                .collect()
        } else {
            let chars: Vec<_> = ligne.chars().collect();
            chars
                .clone()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    '^' => 0,
                    _ => {
                        let mut nombres_de_lignes_temporelles = 0;
                        if x < chars.len() - 1
                            && chars[x + 1] == '^'
                            && compteurs_lignes_temporelles[i - 1][x + 1] > 0
                        {
                            nombres_de_lignes_temporelles =
                                compteurs_lignes_temporelles[i - 1][x + 1];
                        }
                        if x > 0
                            && chars[x - 1] == '^'
                            && compteurs_lignes_temporelles[i - 1][x - 1] > 0
                        {
                            nombres_de_lignes_temporelles +=
                                compteurs_lignes_temporelles[i - 1][x - 1];
                        }
                        if compteurs_lignes_temporelles[i - 1][x] > 0 {
                            nombres_de_lignes_temporelles += compteurs_lignes_temporelles[i - 1][x];
                        }
                        nombres_de_lignes_temporelles
                    }
                })
                .collect()
        };
        compteurs_lignes_temporelles.push(nouvelle_ligne);
    }
    println!("{compteurs_lignes_temporelles:?}");
    compteurs_lignes_temporelles[compteurs_lignes_temporelles.len() - 1]
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_les_divisions() {
        assert_eq!(compte_division(inputs::EXEMPLE), 21);
        assert_eq!(compte_division(inputs::INPUT), 1656);
    }

    #[test]
    fn il_compte_les_lignes_temporelles() {
        assert_eq!(compte_lignes_temporelles(inputs::EXEMPLE), 40);
        assert_eq!(compte_lignes_temporelles(inputs::INPUT), 76624086587804);
    }
}
