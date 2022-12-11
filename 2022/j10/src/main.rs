mod inputs;
use j10::compute_signal;

fn main() {
    let mut screen = String::from("");
    compute_signal(inputs::INPUT, &mut screen);
    println!("{}", screen);
}