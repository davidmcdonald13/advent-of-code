use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let chars: Vec<HashSet<char>> = lines.map(|s| get_chars(&s)).collect();

    let line_count = chars.len();

    let mut result: i64 = 0;

    for i in (0..line_count).step_by(3) {
        for c in chars[i].iter() {
            if chars[i+1].contains(&c) && chars[i+2].contains(&c) {
                let c_pt: i64 = *c as i64;

                if c_pt >= ('a' as i64) {
                    result += c_pt - ('a' as i64) + 1;
                } else {
                    result += c_pt - ('A' as i64) + 27;
                }
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
