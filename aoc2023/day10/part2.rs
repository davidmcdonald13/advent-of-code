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

    let mut expanded: Vec<Vec<bool>> = vec![vec![false; 2 * lines[0].len()]; 2 * lines.len()];

    let start_coords: (i64, i64) = get_start_coords(&lines);
    expanded[(start_coords.0 * 2) as usize][(start_coords.1 * 2) as usize] = true;

    let mut queue: VecDeque<(i64, i64)> = VecDeque::from([(start_coords.0, start_coords.1)]);

    while !queue.is_empty() {
        let (row, col): (i64, i64) = queue.pop_front().unwrap();

        if row < 0 || row >= lines.len() as i64 || col < 0 || col >= lines[0].len() as i64 {
            continue;
        }

        let above: (i64, i64) = (row - 1, col);
        let below: (i64, i64) = (row + 1, col);
        let right: (i64, i64) = (row, col + 1);
        let left: (i64, i64) = (row, col - 1);

        let neighbors: ((i64, i64), (i64, i64)) = match lines[row as usize][col as usize] {
            '|' => (above, below),
            '-' => (left, right),
            'L' => (above, right),
            'J' => (above, left),
            '7' => (left, below),
            'F' => (right, below),
            '.' => continue,
            'S' => {
                let mut dest: (Option<(i64, i64)>, Option<(i64, i64)>) = (None, None);
                if safe_check(&lines, row - 1, col, &['|', '7', 'F']) {
                    dest.0 = Some(above);
                }
                if safe_check(&lines, row + 1, col, &['|', 'J', 'L']) {
                    if dest.0.is_none() {
                        dest.0 = Some(below);
                    } else {
                        dest.1 = Some(below);
                    }
                }
                if safe_check(&lines, row, col - 1, &['-', 'F', 'L']) {
                    if dest.0.is_none() {
                        dest.0 = Some(left);
                    } else {
                        dest.1 = Some(left);
                    }
                }
                if safe_check(&lines, row, col + 1, &['-', '7', 'J']) {
                    dest.1 = Some(right);
                }
                (dest.0.unwrap(), dest.1.unwrap())
            },
            _ => continue,
        };

        for (dest_row, dest_col) in vec![neighbors.0, neighbors.1].iter() {
            expanded[average(2 * dest_row, 2 * row) as usize][average(2 * dest_col, 2 * col) as usize] = true;

            if !expanded[(2 * dest_row) as usize][(2 * dest_col) as usize] {
                queue.push_back((*dest_row, *dest_col));

                expanded[(2 * dest_row) as usize][(2 * dest_col) as usize] = true;
            }
        }
    }

    for row in 0..expanded.len() {
        queue.push_back((row as i64, 0));
        queue.push_back((row as i64, expanded[0].len() as i64 - 1));
    }

    for col in 0..expanded[0].len() {
        queue.push_back((0, col as i64));
        queue.push_back((expanded.len() as i64 - 1, col as i64));
    }

    while !queue.is_empty() {
        let (row, col): (i64, i64) = queue.pop_front().unwrap();

        if row < 0 || row as usize >= expanded.len() || col < 0 || col as usize >= expanded[0].len() {
            continue;
        }

        if expanded[row as usize][col as usize] {
            continue;
        }

        expanded[row as usize][col as usize] = true;

        queue.push_back((row + 1, col));
        queue.push_back((row - 1, col));
        queue.push_back((row, col + 1));
        queue.push_back((row, col - 1));
    }

    let result: i64 = expanded.iter()
                              .enumerate()
                              .filter(|(i, _row)| i & 1 == 0)
                              .map(|(_, row)| 
                                  row.iter()
                                            .enumerate()
                                            .filter(|(j, _)| j & 1 == 0)
                                            .map(|(_, val)| {
                                                if *val {
                                                    return 0;
                                                } else {
                                                    return 1;
                                                }
                                            })
                                            .sum::<i64>()
                                ).sum();

    return result;
}

fn average(a: i64, b: i64) -> i64 {
    return (a + b) / 2;
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
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".to_string()) == 4);

    assert!(run(
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".to_string()) == 8);
}
