use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let mut lines: str::Lines = contents.lines();

    let mut template: Vec<char> = lines.next().unwrap().chars().collect();

    let mut mapping: HashMap<(char, char), char> = HashMap::new();

    lines.next();
    for line in lines {
        let mut chars = line.chars();

        let first: char = chars.next().unwrap();
        let second: char = chars.next().unwrap();
        let result: char = chars.last().unwrap();

        mapping.insert((first, second), result);
    }

    for _step in 0..10 {
        let mut i: usize = 1;
        while i < template.len() {
            template.insert(i, *mapping.get(&(template[i-1], template[i])).unwrap());
            i += 2;
        }
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in template {
        if counts.contains_key(&c) {
            counts.insert(c, counts.get(&c).unwrap() + 1);
        } else {
            counts.insert(c, 1);
        }
    }

    let mut max: usize = usize::MIN;
    let mut min: usize = usize::MAX;
    for count in counts.values() {
        max = cmp::max(max, *count);
        min = cmp::min(min, *count);
    }

    println!("max-min difference: {}", max - min);
}
