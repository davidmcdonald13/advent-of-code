use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let values: Vec<Vec<u32>> = lines.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let height: usize = values.len();
    let width: usize = values[0].len();

    let mut score: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for i in 0..height {
        for j in 0..width {
            let mut visible_above: usize = i;
            for above_i in (0..i).rev() {
                if values[above_i][j] >= values[i][j] {
                    visible_above = i - above_i;
                    break;
                }
            }

            let mut visible_below: usize = height - i - 1;
            for below_i in (i+1)..height {
                if values[below_i][j] >= values[i][j] {
                    visible_below = below_i - i;
                    break;
                }
            }

            let mut visible_left: usize = j;
            for left_j in (0..j).rev() {
                if values[i][left_j] >= values[i][j] {
                    visible_left = j - left_j;
                    break;
                }
            }

            let mut visible_right: usize = width - j - 1;
            for right_j in (j+1)..width {
                if values[i][right_j] >= values[i][j] {
                    visible_right = right_j - j;
                    break;
                }
            }

            score[i][j] = visible_above * visible_below * visible_left * visible_right;
        }
    }

    let result: i64 = score.iter().map(|row| row.iter().map(|x| *x as i64).max().unwrap()).max().unwrap();

    println!("result: {}", result);
}
