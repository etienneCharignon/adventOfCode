mod inputs;

pub fn compte_ingrédients_frais(intervals_str: &str, ids_str: &str) -> usize {
    let intervals = intervals_str
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
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
}
