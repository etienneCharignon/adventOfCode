mod inputs;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn insert_pair(computer1: String, computer2:String, pairs: &mut HashMap<String, HashSet<String>>) {
    if let Some(connexions) = pairs.get(&computer1) {
        let mut new_connexions = connexions.clone();
        new_connexions.insert(computer2);
        pairs.insert(computer1, new_connexions);
    }
    else {
        pairs.insert(computer1, HashSet::from([computer2]));
    }
}

pub fn count_t_computers(input: &str) -> usize {
    let mut pairs = HashMap::<String, HashSet<String>>::new();
    for pair in input.split('\n') {
        let computers: Vec<&str> = pair.split('-').collect();
        insert_pair(computers[0].to_string(), computers[1].to_string(), &mut pairs);
        insert_pair(computers[1].to_string(), computers[0].to_string(), &mut pairs);
    }
    println!("{pairs:?}");
    let mut clusters: HashSet<Vec<&str>> = HashSet::new();
    for pair in input.split('\n') {
        let computers: Vec<&str> = pair.split('-').collect();
        let connexions1 = pairs.get(computers[1]).unwrap();
        for connexion in connexions1 {
            if pairs.get(connexion).unwrap().contains(computers[0]) {
                let mut cluster = vec![computers[0], computers[1], connexion];
                cluster.sort();
                clusters.insert(cluster);
            }
        }
    }
    println!("\n{clusters:?}");
    clusters.iter().filter(|cluster| cluster.iter().any(|c| c.starts_with('t'))).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_t_computers(inputs::EXAMPLE), 7);
        assert_eq!(count_t_computers(inputs::INPUT), 1077);
    }
}
