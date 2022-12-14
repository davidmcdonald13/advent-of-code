use std::cmp;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut rocks: HashSet<(i64, i64)> = HashSet::new();

    for line in lines {
        let vertices: Vec<(i64, i64)> = line.split(" -> ")
                                            .map(|pair| pair.split(","))
                                            .map(|mut split_pair| (split_pair.next().unwrap().parse::<i64>().unwrap(), split_pair.next().unwrap().parse::<i64>().unwrap()))
                                            .collect();

        let len: usize = vertices.len();

        for i in 0..(len-1) {
            let low_row: i64 = cmp::min(vertices[i].1, vertices[i+1].1);
            let high_row: i64 = cmp::max(vertices[i].1, vertices[i+1].1);

            let low_col: i64 = cmp::min(vertices[i].0, vertices[i+1].0);
            let high_col: i64 = cmp::max(vertices[i].0, vertices[i+1].0);

            for row in low_row..(high_row+1) {
                for col in low_col..(high_col+1) {
                    rocks.insert((row, col));
                }
            }
        }
    }

    let floor_row: i64 = rocks.iter().map(|pair| pair.0).max().unwrap() + 2;

    let mut sands: HashSet<(i64, i64)> = HashSet::new();

    loop {
        let mut row: i64 = 0;
        let mut col: i64 = 500;
        loop {
            if row + 1 == floor_row {
                sands.insert((row, col));
                break;
            }
            let below: (i64, i64) = (row + 1, col);
            if sands.contains(&below) || rocks.contains(&below) {
                let left: (i64, i64) = (row + 1, col - 1);
                if sands.contains(&left) || rocks.contains(&left) {
                    let right: (i64, i64) = (row + 1, col + 1);
                    if sands.contains(&right) || rocks.contains(&right) {
                        sands.insert((row, col));
                        break;
                    } else {
                        row += 1;
                        col += 1;
                    }
                } else {
                    row += 1;
                    col -= 1;
                }
            } else {
                row += 1;
            }
        }
        if sands.contains(&(0, 500)) {
            break;
        }
    }

    println!("result: {}", sands.len());
}
