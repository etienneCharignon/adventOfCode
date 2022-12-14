mod inputs;
use j14::read_world_2;
use j14::drop_all_sand;

fn main() {
    let mut world = read_world_2(inputs::INPUT);
    println!("{}", drop_all_sand(&mut world));
}