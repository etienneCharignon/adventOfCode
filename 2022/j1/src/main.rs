use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let mut elves = Vec::new();
    elves.push(0);
    for line in content.lines() {
        if line.is_empty() {
            elves.push(0);
        }
        else {
            let elf = elves.pop();
            elves.push(elf.unwrap() + line.parse::<i32>().unwrap());
        }
    }
    elves.sort();
    let max1 = elves.pop().unwrap();
    let max2 = elves.pop().unwrap();
    let max3 = elves.pop().unwrap();
    println!("max = {max1}");
    println!("max = {max2}");
    println!("max = {max3}");
    let total = max1 + max2 + max3;
    println!("total = {total}");
}
