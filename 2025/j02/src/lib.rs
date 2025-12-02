mod inputs;

pub fn invalide(n: &str) -> bool {
    let chars: Vec<_> = n.chars().collect();
    if chars.len() % 2 != 0 {
        return false;
    }
    let milieu = chars.len() / 2;
    for i in 0..milieu {
        if chars[i] != chars[i + milieu] {
            return false;
        }
    }
    true
}

pub fn trouve_invalide(interval_str: &str) -> Vec<i64> {
    let interval: Vec<i64> = interval_str
        .split('-')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut invalides: Vec<i64> = vec![];
    for i in interval[0]..=interval[1] {
        let istr = i.to_string();
        if invalide(&istr) {
            invalides.push(i);
        }
    }
    invalides
}

pub fn somme(inputs: &str) -> i64 {
    inputs
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|interval| trouve_invalide(interval).iter().sum::<i64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_trouve_invalide_dans_un_interval() {
        assert_eq!(trouve_invalide("11-22"), vec![11, 22]);
        assert_eq!(trouve_invalide("95-115"), vec![99]);
        assert_eq!(trouve_invalide("1188511880-1188511890"), vec![1188511885]);
    }

    #[test]
    fn il_trouve_la_somme() {
        assert_eq!(somme(inputs::EXEMPLE), 1227775554);
        assert_eq!(somme(inputs::INPUT), 30608905813);
    }
}
