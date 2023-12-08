use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: Vec<&str> = contents.split("\n\n").collect();

    let directions: Vec<char> = lines[0].chars().collect();

    let graph: HashMap<&str, (&str, &str)> = parse_map(lines[1]);

    let starts: Vec<&str> = graph.keys().filter(|node| node.ends_with('A')).map(|x| *x).collect();

    let result: usize = starts.iter().map(|start| {
        let mut ptr: &str = start;
        let mut direction_index: usize = 0;
        while !ptr.ends_with('Z') {
            let neighbors: (&str, &str) = *graph.get(ptr).unwrap();
            if directions[direction_index % directions.len()] == 'L' {
                ptr = neighbors.0;
            } else {
                ptr = neighbors.1;
            }

            direction_index += 1;
        }
        direction_index
    }).reduce(|acc, x| lcm(acc, x)).unwrap();

    println!("result: {}", result);
}

fn lcm(x: usize, y: usize) -> usize {
    let mut result: usize = x;
    while result % y != 0 {
        result += x;
    }
    return result;
}

fn parse_map(s: &str) -> HashMap<&str, (&str, &str)> {
    return s.lines()
            .map(|line| {
                let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
                let left: &str = &tokens[2][1..4];
                let right: &str = &tokens[3][..3];

                (tokens[0], (left, right))
            })
            .collect();
}
