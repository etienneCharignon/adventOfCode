mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = inputs::INPUT.split('\n');
        let (mut list1, mut list2): (Vec<usize>, Vec<usize>) = lines.into_iter().map(|line| {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            (numbers[0], numbers[1])
        }).unzip();

        list1.sort();
        list2.sort();


        let similarity: usize = list1.iter().map(|n1|
            *n1 * list2.iter().filter(|&n| *n == *n1).count()
            )
            .sum();


        assert_eq!(similarity, 22565391);
    }
}
