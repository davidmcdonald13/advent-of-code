use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let result: i64 = lines.map(|line| get_match_count(line))
                             .map(|count| {
                                 if count == 0 {
                                     return 0;
                                 } else {
                                     return 2_i64.pow((count as u32) - 1);
                                 }
                             })
                             .sum();

    println!("result: {}", result);
}

fn get_match_count(line: &str) -> i64 {
    let chunks: Vec<&str> = line.split(&[':', '|'][..]).collect();

    let winners: HashSet<i64> = parse_integers(chunks[1]).collect();
    return parse_integers(chunks[2]).map(|val| winners.contains(&val) as i64).sum();
}

fn parse_integers(s: &str) -> impl Iterator<Item=i64> + '_ {
    return s.split_ascii_whitespace()
            .map(|token| token.parse::<i64>())
            .filter_map(|result| result.ok());
}
