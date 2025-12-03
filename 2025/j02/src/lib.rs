mod inputs;

pub fn invalide(n: &str, multiple: usize) -> bool {
    println!("{multiple}");
    if !n.len().is_multiple_of(multiple) {
        return false;
    }
    let taille = n.len() / multiple;
    n.as_bytes()
        .chunks(taille)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .all(|s| *s == n[..taille])
}

pub fn trouve_invalides(interval_str: &str) -> Vec<i64> {
    let interval: Vec<i64> = interval_str
        .split('-')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    (interval[0]..=interval[1])
        .filter(|i| {
            let istr = i.to_string();
            for d in 2..=istr.len() {
                if invalide(&istr, d) {
                    return true;
                }
            }
            false
        })
        .collect()
}

pub fn somme(inputs: &str) -> i64 {
    inputs
        .split(',')
        .map(|interval| trouve_invalides(interval).iter().sum::<i64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_trouve_invalide_dans_un_interval() {
        assert_eq!(trouve_invalides("11-22"), vec![11, 22]);
        assert_eq!(trouve_invalides("95-115"), vec![99, 111]);
        assert_eq!(trouve_invalides("1188511880-1188511890"), vec![1188511885]);
    }

    #[test]
    fn il_trouve_la_somme() {
        assert_eq!(somme(inputs::EXEMPLE), 4174379265);
        assert_eq!(somme(inputs::INPUT), 31898925685);
    }
}
