mod inputs;

pub fn invalide(n: &str, multiple: usize) -> bool {
    println!("{multiple}");
    let chars: Vec<_> = n.chars().collect();
    if chars.len() % multiple != 0 {
        return false;
    }
    let sections: Vec<&str> = n
        .as_bytes()
        .chunks(chars.len() / multiple)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    let premier = sections[0];
    sections.iter().all(|s| *s == premier)
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
        for d in 2..=istr.len() {
            if invalide(&istr, d) {
                invalides.push(i);
                break;
            }
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
        assert_eq!(trouve_invalide("95-115"), vec![99, 111]);
        assert_eq!(trouve_invalide("1188511880-1188511890"), vec![1188511885]);
    }

    #[test]
    fn il_trouve_la_somme() {
        assert_eq!(somme(inputs::EXEMPLE), 4174379265);
        assert_eq!(somme(inputs::INPUT), 31898925685);
    }
}
