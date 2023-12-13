use std::env;
use std::fs;

fn main() {
    unit_tests();
    println!("All unit tests passed!");

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");

    let result: usize = run(contents);
    println!("result: {}", result);
}

fn run(contents: String) -> usize {
    return contents.split("\n\n")
                   .map(get_mirror_value)
                   .sum();
}

fn get_mirror_value(s: &str) -> usize {
    let chars: &Vec<Vec<char>> = &s.lines().map(|line| line.chars().collect()).collect();

    let horizontal_value: Option<usize> = get_horizontal_mirror_value(chars);
    if horizontal_value.is_some() {
        return 100 * horizontal_value.unwrap();
    }

    let vertical_value: Option<usize> = get_vertical_mirror_value(chars);
    return vertical_value.unwrap();
}

fn get_horizontal_mirror_value(char_arrs: &Vec<Vec<char>>) -> Option<usize> {
    for row in 1..char_arrs.len() {
        let mut diff_count: usize = 0;
        for top in (0..row).rev() {
            let bottom: usize = 2 * row - 1 - top;
            if bottom >= char_arrs.len() {
                break;
            }

            for (top_val, bottom_val) in char_arrs[top].iter().zip(&char_arrs[bottom]) {
                if top_val != bottom_val {
                    diff_count += 1;
                    if diff_count >= 1 {
                        break;
                    }
                }
            }

            if diff_count >= 1 {
                break;
            }
        }
        if diff_count == 0 {
            return Some(row);
        }
    }
    return None;
}

fn get_vertical_mirror_value(char_arrs: &Vec<Vec<char>>) -> Option<usize> {
    for col in 1..char_arrs[0].len() {
        if char_arrs.iter().map(|line| {
            let mut diff_count: usize = 0;
            for left in (0..col).rev() {
                let right: usize = 2 * col - 1 - left;
                if right >= char_arrs[0].len() {
                    break;
                }

                if line[left] != line[right] {
                    diff_count += 1;
                }
            }
            return diff_count;
        }).sum::<usize>() == 0 {
            return Some(col);
        }
    }
    return None;
}

fn unit_tests() -> () {
    assert!(run("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".to_string()) == 405);
}
