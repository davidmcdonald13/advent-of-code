use std::fs;
use std::str;

fn main() {
    let filename = "depths.txt";

    let contents: String = fs::read_to_string(filename).expect("failed to read file");

    let mut depths: str::SplitAsciiWhitespace = contents.split_ascii_whitespace();

    let mut prev: i32 = depths.next().expect("failed to get value").parse::<i32>().unwrap();

    let mut result: i32 = 0;

    for val in depths {
        let curr = val.parse::<i32>().unwrap();
        if curr > prev {
            result += 1;
        }
        prev = curr;
    }

    println!("result: {}", result);
}
