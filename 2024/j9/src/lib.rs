mod inputs;


#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct File {
    pub id: usize,
    pub size: u64
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Free {
    pub size: u64
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum Segment {
    File(File),
    Free(Free)
}

pub fn checksum_d1(input: &str) -> usize {
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

pub fn find_index_of_first_free_space(diskmap: &Vec<Segment>, file: &File) -> usize {
    for (index, segment) in diskmap.into_iter().enumerate() {
        match segment {
            Segment::File(s) => {
                if s.id == file.id {
                    return 0;
                }
            },
            Segment::Free(s) => { 
                if s.size >= file.size {
                    return index;
                }
            }
        }
    }
    0
}

pub fn checksum_d2(input: &str) -> usize {
    let data: Vec<u64> = input.chars().map(|c| c.to_string().parse().expect("failed to parse number")).collect();
    let mut diskmap: Vec<Segment> = vec![];
    let mut files: Vec<File> = vec![];
    for (index, f) in data.into_iter().enumerate() {
        if index % 2 == 0 {
            let file = File { id: index/2, size: f };
            diskmap.push(Segment::File(file));
            files.push(file);
        }
        else {
            diskmap.push(Segment::Free(Free { size: f }));
        }
    }

    for file in files.iter().rev() {
        let i = find_index_of_first_free_space(&diskmap, &file);
        println!("first free space : {i} : {:?}", file);
        if i > 0 {
            let Segment::Free(free) = diskmap.remove(i) else { panic!("i must be a free space") };
            let from = diskmap.iter().position(|&segment| {
                match segment {
                    Segment::File(s) => { s.id == file.id },
                    Segment::Free(_) => { false }
                }
            }).unwrap();
            diskmap[from] = Segment::Free(Free { size: file.size });
            diskmap.insert(i, Segment::File(*file));
            let remaining_space = free.size - file.size;
            if remaining_space > 0 {
                diskmap.insert(i + 1, Segment::Free(Free { size: remaining_space }));
            }
        }
    }
    println!("===========");

    let mut checksum = 0;
    let mut block_position = 0;
    for s in diskmap {
        match s {
            Segment::Free(s) => { block_position += s.size },
            Segment::File(s) => { 
                for _ in 0..s.size {
                    checksum += s.id * block_position as usize;
                    block_position += 1;
                }
            }
        }
        println!("segment : {:?}, checksum {}, block_position {block_position}", s, checksum);
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checksum_d1() {
        assert_eq!(checksum_d1(inputs::EXAMPLE), 1928);
        assert_eq!(checksum_d1(inputs::INPUT), 6334655979668);
    }

    #[test]
    fn it_checksum_j2() {
        assert_eq!(checksum_d2(inputs::EXAMPLE), 2858);
        assert_eq!(checksum_d2(inputs::INPUT), 6349492251099);
    }
}
