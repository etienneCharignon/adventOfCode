use std::collections::HashMap;
mod inputs;

fn read_line(commande: &str, pwd: String) -> String {
    let commande_line: Vec<&str> = commande.split(' ').collect();
    match commande_line[1] {
        "cd" => {
            match commande_line[2] {
                ".." => {
                    let mut dirs: Vec<&str> = pwd.split("/").collect();
                    dirs.pop();
                    if dirs.len() == 1 { return String::from("/"); }
                    dirs.join("/")
                },
                _ => pwd + commande_line[2],
            }
        }
        _ => panic!("not implemented")
    }
}

fn read_directory(lines: &Vec<&str>, mut i: usize, dirs: &mut HashMap<String, usize>, pwd: &String) -> (usize, usize) {
    let dir = read_line(lines[i], (&pwd).to_string());
    let mut size = 0;
    while i < lines.len() - 1 {
        i += 1;
        if lines[i].starts_with("$ ls") { continue; };
        if lines[i].starts_with("dir") { continue; };
        if lines[i].starts_with("$ cd ..") {
            break;
        }
        if lines[i].starts_with("$ cd") {
            let dir_data = read_directory(lines, i, dirs, &dir);
            size += dir_data.0;
            i = dir_data.1;
            continue;
        };
        size += lines[i].split(' ').next().unwrap().parse::<usize>().unwrap();
    }
    &dirs.insert(dir, size);
    (size, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_change_directory() {
        assert_eq!(read_line("$ cd /", String::from("")), String::from("/"));
        assert_eq!(read_line("$ cd a", String::from("/")), String::from("/a"));
        assert_eq!(read_line("$ cd a", String::from("/b")), String::from("/ba"));
        assert_eq!(read_line("$ cd ..", String::from("/b/a")), String::from("/b"));
        assert_eq!(read_line("$ cd ..", String::from("/b")), String::from("/"));
    }

    #[test]
    fn it_read_a_directory() {
        let mut directories: HashMap<String, usize> = HashMap::new();
        assert_eq!(read_directory(&("$ cd /".lines().collect()), 0, &mut directories, &String::new()), (0, 0));
        assert_eq!(directories, HashMap::from([(String::from("/"), 0)]));
    }

    #[test]
    fn it_read_a_directory_content() {
        let mut directories: HashMap<String, usize> = HashMap::new();
        assert_eq!(read_directory(&("$ cd /
14848514 b.txt
2 c.txt
dir c".lines().collect()), 0, &mut directories, &String::new()), (14848516, 3));
        assert_eq!(directories, HashMap::from([(String::from("/"), 14848516)]));
    }

    #[test]
    fn it_read_a_directory_with_sub_directory() {
        let mut directories: HashMap<String, usize> = HashMap::new();
        assert_eq!(read_directory(&("$ cd /
$ ls
14848514 b.txt
2 c.txt
dir c
$ cd c
$ ls
1 t
$ cd ..
$ cd d
$ ls
1 n".lines().collect()), 0, &mut directories, &String::new()), (14848518, 11));
        assert_eq!(directories, HashMap::from([
            (String::from("/"), 14848518),
            (String::from("/c"), 1),
            (String::from("/d"), 1),
        ]));
    }


    #[test]
    fn it_read_example() {
        let mut directories: HashMap<String, usize> = HashMap::new();
        read_directory(&(inputs::EXAMPLE.lines().collect()), 0, &mut directories, &String::new());
        let total: u32 = *directories.get("/").unwrap() as u32;
        assert_eq!(total, 48381165);
        let wanted: u32 = 30000000;
        let unused: u32 = 70000000 - total;
        assert_eq!(unused, 21618835);
        let needed: u32 = wanted - unused;
        assert_eq!(needed, 8381165);
        let mut min: u32 = std::u32::MAX;
        for value in directories.values() {
            let value32 = *value as u32; 
            if value32 >= needed && value32 < min {
                min = value32;
            }
        }
        assert_eq!(min, 24933642);
    }

    #[test]
    fn it_read_input() {
        let mut directories: HashMap<String, usize> = HashMap::new();
        read_directory(&(inputs::INPUT.lines().collect()), 0, &mut directories, &String::new());
        let total: u32 = *directories.get("/").unwrap() as u32;
        let wanted: u32 = 30000000;
        let unused: u32 = 70000000 - total;
        let needed: u32 = wanted - unused;
        let mut min: u32 = std::u32::MAX;
        for value in directories.values() {
            let value32 = *value as u32; 
            if value32 >= needed && value32 < min {
                min = value32;
            }
        }
        assert_eq!(min, 6296435);
    }
}
