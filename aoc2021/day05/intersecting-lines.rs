use std::collections::HashMap;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let is_part_2: bool = true;

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let mut points: HashMap<(isize, isize), isize> = HashMap::new();

    for line in lines {
        let mut pairs: str::Split<&str> = line.split(" -> ");

        let mut start: str::Split<&str> = pairs.next().unwrap().split(",");
        let mut finish: str::Split<&str> = pairs.next().unwrap().split(",");

        let start_x: isize = start.next().unwrap().parse::<isize>().unwrap();
        let start_y: isize = start.next().unwrap().parse::<isize>().unwrap();

        let finish_x: isize = finish.next().unwrap().parse::<isize>().unwrap();
        let finish_y: isize = finish.next().unwrap().parse::<isize>().unwrap();

        let x_dir: isize = get_direction(start_x, finish_x);
        let y_dir: isize = get_direction(start_y, finish_y);

        if is_part_2 || (start_x == finish_x || start_y == finish_y) {
            let mut x: isize = start_x;
            let mut y: isize = start_y;
            loop {
                let key: (isize, isize) = (x, y);
                let before: isize = *points.get(&key).unwrap_or(&0);
                points.insert(key, before + 1);

                if x == finish_x &&  y == finish_y {
                    break;
                }

                x += x_dir;
                y += y_dir;
            }
        }
    }

    let multi_points: Vec<&isize> = points.values().filter(|count| **count > 1).collect();

    println!("result: {}", multi_points.len());
}

fn get_direction(start: isize, finish: isize) -> isize {
    if start == finish {
        return 0;
    } else if start < finish {
        return 1;
    } else {
        return -1;
    }
}
