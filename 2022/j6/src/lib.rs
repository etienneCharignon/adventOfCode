use itertools::Itertools;
mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    fn all_unique(candidate: &str) -> bool {
        candidate.chars().unique().count() == 4
    }

    fn first_start(buffer: &str) -> (&str, usize) {
        for (i, _) in buffer.chars().enumerate() {
            let candidate = &buffer[i..i + 4];
            if all_unique(candidate) {
                return (candidate, i + 4)
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
        assert_eq!(first_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), ("jpqm", 7));
        assert_eq!(first_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), ("vwbj", 5));
        assert_eq!(first_start(inputs::INPUT), ("bdjq", 1578));
    }
}
