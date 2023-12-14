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
    let start: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();

    let rolled: Vec<Vec<char>> = roll_north(&start);

    return get_load(&rolled);
}

fn roll_north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return grid.iter()
               .enumerate()
               .map(|(i, row)| {
                   row.iter()
                      .enumerate()
                      .map(|(j, val)| {
                          if val == &'#' {
                              return '#';
                          }

                          let section_low: usize = get_section_low(grid, i, j);
                          let section_high: usize = get_section_high(grid, i, j);

                          let stone_count: usize = get_stone_count(grid, section_low, section_high, j);

                          if (i - section_low + 1) <= stone_count {
                              return 'O';
                          } else {
                              return '.';
                          }
                      }).collect()
               }).collect();
}

fn get_section_low(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    for check_row in (1..=row_idx).rev() {
        if grid[check_row - 1][col_idx] == '#' {
            return check_row;
        }
    }
    return 0;
}

fn get_section_high(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    for check_row in (row_idx+1)..grid.len() {
        if grid[check_row][col_idx] == '#' {
            return check_row;
        }
    }
    return grid.len();
}

fn get_stone_count(grid: &Vec<Vec<char>>, low_row: usize, high_row: usize, col: usize) -> usize {
    return grid[low_row..high_row].iter()
                                  .filter(|row| row[col] == 'O')
                                  .count();
}

fn get_load(grid: &Vec<Vec<char>>) -> usize {
    return grid.iter()
               .enumerate()
               .map(|(i, row)| (grid.len() - i) * row.iter()
                                                     .filter(|x| **x == 'O')
                                                     .count())
               .sum();
}

fn unit_tests() -> () {
    assert!(run("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".to_string()) == 136);
}
