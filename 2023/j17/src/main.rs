mod inputs;
use j17::find_minimum_heat_lost_path;
use j17::read;

fn main() {
    println!("{}", find_minimum_heat_lost_path(&read(inputs::INPUT)));
}