use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut values: Vec<i64> = vec![1];

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let prev: i64 = *values.last().unwrap();

        values.push(prev);
        if tokens[0] == "addx" {
            let val = tokens[1].parse::<i64>().unwrap();
            values.push(prev + val);
        }
    }

    let pixels: Vec<char> = values.iter().enumerate().map(|(i, loc)| {
        if (((i as i64) % 40) - loc).abs() <= 1 {
            return '#';
        } else {
            return '.';
        }
    }).collect();

    for row_start in (0..240).step_by(40) {
        println!("{}", pixels[row_start..(row_start+40)].iter().collect::<String>());
    }
}
