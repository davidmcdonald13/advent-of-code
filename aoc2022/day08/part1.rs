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

    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 0..height {
        for j in 0..width {
            let mut visible_above: bool = true;
            for above_i in (0..i).rev() {
                if values[above_i][j] >= values[i][j] {
                    visible_above = false;
                    break;
                }
            }

            let mut visible_below: bool = true;
            for below_i in (i+1)..height {
                if values[below_i][j] >= values[i][j] {
                    visible_below = false;
                    break;
                }
            }

            let mut visible_left: bool = true;
            for left_j in (0..j).rev() {
                if values[i][left_j] >= values[i][j] {
                    visible_left = false;
                    break;
                }
            }

            let mut visible_right: bool = true;
            for right_j in (j+1)..width {
                if values[i][right_j] >= values[i][j] {
                    visible_right = false;
                    break;
                }
            }

            if visible_above || visible_below || visible_left || visible_right {
                visible[i][j] = true;
            }
        }
    }

    let result: i64 = visible.iter().map(|row| row.iter().map(|x| *x as i64).sum::<i64>()).sum();

    println!("result: {}", result);
}
