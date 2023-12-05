mod inputs;
use j5::find_minimum_location;
use j5::read;

fn main() {
    println!("{:?}", find_minimum_location(read(inputs::INPUT)));
}