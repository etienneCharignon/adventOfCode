use std::collections::HashMap;

mod inputs;

pub fn lit_entree(entree: &str) -> HashMap<String, Vec<String>> {
    entree
        .lines()
        .fold(HashMap::<String, Vec<String>>::new(), |mut map, l| {
            let jettons = l.split_whitespace().map(String::from).collect::<Vec<_>>();
            let (premier, vers) = jettons.split_first().unwrap();
            let mut de = premier.to_string();
            de.pop();
            map.insert(de, vers.to_vec());
            map
        })
}

pub fn compt_chemins(de: &str, map: &HashMap<String, Vec<String>>) -> i64 {
    if de == "out" {
        return 1;
    }
    map[de]
        .iter()
        .map(|appareil| compt_chemins(appareil, map))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_les_chemins() {
        assert_eq!(compt_chemins("you", &lit_entree(inputs::EXEMPLE)), 5);
        assert_eq!(compt_chemins("you", &lit_entree(inputs::INPUT)), 413);
    }
}
