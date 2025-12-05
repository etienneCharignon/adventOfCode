use std::vec;

mod inputs;

pub fn lit_intervals(str: &str) -> Vec<Vec<i64>> {
    str.lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn compte_ingrédients_frais(intervals_str: &str, ids_str: &str) -> usize {
    let intervals = lit_intervals(intervals_str);
    let ids: Vec<_> = ids_str.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    println!("{intervals:?}");
    println!("{ids:?}");
    ids.iter()
        .filter(|id| {
            intervals
                .iter()
                .any(|interval| (interval[0]..=interval[1]).contains(id))
        })
        .count()
}

pub fn compte_ingrédients_possibles(intervals_str: &str) -> usize {
    let mut intervals = lit_intervals(intervals_str);
    intervals.sort_by(|i1, i2| i1[0].cmp(&i2[0]));
    let mut debut = -1;
    let mut fin = -1;
    intervals
        .into_iter()
        .map(|interval| {
            if debut == -1 {
                debut = interval[0];
                fin = interval[1];
                return interval;
            }
            if interval[0] <= fin {
                if interval[1] <= fin {
                    return vec![0, -1];
                } else {
                    let nouveau_debut = fin + 1;
                    fin = interval[1];
                    return vec![nouveau_debut, fin];
                }
            }
            debut = interval[0];
            fin = interval[1];
            interval
        })
        .map(|interval| (interval[1] - interval[0] + 1) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_ingrédients_frais() {
        assert_eq!(
            compte_ingrédients_frais(inputs::EXEMPLE_INTERVALS, inputs::EXEMPLE_IDS),
            3
        );
        assert_eq!(
            compte_ingrédients_frais(inputs::INPUT_INTERVALS, inputs::INPUT_IDS),
            726
        );
    }

    #[test]
    fn il_compte_ingrédients_possibles() {
        assert_eq!(compte_ingrédients_possibles(inputs::EXEMPLE_INTERVALS), 14);
        assert_eq!(
            compte_ingrédients_possibles(inputs::INPUT_INTERVALS),
            354226555270043
        );
    }
}
