use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let parsed_values: Vec<Vec<i64>> = lines.map(|line| line.split(&[',', '-']))
                                            .map(|vals| vals.map(|val| val.parse::<i64>().unwrap()))
                                            .map(|x| x.collect())
                                            .collect();

    let result = parsed_values.iter()
                                .filter(|vals| {
        let start_a = vals[0];
        let end_a = vals[1];
        let start_b = vals[2];
        let end_b = vals[3];

        return (start_a <= start_b && end_a >= end_b) || (start_a >= start_b && end_a <= end_b);
    });

    println!("result: {}", result.count());
}
