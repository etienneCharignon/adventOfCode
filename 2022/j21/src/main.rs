mod inputs;
use j21::find_yell;
use inputs::calculate;

fn main() {
    let yell = find_yell("lvvf", "rqgq");
    println!("{}", yell);
    println!("{}", calculate(yell-1, "root"));
}