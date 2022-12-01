use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let mut values: Vec<i64> = vec![0];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    for line in lines {
        if line.is_empty() {
            values.push(0);
        } else {
            let index: usize = values.len() - 1;
            values[index] += line.parse::<i64>().unwrap();
        }
    }

    values.sort_unstable();

    let result: i64 = values[values.len() - 1];
    println!("result: {}", result);
}
