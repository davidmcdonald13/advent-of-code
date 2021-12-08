use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let targets: Vec<usize> = vec![2, 3, 4, 7];

    let mut result: usize = 0;
    for line in lines {
        let outputs: str::SplitAsciiWhitespace = line.split(" | ").last().unwrap().split_ascii_whitespace();
        result += outputs.filter(|output| targets.contains(&output.len())).count();
    }

    println!("result: {}", result);
}
