mod inputs;
use j24::Position;
use j24::solve;
use j24::read_blizzards;

fn main() {
    let time = solve(Position(1, 0, 0), Position(120, 26, 0), read_blizzards(inputs::INPUT));
    println!("{:?}", time);
}