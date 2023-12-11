use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let arrs: Vec<Vec<i64>> = lines.map(parse_line).collect();

    let result: i64 = arrs.iter().map(|arr| get_prev_value(arr)).sum();

    println!("result: {}", result);
}

fn get_prev_value(arr: &Vec<i64>) -> i64 {
    if arr.iter().all(|x| x == &0) {
        return 0;
    }

    let diff_arr: Vec<i64> = arr.iter()
                                .enumerate()
                                .skip(1)
                                .map(|(i, x)| x - arr[i-1])
                                .collect();
    return arr[0] - get_prev_value(&diff_arr);
}

fn parse_line(line: &str) -> Vec<i64> {
    return line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
}
