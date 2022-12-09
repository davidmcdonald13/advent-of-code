use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut head_row: i64 = 0;
    let mut head_col: i64 = 0;
    let mut tail_row: i64 = 0;
    let mut tail_col: i64 = 0;

    visited.insert((0, 0));

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let count = tokens[1].parse::<i64>().unwrap();

        for _ in 0..count {
            match tokens[0] {
                "U" => head_row += 1,
                "D" => head_row -= 1,
                "L" => head_col -= 1,
                "R" => head_col += 1,
                _ => continue,
            }

            (tail_row, tail_col) = move_tail(head_row, head_col, tail_row, tail_col);

            visited.insert((tail_row, tail_col));
        }
    }

    println!("result: {}", visited.len());
}

fn move_tail(head_row: i64, head_col: i64, tail_row: i64, tail_col: i64) -> (i64, i64) {
    let row_diff = head_row - tail_row;
    let col_diff = head_col - tail_col;

    if row_diff.abs() <= 1 && col_diff.abs() <= 1 {
        // they are touching
        return (tail_row, tail_col);
    }

    let mut res_row: i64 = tail_row;
    let mut res_col: i64 = tail_col;

    if row_diff.abs() == 2 {
        res_row += row_diff >> 1;
        if col_diff.abs() == 1 {
            res_col += col_diff;
        }
    }

    if col_diff.abs() == 2 {
        res_col += col_diff >> 1;
        if row_diff.abs() == 1 {
            res_row += row_diff;
        }
    }

    return (res_row, res_col);
}
