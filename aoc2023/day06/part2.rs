use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: Vec<&str> = contents.lines().collect();

    let time: i64 = parse_numbers(lines[0]);
    let distance: i64 = parse_numbers(lines[1]);

    let min: i64 = find_min_hold_time(time, distance);
    let max: i64 = find_max_hold_time(time, distance);

    let result = max - min + 1;

    println!("result: {}", result);
}

fn parse_numbers(s: &str) -> i64 {
    return s.split(':')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<i64>()
            .unwrap();
}

fn find_extreme_hold_time(time: i64, distance: i64, reverse: bool) -> i64 {
    let start;
    let end;

    if reverse {
        start = time;
        end = 0;
    } else {
        start = 0;
        end = time;
    }

    let mut result = start;

    while result != end {
        if result * (time - result) > distance {
            return result;
        }
        if reverse {
            result -= 1;
        } else {
            result += 1;
        }
    }
    return i64::MIN;
}

fn find_min_hold_time(time: i64, distance: i64) -> i64 {
    return find_extreme_hold_time(time, distance, false);
}

fn find_max_hold_time(time: i64, distance: i64) -> i64 {
    return find_extreme_hold_time(time, distance, true);
}
