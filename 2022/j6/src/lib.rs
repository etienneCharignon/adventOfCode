use itertools::Itertools;
mod inputs;


fn all_unique(candidate: &str) -> bool {
    candidate.chars().unique().count() == candidate.len()
}

pub fn detect_first_of_size(buffer: &str, size: usize) -> (&str, usize) {
    for (i, _) in buffer.chars().enumerate() {
        let candidate = &buffer[i..i + size];
        if all_unique(candidate) {
            return (candidate, i + candidate.len())
        }
    }
    panic!("not found");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_solve_part_1() {
        assert_eq!(detect_first_of_size("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), ("jpqm", 7));
        assert_eq!(detect_first_of_size("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), ("vwbj", 5));
        assert_eq!(detect_first_of_size(inputs::INPUT, 4), ("bdjq", 1578));
    }

    #[test]
    fn it_solve_part_2() {
        assert_eq!(detect_first_of_size("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), ("qmgbljsphdztnv", 19));
        assert_eq!(detect_first_of_size("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), ("vbhsrlpgdmjqwf", 23));
        assert_eq!(detect_first_of_size(inputs::INPUT, 14), ("mdcbnwqgshpvfj", 2178));
    }
}
