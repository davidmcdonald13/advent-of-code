use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let moves: &usize = &args[2].parse::<usize>().unwrap();

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();
    let mut octopuses: Vec<Vec<usize>> = lines.map(|line| line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()).collect();

    let mut flashes: usize = 0;
    let mut move_num: usize = 0;
    loop {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..octopuses.len() {
            for j in 0..octopuses[0].len() {
                queue.push((i, j));
            }
        }

        while queue.len() > 0 {
            let location: (usize, usize) = queue.remove(0);
            let i: usize = location.0;
            let j: usize = location.1;

            if flashed.contains(&location) || i >= octopuses.len() || j >= octopuses[0].len() {
                continue;
            }

            octopuses[i][j] += 1;
            if octopuses[i][j] == 10 {
                octopuses[i][j] = 0;
                flashed.insert(location);

                for i_diff in 0..3 {
                    for j_diff in 0..3 {
                        if i + i_diff == 0 || j + j_diff == 0 {
                            continue;
                        }
                        queue.push((i + i_diff - 1, j + j_diff - 1));
                    }
                }
            }
        }

        if move_num < *moves {
            flashes += flashed.len();
        }
        move_num += 1;

        if flashed.len() == octopuses.len() * octopuses[0].len() {
            println!("all flashed on move {}", move_num);
            break;
        }
    }

    println!("flashes in first {} moves: {}", moves, flashes);
}
