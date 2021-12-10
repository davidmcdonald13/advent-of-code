use std::collections::HashMap;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let mut matches: HashMap<char, char> = HashMap::new();
    matches.insert(')', '(');
    matches.insert(']', '[');
    matches.insert('}', '{');
    matches.insert('>', '<');

    let mut corruption_scores: HashMap<char, usize> = HashMap::new();
    corruption_scores.insert(')', 3);
    corruption_scores.insert(']', 57);
    corruption_scores.insert('}', 1197);
    corruption_scores.insert('>', 25137);

    let mut comp_scores: HashMap<char, usize> = HashMap::new();
    comp_scores.insert('(', 1);
    comp_scores.insert('[', 2);
    comp_scores.insert('{', 3);
    comp_scores.insert('<', 4);

    let mut corrupted_score: usize = 0;
    let mut completion_scores: Vec<usize> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();

        let mut corrupted = false;
        for c in line.chars() {
            if matches.contains_key(&c) {
                if stack.len() == 0 || stack.pop().unwrap() != matches[&c] {
                    corrupted_score += corruption_scores[&c];
                    corrupted = true;
                    break;
                }
            } else {
                stack.push(c);
            }
        }

        if !corrupted {
            let mut completion_score: usize = 0;
            while stack.len() > 0 {
                completion_score = completion_score * 5 + comp_scores[&stack.pop().unwrap()];
            }
            completion_scores.push(completion_score);
        }
    }

    println!("corrupted_score: {}", corrupted_score);

    completion_scores.sort();
    println!("completion_score: {}", completion_scores.get(completion_scores.len() / 2).unwrap());
}
