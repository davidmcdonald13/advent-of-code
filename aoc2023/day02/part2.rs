use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let red_key: &str = "red";
    let green_key: &str = "green";
    let blue_key: &str = "blue";

    let result: i64 =
        lines.map(|line| {
                let mut max_counts: HashMap<&str, i64> = HashMap::from([
                    (red_key, 0),
                    (green_key, 0),
                    (blue_key, 0),
                ]);
                line.split(&[':', ',', ';'][..])
                    .skip(1)
                    .for_each(|s| {
                        let tokens: Vec<&str> = s.trim().split(' ').collect();
                        let count: i64 = tokens.get(0).unwrap().parse::<i64>().unwrap();
                        let key: &str = tokens.get(1).unwrap();

                        max_counts.insert(key, cmp::max(*max_counts.get(key).unwrap(), count));
                    });
                let result: i64 = max_counts.values()
                          .cloned()
                          .product();
                return result;
             })
             .sum();

    println!("result: {}", result);
}
