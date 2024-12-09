mod inputs;


pub fn checksum(input: &str) -> usize {
    let mut checksum = 0;
    let diskmap: Vec<u64> = input.chars().map(|c| c.to_string().parse().expect("failed to parse number")).collect();
    let mut ongoing_i = 0;
    let mut backward_i = diskmap.len() - 1;
    let mut block_position = 0;
    let mut last_file_size = diskmap[backward_i];
    while ongoing_i < backward_i {
        println!("{ongoing_i} - {backward_i}");
        // file
        for _i in 0..diskmap[ongoing_i] {
            println!("{}: {} * {} (file)", block_position, (ongoing_i/2), block_position);
            checksum += (ongoing_i/2) * block_position;
            block_position += 1;
        }
        ongoing_i += 1;
        // empty zone
        let mut empty_size = diskmap[ongoing_i];
        while empty_size > 0 {
            if empty_size >= last_file_size {
                for _i in 0..last_file_size {
                    println!("{}: {} * {} (empty_size >=)", block_position, (backward_i/2), block_position);
                    checksum += (backward_i/2) * block_position;
                    block_position += 1;
                }
                empty_size -= last_file_size;
                backward_i -= 2;
                last_file_size = diskmap[backward_i];
            }
            else {
                for _i in 0..empty_size {
                    println!("{}: {} * {} (empty_size <)", block_position, (backward_i/2), block_position);
                    checksum += (backward_i/2) * block_position;
                    block_position += 1;
                }
                last_file_size -= empty_size;
                empty_size = 0;
            }
        }
        ongoing_i += 1;
    }
    if last_file_size > 0 {
        for _i in 0..last_file_size {
            println!("{}: {} * {} (empty_size >=)", block_position, (backward_i/2), block_position);
            checksum += (backward_i/2) * block_position;
            block_position += 1;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(checksum(inputs::EXAMPLE), 1928);
        assert_eq!(checksum(inputs::INPUT), 6334655979668);
    }
}
