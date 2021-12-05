use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    for line in lines {
        let mut pairs: str::Split<&str> = line.split(" -> ");

        let mut start: str::Split<&str> = pairs.next().unwrap().split(",");
        let mut finish: str::Split<&str> = pairs.next().unwrap().split(",");

        let start_x: usize = start.next().unwrap().parse::<usize>().unwrap();
        let start_y: usize = start.next().unwrap().parse::<usize>().unwrap();

        let finish_x: usize = finish.next().unwrap().parse::<usize>().unwrap();
        let finish_y: usize = finish.next().unwrap().parse::<usize>().unwrap();

        if start_x == finish_x || start_y == finish_y {
            for x in cmp::min(start_x, finish_x)..(cmp::max(start_x, finish_x) + 1) {
                for y in cmp::min(start_y, finish_y)..(cmp::max(start_y, finish_y) + 1) {
                    let key: (usize, usize) = (x, y);
                    let before: usize = *points.get(&key).unwrap_or(&0);
                    points.insert(key, before + 1);
                }
            }
        }
    }

    let multi_points: Vec<&usize> = points.values().filter(|count| **count > 1).collect();

    println!("result: {}", multi_points.len());
}
