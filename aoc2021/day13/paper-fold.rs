use std::collections::HashSet;
use std::cmp;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut finding_points: bool = true;
    let mut max_x: usize = usize::MAX;
    let mut max_y: usize = usize::MAX;
    for line in lines {
        if finding_points {
            if line.is_empty() {
                finding_points = false;
                continue;
            }

            let mut coords: str::Split<&str> = line.split(",");
            let x: usize = coords.next().unwrap().parse::<usize>().unwrap();
            let y: usize = coords.next().unwrap().parse::<usize>().unwrap();

            points.insert((x, y));
        } else {
            let mut vector: str::Split<&str> = line.split(" ").last().unwrap().split("=");
            let axis: &str = vector.next().unwrap();
            let offset: usize = vector.next().unwrap().parse::<usize>().unwrap();

            let mut new_points: HashSet<(usize, usize)> = HashSet::new();

            for (x, y) in points {
                if axis == "x" {
                    max_x = cmp::min(max_x, offset);
                    if x < offset {
                        new_points.insert((x, y));
                    } else {
                        let new_x: usize = offset - (x - offset);
                        new_points.insert((new_x, y));
                    }
                } else {
                    max_y = cmp::min(max_y, offset);
                    if y < offset {
                        new_points.insert((x, y));
                    } else {
                        let new_y: usize = offset - (y - offset);
                        new_points.insert((x, new_y));
                    }
                }
            }

            points = new_points;
            println!("num_points: {}", points.len());
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!("");
    }
}
