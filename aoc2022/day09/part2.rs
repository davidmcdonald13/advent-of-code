use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let knot_count = 10;
    let mut knots: Vec<(i64, i64)> = vec![(0, 0); knot_count];

    visited.insert(knots[knot_count - 1]);

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let count = tokens[1].parse::<i64>().unwrap();

        for _ in 0..count {
            let (mut head_row, mut head_col) = knots[0];
            match tokens[0] {
                "U" => head_row += 1,
                "D" => head_row -= 1,
                "L" => head_col -= 1,
                "R" => head_col += 1,
                _ => continue,
            }

            knots[0] = (head_row, head_col);

            for i in 1..knot_count {
                knots[i] = move_tail(knots[i-1], knots[i]);
            }

            let tail = knots[knot_count - 1];
            visited.insert(tail);
        }
    }

    println!("result: {}", visited.len());
}

fn move_tail(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
    let (head_row, head_col) = head;
    let (tail_row, tail_col) = tail;

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
