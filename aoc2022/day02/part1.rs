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

        let diff: i64 = modulo(player - opp, 3);
        result += player + 1; // shape score
        result += 3 + 3 * (diff & 1) - 3 * ((diff & 2) >> 1);
            // b00 => 3
            // b01 => 6
            // b10 => 0
    }

    println!("result: {}", result);
}

fn modulo(a: i64, b: i64) -> i64 {
    return ((a % b) + b) % b;
}
