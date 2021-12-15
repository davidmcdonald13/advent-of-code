use std::cmp;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let risks: Vec<Vec<usize>> = lines.map(|line| line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()).collect();

    let mut result: Vec<Vec<usize>> = vec![vec![usize::MAX; risks[0].len()]; risks.len()];

    for i in 0..risks.len() {
        for j in 0..risks[0].len() {
            let row = risks.len() - 1 - i;
            let col = risks[0].len() - 1 - j;

            if i == 0 && j == 0 {
                result[row][col] = risks[row][col];
            } else if i == 0 {
                result[row][col] = result[row][col + 1] + risks[row][col];
            } else if j == 0 {
                result[row][col] = result[row + 1][col] + risks[row][col];
            } else {
                result[row][col] = risks[row][col] + cmp::min(result[row + 1][col], result[row][col + 1]);
            }
        }
    }

    println!("risk: {}", result[0][0] - risks[0][0]);
}
