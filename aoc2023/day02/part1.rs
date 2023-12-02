use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let result: usize =
        lines.enumerate()
             .map(|(i, line)| {
                 let invalid_values: usize =
                     line.split(&[':', ',', ';'][..])
                         .skip(1)
                         .filter(|s| {
                             let tokens: Vec<&str> = s.trim().split(' ').collect();
                             let count: i64 = tokens.get(0).unwrap().parse::<i64>().unwrap();
                             match tokens.get(1).unwrap() {
                                 &"red" => return count > 12,
                                 &"blue" => return count > 14,
                                 &"green" => return count > 13,
                                 _ => return false,
                             }
                         })
                         .count();
                if invalid_values == 0 {
                    return i + 1; // game ID is not zero-indexed
                } else {
                    return 0;
                }
             })
             .sum();

    println!("result: {}", result);
}
