use std::env;
use std::iter::zip;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: Vec<&str> = contents.lines().collect();

    let times: Vec<i64> = parse_numbers(lines[0]);
    let distances: Vec<i64> = parse_numbers(lines[1]);

    let mut result: i64 = 1;
    for (time, distance) in zip(times.into_iter(), distances.into_iter()) {
        let min: i64 = find_min_hold_time(time, distance);
        let max: i64 = find_max_hold_time(time, distance);

        result *= max - min + 1;
    }

    println!("result: {}", result);
}

fn parse_numbers(s: &str) -> Vec<i64> {
    return s.split_ascii_whitespace()
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
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
