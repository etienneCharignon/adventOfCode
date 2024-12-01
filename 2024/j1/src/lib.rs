mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = inputs::INPUT.split('\n');
        let mut list1: Vec<i32> = vec![];
        let mut list2: Vec<i32> = vec![];
        for line in lines {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        }
        list1.sort();
        list2.sort();

        let mut distance = 0;
        for (index, n1) in list1.iter().enumerate() {
            let d = (n1 - list2[index]).abs();
            println!("{d}");
            distance = distance + d;
        }


        assert_eq!(distance, 3574690);
    }
}
