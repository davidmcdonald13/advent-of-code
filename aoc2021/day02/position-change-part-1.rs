use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let moves: str::Lines = contents.lines();

    let mut depth_change: i64 = 0;
    let mut horiz_change: i64 = 0;

    for full_move in moves {
        let amount: i64 = full_move.split_ascii_whitespace().last().expect("failed to extract number").parse::<i64>().unwrap();
        if full_move.starts_with("forward") {
            horiz_change += amount;
        } else if full_move.starts_with("up") {
            depth_change -= amount;
        } else if full_move.starts_with("down") {
            depth_change += amount;
        }
    }

    println!("depth_change: {}", depth_change);
    println!("horiz_change: {}", horiz_change);
    println!("product: {}", depth_change * horiz_change);
}
