use priority_queue::PriorityQueue;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let tile_multiplier: usize = args[2].parse::<usize>().unwrap();

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let original_risks: Vec<Vec<isize>> = lines.map(|line| line.chars().map(|c| c.to_string().parse::<isize>().unwrap()).collect()).collect();

    let mut risks: Vec<Vec<isize>> = vec![vec![isize::MAX; tile_multiplier * original_risks[0].len()]; tile_multiplier * original_risks.len()];
    for i in 0..risks.len() {
        for j in 0..risks[0].len() {
            let base_i: usize = i % original_risks.len();
            let base_j: usize = j % original_risks[0].len();

            let i_offset: usize = i / original_risks.len();
            let j_offset: usize = j / original_risks[0].len();

            let mut value: isize = original_risks[base_i][base_j];
            for _ in 0..(i_offset + j_offset) {
                value += 1;
                if value > 9 {
                    value = 1;
                }
            }

            risks[i][j] = value;
        }
    }

    let mut result: Vec<Vec<isize>> = vec![vec![isize::MAX; risks[0].len()]; risks.len()];
    let mut pq = PriorityQueue::new();
    pq.push((0, 0, 0), 0);

    let mut uniqifier: usize = 1;
    while !pq.is_empty() {
        let val = pq.pop().unwrap();

        let row: usize = val.0.0;
        let col: usize = val.0.1;

        let cost: isize = -val.1;

        if row >= risks.len() || col >= risks[0].len() || result[row][col] <= cost {
            continue;
        }

        result[row][col] = cost;

        if row > 0 {
            pq.push((row - 1, col, uniqifier), -cost - risks[row - 1][col]);
        }
        if row + 1 < risks.len() {
            pq.push((row + 1, col, uniqifier), -cost - risks[row + 1][col]);
        }
        if col > 0 {
            pq.push((row, col - 1, uniqifier), -cost - risks[row][col - 1]);
        }
        if col + 1 < risks[0].len() {
            pq.push((row, col + 1, uniqifier), -cost - risks[row][col + 1]);
        }

        uniqifier += 1;
    }

    println!("result: {}", result[risks.len() - 1][risks[0].len() - 1]);
}
