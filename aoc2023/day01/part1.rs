use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let result: i64 = lines.map(|line| combine_first_last_digit(line)).sum();

    println!("result: {}", result);
}

fn combine_first_last_digit(line: &str) -> i64 {
    let first_index: usize = line.find(char::is_numeric).unwrap();
    let last_index: usize = line.rfind(char::is_numeric).unwrap();

    return 10 * line.get(first_index..first_index+1).unwrap().parse::<i64>().unwrap()
        + line.get(last_index..last_index+1).unwrap().parse::<i64>().unwrap();
}
