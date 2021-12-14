use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let moves: usize = args[2].parse::<usize>().unwrap();

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let mut lines: str::Lines = contents.lines();

    let mut polymers: HashMap<(char, char), usize> = HashMap::new();
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    for i in 1..template.len() {
        let polymer: (char, char) = (template[i-1], template[i]);
        polymers.insert(polymer, polymers.get(&polymer).unwrap_or(&0) + 1);
    }

    let mut mapping: HashMap<(char, char), char> = HashMap::new();

    lines.next();
    for line in lines {
        let mut chars = line.chars();

        let first: char = chars.next().unwrap();
        let second: char = chars.next().unwrap();
        let result: char = chars.last().unwrap();

        mapping.insert((first, second), result);
    }

    for _step in 0..moves {
        let mut new_polymers: HashMap<(char, char), usize> = HashMap::new();
        for (key, value) in polymers {
            let middle: char = *mapping.get(&key).unwrap();

            let left: (char, char) = (key.0, middle);
            let right: (char, char) = (middle, key.1);

            new_polymers.insert(left, new_polymers.get(&left).unwrap_or(&0) + value);
            new_polymers.insert(right, new_polymers.get(&right).unwrap_or(&0) + value);
        }

        polymers = new_polymers;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    counts.insert(template[0], 1);
    counts.insert(*template.last().unwrap(), 1);
    for (key, value) in polymers {
        counts.insert(key.0, counts.get(&key.0).unwrap_or(&0) + value);
        counts.insert(key.1, counts.get(&key.1).unwrap_or(&0) + value);
    }




    let mut max: usize = usize::MIN;
    let mut min: usize = usize::MAX;
    for count in counts.values() {
        max = cmp::max(max, *count);
        min = cmp::min(min, *count);
    }

    println!("max-min difference: {}", (max - min) / 2);
}
