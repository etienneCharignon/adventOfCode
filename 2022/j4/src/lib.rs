mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    fn read_line(line: &str) -> Vec<Vec<u8>> {
        line.split(',')
            .map(|assignment| assignment.split('-')
                                        .map(|id| id.parse::<u8>().unwrap())
                                        .collect())
            .collect() 
    }

    fn full_overlap(assignments: &Vec<Vec<u8>>) -> bool {
        assignments[0][0] >= assignments[1][0] && assignments[0][1] <= assignments[1][1] ||
        assignments[0][0] <= assignments[1][0] && assignments[0][1] >= assignments[1][1]
    }

    fn do_not_overlap(assignments: &Vec<Vec<u8>>) -> bool {
        assignments[0][1] < assignments[1][0] || assignments[1][1] < assignments[0][0]
    }

    fn count_overlapped(pair_assignments: &str) -> usize {
        pair_assignments.lines()
                        .map(|assignments| read_line(assignments))
                        .filter(|assignments| ! do_not_overlap(assignments))
                        .count()
    }

    #[test]
    fn it_read_line() {
        assert_eq!(read_line("2-4,6-8"), vec!(vec!(2, 4), vec!(6, 8)));
    }

    #[test]
    fn it_count_overlapped() {
        assert_eq!(count_overlapped(inputs::EXAMPLE), 4);
        assert_eq!(count_overlapped(inputs::INPUT), 911);
    }
}
