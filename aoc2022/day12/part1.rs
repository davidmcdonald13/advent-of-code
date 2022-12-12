use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let array: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let start: (usize, usize) = get_coords(&array, 'S');
    let end: (usize, usize) = get_coords(&array, 'E');

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut distances: Vec<Vec<usize>> = array.iter().map(|row| row.iter().map(|_x| usize::MAX).collect()).collect();

    while !queue.is_empty() {
        let (row, col, dist) = queue.pop_front().unwrap();

        if (row, col) == end {
            println!("result: {}", dist);
            return;
        }

        if distances[row][col] <= dist {
            continue;
        }

        distances[row][col] = dist;

        if row > 0 && is_valid_step(&array, row, col, row - 1, col) {
            queue.push_back((row - 1, col, dist + 1));
        }
        if row < (array.len() - 1) && is_valid_step(&array, row, col, row + 1, col) {
            queue.push_back((row + 1, col, dist + 1));
        }
        if col > 0 && is_valid_step(&array, row, col, row, col - 1) {
            queue.push_back((row, col - 1, dist + 1));
        }
        if col < (array[0].len() - 1) && is_valid_step(&array, row, col, row, col + 1) {
            queue.push_back((row, col + 1, dist + 1));
        }
    }
}

fn is_valid_step(array: &Vec<Vec<char>>, src_row: usize, src_col: usize, dest_row: usize, dest_col: usize) -> bool {
    let src_char = array[src_row][src_col];
    let dest_char = array[dest_row][dest_col];

    if src_char == 'S' {
        return dest_char == 'a' || dest_char == 'b';
    }
    if dest_char == 'E' {
        return src_char == 'y' || src_char == 'z';
    }

    let src_val = src_char as i64;
    let dest_val = dest_char as i64;

    return src_val >= dest_val || src_val + 1 == dest_val;
}

fn get_coords(arr: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    let rows = arr.len();
    let cols = arr[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if arr[row][col] == target {
                return (row, col);
            }
        }
    }
    return (usize::MAX, usize::MAX);
}
