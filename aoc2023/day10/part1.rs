use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    unit_tests();

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    println!("Result: {}", run(contents));
}

fn run(input: String) -> i64 {
    let lines: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut distances: Vec<Vec<Option<i64>>> = vec![vec![None; lines[0].len()]; lines.len()];

    let start_coords: (i64, i64) = get_start_coords(&lines);

    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::from([(start_coords.0, start_coords.1, 0)]);

    while !queue.is_empty() {
        let (row, col, distance): (i64, i64, i64) = queue.pop_front().unwrap();

        if row < 0 || row >= lines.len() as i64 || col < 0 || col >= lines[0].len() as i64 {
            continue;
        }
        if distances[row as usize][col as usize].is_some() {
            continue;
        }

        distances[row as usize][col as usize] = Some(distance);

        let above: (i64, i64, i64) = (row - 1, col, distance + 1);
        let below: (i64, i64, i64) = (row + 1, col, distance + 1);
        let right: (i64, i64, i64) = (row, col + 1, distance + 1);
        let left: (i64, i64, i64) = (row, col - 1, distance + 1);

        match lines[row as usize][col as usize] {
            '|' => {
                queue.push_back(above);
                queue.push_back(below);
            },
            '-' => {
                queue.push_back(left);
                queue.push_back(right);
            },
            'L' => {
                queue.push_back(above);
                queue.push_back(right);
            },
            'J' => {
                queue.push_back(above);
                queue.push_back(left);
            },
            '7' => {
                queue.push_back(left);
                queue.push_back(below);
            },
            'F' => {
                queue.push_back(right);
                queue.push_back(below);
            },
            '.' => continue,
            'S' => {
                if safe_check(&lines, row - 1, col, &['|', '7', 'F']) {
                    queue.push_back(above);
                }
                if safe_check(&lines, row + 1, col, &['|', 'J', 'L']) {
                    queue.push_back(below);
                }
                if safe_check(&lines, row, col - 1, &['-', 'F', 'L']) {
                    queue.push_back(left);
                }
                if safe_check(&lines, row, col + 1, &['-', '7', 'J']) {
                    queue.push_back(right);
                }
            },
            _ => continue,
        };
    }

    let result: i64 = distances.iter()
                                 .map(|dist| dist.iter()
                                                 .map(|x| match x { Some(y) => *y, None => 0, })
                                                 .max()
                                                 .unwrap())
                                 .max()
                                 .unwrap();

    return result;
}

fn safe_check(arr: &Vec<Vec<char>>, row: i64, col: i64, targets: &[char]) -> bool {
    if row < 0 || row >= arr.len() as i64 || col < 0 || col >= arr[0].len() as i64 {
        return false;
    }
    return targets.contains(&arr[row as usize][col as usize]);
}

fn get_start_coords(lines: &Vec<Vec<char>>) -> (i64, i64) {
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == 'S' {
                return (i as i64, j as i64);
            }
        }
    }
    return (0, 0);
}

fn unit_tests() -> () {
    assert!(run(
".....
.S-7.
.|.|.
.L-J.
.....".to_string()) == 4);

    assert!(run(
"..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string()) == 8);
}
