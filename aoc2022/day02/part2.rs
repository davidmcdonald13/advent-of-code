use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut result: i64 = 0;

    for line in lines {
        let opp: i64 = (line.chars().nth(0).unwrap() as i64) - ('A' as i64);
        let player: i64 = (line.chars().nth(2).unwrap() as i64) - ('X' as i64);

        // Trust me, this works.
        result += modulo(opp + player - 1, 3) + 1;
        result += 3 * player;
    }

    println!("result: {}", result);
}

fn modulo(a: i64, b: i64) -> i64 {
    return ((a % b) + b) % b;
}
