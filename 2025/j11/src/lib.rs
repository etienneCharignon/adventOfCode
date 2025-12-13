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

pub fn compte_chemins(
    de: &str,
    vers: &str,
    map: &HashMap<String, Vec<String>>,
    chemin: &mut Vec<String>,
    visités: &mut HashMap<String, i64>,
) -> i64 {
    if visités.contains_key(de) {
        let n = visités[de];
        println!("déja visité : {n}");
        return n;
    }
    if de == vers {
        println!("visité : {chemin:?}");
        return 1;
    } else if de == "out" {
        return 0;
    }
    println!("{de} : {map:?}");
    map[de]
        .iter()
        .map(|appareil| {
            chemin.push(appareil.to_string());
            let n = compte_chemins(appareil, vers, map, &mut chemin.clone(), visités);
            visités.insert(appareil.to_string(), n);
            n
        })
        .sum()
}

pub fn compte_chemins_dac_fft(map: &HashMap<String, Vec<String>>) -> i64 {
    let svr_dac = compte_chemins(
        "svr",
        "dac",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    let svr_fft = compte_chemins(
        "svr",
        "fft",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    let dac_fft = compte_chemins(
        "dac",
        "fft",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    let fft_dac = compte_chemins(
        "fft",
        "dac",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    let fft_out = compte_chemins(
        "fft",
        "out",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    let dac_out = compte_chemins(
        "dac",
        "out",
        map,
        &mut Vec::new(),
        &mut HashMap::<String, i64>::new(),
    );
    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_compte_les_chemins() {
        assert_eq!(
            compte_chemins(
                "you",
                "out",
                &lit_entree(inputs::EXEMPLE),
                &mut Vec::new(),
                &mut HashMap::<String, i64>::new()
            ),
            5
        );
        assert_eq!(
            compte_chemins(
                "you",
                "out",
                &lit_entree(inputs::INPUT),
                &mut Vec::new(),
                &mut HashMap::<String, i64>::new()
            ),
            413
        );
        assert_eq!(
            compte_chemins(
                "dac",
                "fft",
                &lit_entree(inputs::EXEMPLE2),
                &mut Vec::new(),
                &mut HashMap::<String, i64>::new()
            ),
            0
        );
    }

    #[test]
    fn il_compte_les_chemins_avec_dac_et_fft() {
        assert_eq!(compte_chemins_dac_fft(&lit_entree(inputs::EXEMPLE2)), 2);
        assert_eq!(
            compte_chemins_dac_fft(&lit_entree(inputs::INPUT)),
            525518050323600
        );
    }
}
