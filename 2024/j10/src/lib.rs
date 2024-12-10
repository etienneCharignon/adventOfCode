mod inputs;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}


pub fn read(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let mut world = vec![];
    let lines: Vec<&str> = input.split('\n').collect();
    let height = lines.len();
    let width = lines[0].len();
    for line in lines {
        world.push(line.chars().map(|c| c.to_string().parse().expect("not a number")).collect());
    }
    (world, height, width)
}

pub fn find_heads(world: &Vec<Vec<u32>>) -> Vec<Pos> {
    let mut heads = vec![];
    for (r, row) in world.iter().enumerate() {
        for (c, &altitude) in row.iter().enumerate() {
            if altitude == 0 {
                heads.push(Pos {x : c as i32, y: r as i32});
            }
        }
    }
    heads
}

pub fn add(a: Pos, b: Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn outside_map(p : &Pos, height: i32, width: i32) -> bool {
    p.x <0 || p.x >= width || p.y < 0 || p.y >= height
}

pub fn head_score(head: Pos, world: &Vec<Vec<u32>>, height: i32, width: i32) -> usize {
    let current = world[head.y as usize][head.x as usize];
    if current == 9 {
        1
    }
    else {
        let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
        directions.iter().map(|&d| {
            let neighbour_pos = add(head, d);
            if !outside_map(&neighbour_pos, height, width)
                && world[neighbour_pos.y as usize][neighbour_pos.x as usize] == current + 1 {
                    head_score(neighbour_pos, world, height, width)
            }
            else {
                0
            }
        }).sum()
    }
}

pub fn score(input: &str) -> usize {
    let (world, height, width) = read(input);
    let trail_heads: Vec<Pos> = find_heads(&world);
    println!("{:?}", trail_heads);
    let mut total_score = 0;
    for head in trail_heads {
        total_score += head_score(head, &world, height as i32, width as i32);
    }

    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score(inputs::SIMPLE_EXAMPLE), 2);
        assert_eq!(score(inputs::EXAMPLE), 81);
        assert_eq!(score(inputs::INPUT), 1786);
    }
}
