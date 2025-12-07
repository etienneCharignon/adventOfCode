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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_les_divisions() {
        assert_eq!(compte_division(inputs::EXEMPLE), 21);
        assert_eq!(compte_division(inputs::INPUT), 1656);
    }
}
