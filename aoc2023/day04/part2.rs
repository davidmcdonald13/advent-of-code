use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: Vec<&str> = contents.lines().collect();

    let num_cards: usize = lines.len();

    let correct_counts: Vec<usize> = lines.into_iter()
                                        .map(|line| get_match_count(line))
                                        .collect();
    let mut card_counts: Vec<usize> = vec![1; num_cards];

    for i in 0..num_cards {
        let correct: usize = correct_counts[i];
        let new_cards: usize = card_counts[i];

        for j in (i+1)..(i+1+correct) {
            if j < num_cards {
                card_counts[j] += new_cards;
            }
        }
    }

    let result: usize = card_counts.into_iter().sum();

    println!("result: {}", result);
}

fn get_match_count(line: &str) -> usize {
    let chunks: Vec<&str> = line.split(&[':', '|'][..]).collect();

    let winners: HashSet<usize> = parse_integers(chunks[1]).collect();
    return parse_integers(chunks[2]).map(|val| winners.contains(&val) as usize).sum();
}

fn parse_integers(s: &str) -> impl Iterator<Item=usize> + '_ {
    return s.split_ascii_whitespace()
            .map(|token| token.parse::<usize>())
            .filter_map(|result| result.ok());
}
