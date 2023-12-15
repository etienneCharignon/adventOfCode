mod inputs;

pub fn hash(string: &str) -> usize {
    string.chars().fold(0, |hash, c| {
        let mut new_hash = hash + c as usize;
        new_hash *= 17;
        new_hash %= 256;
        new_hash
    })
}

pub fn solve(input: &str) -> usize {
    input.split(",").map(|c| hash(c)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn it_solve_example() {
        assert_eq!(solve(inputs::EXAMPLE), 1320);
        assert_eq!(solve(inputs::INPUT), 519041);
    }
}
