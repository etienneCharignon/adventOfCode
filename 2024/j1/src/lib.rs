mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = inputs::INPUT.split('\n');
        let mut list1: Vec<usize> = vec![];
        let mut list2: Vec<usize> = vec![];
        for line in lines {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        }
        list1.sort();
        list2.sort();

        let mut similarity = 0;
        for n1 in list1.iter() {
            let s = n1 * list2.iter().filter(|&n| *n == *n1).count();
            println!("{s}");
            similarity = similarity + s;
        }


        assert_eq!(similarity, 22565391);
    }
}
