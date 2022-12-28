mod inputs;
use j24::Position;
use j24::find_path;
use j24::read_blizzards;

fn main() {
    let time = find_path(Position(1, 0, 0), Position(120, 25, 0), read_blizzards(inputs::INPUT));
    println!("{}", time);
}