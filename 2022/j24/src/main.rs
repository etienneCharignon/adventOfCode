mod inputs;
use j24::Position;
use j24::solve;
use j24::read_blizzards;

fn main() {
    let time = solve(Position(120, 26, 314), Position(1, 0, i32::MAX), read_blizzards(inputs::INPUT));
    println!("{:?}", time);
}