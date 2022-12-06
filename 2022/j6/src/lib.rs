use itertools::Itertools;
mod inputs;

const LEN: usize = 14;

#[cfg(test)]
mod tests {
    use super::*;

    fn all_unique(candidate: &str) -> bool {
        candidate.chars().unique().count() == LEN
    }

    fn first_start(buffer: &str) -> (&str, usize) {
        for (i, _) in buffer.chars().enumerate() {
            let candidate = &buffer[i..i + LEN];
            if all_unique(candidate) {
                return (candidate, i + LEN)
            }
        }
        //     if start.contains(&c) {
        //         start = Vec::new();
        //     }
        //     start.push(c);
        //     if start.len() == 4 {
        //         return (start, i + 1)
        //     }
        // }
        panic!("start not found");
    }

    #[test]
    fn it_detect_first_start() {
        assert_eq!(first_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), ("qmgbljsphdztnv", 19));
        assert_eq!(first_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), ("vbhsrlpgdmjqwf", 23));
        assert_eq!(first_start(inputs::INPUT), ("mdcbnwqgshpvfj", 2178));
    }
}
