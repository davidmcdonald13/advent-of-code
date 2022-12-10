use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut values: Vec<i64> = vec![1];

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let prev: i64 = *values.last().unwrap();

        values.push(prev);
        if tokens[0] == "addx" {
            let val = tokens[1].parse::<i64>().unwrap();
            values.push(prev + val);
        }
    }

    let timestamps: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut result: i64 = 0;

    for time in timestamps {
        result += (time as i64) * values[time - 1];
    }

    println!("result: {}", result);
}
