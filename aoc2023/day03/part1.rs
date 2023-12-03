use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let lines_vec: Vec<&str> = lines.collect();

    let symbols: HashSet<(i64, i64)> =
        lines_vec.iter().enumerate()
             .flat_map(|(row_idx, line)|
                 line.chars()
                     .enumerate()
                     .filter(|(_col_idx, val)| !val.is_numeric() && val != &'.')
                     .map(move |(col_idx, _)| (row_idx as i64, col_idx as i64))
             ).collect();

    let result: usize =
        lines_vec.iter().enumerate()
             .map(|(row_idx, line)| sum_part_numbers(row_idx as i64, line, &symbols))
              .sum();

    println!("result: {}", result);
}

fn sum_part_numbers(row_idx: i64, line: &str, symbols: &HashSet<(i64, i64)>) -> usize {
    let line_vec: Vec<char> = line.chars().collect();

    let len: usize = line_vec.len();

    let mut result: usize = 0;

    let mut running: usize = 0;
    let mut is_adjacent: bool = false;

    for col_idx in 0..len {
        let val: char = line_vec[col_idx];
        if val.is_numeric() {
            running = running * 10 + val.to_digit(10).unwrap() as usize;

            if !is_adjacent {
                is_adjacent = check_for_adjacency(row_idx, col_idx as i64, symbols);
            }
        } else {
            if is_adjacent {
                result += running;
            }
            running = 0;
            is_adjacent = false;
        }
    }

    if is_adjacent {
        result += running;
    }

    return result;
}

fn check_for_adjacency(row_idx: i64, col_idx: i64, symbols: &HashSet<(i64, i64)>) -> bool {
    for row in row_idx-1..row_idx+2 {
        for col in col_idx-1..col_idx+2 {
            if symbols.contains(&(row, col)) {
                return true;
            }
        }
    }
    return false;
}
