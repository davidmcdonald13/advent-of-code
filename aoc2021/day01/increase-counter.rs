use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let window_size: &usize = &args[2].parse::<usize>().unwrap();

    let depths: Vec<i64> = get_depths(filename);

    let mut prev_sum: i64 = 0;
    for i in 0..*window_size {
        prev_sum += &depths[i];
    }

    let mut result: i64 = 0;
    let mut tail = *window_size;
    while tail < depths.len() {
        let curr_sum = prev_sum + &depths[tail] - &depths[tail - *window_size];
        if curr_sum > prev_sum {
            result += 1;
        }
        prev_sum = curr_sum;
        tail += 1;
    }

    println!("result: {}", result);
}

fn get_depths(filename: &String) -> Vec<i64> {
    let contents: String = fs::read_to_string(filename).expect("failed to read file");

    let depth_strings: str::SplitAsciiWhitespace = contents.split_ascii_whitespace();
    let mut depths: Vec<i64> = Vec::new();
    for d in depth_strings {
        depths.push(d.parse::<i64>().unwrap());
    }

    return depths;
}
