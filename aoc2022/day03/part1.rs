use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut result: i64 = 0;

    for line in lines {
        let length: usize = line.len();

        let (a, b) = line.split_at(length/2);

        let comp1: HashSet<char> = get_chars(a);
        let comp2: HashSet<char> = get_chars(b);

        let shared = comp1.intersection(&comp2);

        for c in shared {
            let c_pt: i64 = *c as i64;

            if c_pt >= ('a' as i64) {
                result += c_pt - ('a' as i64) + 1;
            } else {
                result += c_pt - ('A' as i64) + 27;
            }
        }
    }

    println!("result: {}", result);
}

fn get_chars(s: &str) -> HashSet<char> {
    let mut result: HashSet<char> = HashSet::new();

    for c in s.chars() {
        result.insert(c);
    }

    return result;
}
