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

struct Distances {
    north: usize,
    west: usize,
    south: usize,
    east: usize,
}

fn run(contents: String) -> usize {
    let start: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();

    let distances: Vec<Vec<Option<Distances>>> = get_distances(&start);

    let mut states: Vec<Vec<Vec<char>>> = vec![start];
    for _ in 0..1000000000 {
        let rolled_north: Vec<Vec<char>> = roll_north(&states[states.len() - 1], &distances);
        let rolled_west: Vec<Vec<char>> = roll_west(&rolled_north, &distances);
        let rolled_south: Vec<Vec<char>> = roll_south(&rolled_west, &distances);
        let rolled_east: Vec<Vec<char>> = roll_east(&rolled_south, &distances);

        states.push(rolled_east);

        let result: Option<usize> = get_result(&states);
        if result.is_some() {
            return result.unwrap();
        }
    }

    return get_load(&states[states.len() - 1]);
}

fn get_result(past_states: &Vec<Vec<Vec<char>>>) -> Option<usize> {
    for i in (0..(past_states.len()-1)).rev() {
        if past_states[i] == past_states[past_states.len() - 1] {
            let repeat_len: usize = past_states.len() - 1 - i; // 7
            let pattern_index: usize = (1000000000 - i) % repeat_len; // 3
            return Some(get_load(&past_states[i + pattern_index]));
        }
    }
    return None;
}

fn roll_north(grid: &Vec<Vec<char>>, distances: &Vec<Vec<Option<Distances>>>) -> Vec<Vec<char>> {
    roll_vert(grid, distances, true)
}

fn roll_south(grid: &Vec<Vec<char>>, distances: &Vec<Vec<Option<Distances>>>) -> Vec<Vec<char>> {
    roll_vert(grid, distances, false)
}

fn roll_vert(grid: &Vec<Vec<char>>,
             distances: &Vec<Vec<Option<Distances>>>,
             rolling_north: bool) -> Vec<Vec<char>> {
    return grid.iter()
               .enumerate()
               .map(|(i, row)| {
                   row.iter()
                      .enumerate()
                      .map(|(j, val)| {
                          if val == &'#' {
                              return '#';
                          }

                          let dist: &Distances = distances[i][j].as_ref().unwrap();

                          let section_low: usize = i - dist.north;
                          let section_high: usize = i + dist.south;

                          let stone_count: usize = get_stone_count_vert(grid, section_low, section_high, j);

                          if (rolling_north && dist.north < stone_count) ||
                             (!rolling_north && dist.south < stone_count) {
                              return 'O';
                          } else {
                              return '.';
                          }
                      }).collect()
               }).collect();
}

fn roll_east(grid: &Vec<Vec<char>>, distances: &Vec<Vec<Option<Distances>>>) -> Vec<Vec<char>> {
    roll_horiz(grid, distances, true)
}

fn roll_west(grid: &Vec<Vec<char>>, distances: &Vec<Vec<Option<Distances>>>) -> Vec<Vec<char>> {
    roll_horiz(grid, distances, false)
}

fn roll_horiz(grid: &Vec<Vec<char>>,
              distances: &Vec<Vec<Option<Distances>>>,
              rolling_east: bool) -> Vec<Vec<char>> {
    grid.iter()
        .enumerate()
        .map(|(i, row)|
             row.iter()
                .enumerate()
                .map(|(j, val)| {
                    if val == &'#' {
                        return '#';
                    }

                    let dist: &Distances = distances[i][j].as_ref().unwrap();

                    let section_low: usize = j - dist.west;
                    let section_high: usize = j + dist.east;

                    let stone_count: usize = get_stone_count_horiz(row, section_low, section_high);

                    if (rolling_east && dist.east < stone_count) ||
                       (!rolling_east && dist.west < stone_count) {
                           return 'O';
                       } else {
                           return '.';
                       }
                }).collect()
        ).collect()
}

fn get_distances(grid: &Vec<Vec<char>>) -> Vec<Vec<Option<Distances>>> {
    return grid.iter()
               .enumerate()
               .map(|(i, row)|
                   row.iter()
                      .enumerate()
                      .map(|(j, val)|
                           if *val == '#' {
                               None
                           } else {
                               Some(Distances {
                                   north: get_free_north(grid, i, j),
                                   west: get_free_west(grid, i, j),
                                   south: get_free_south(grid, i, j),
                                   east: get_free_east(grid, i, j),
                               })
                           }
                      ).collect()
               ).collect();
}

fn get_free_north(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    get_free_vert(grid[..row_idx].iter().rev().collect(), col_idx)
}

fn get_free_south(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    get_free_vert(grid[(row_idx+1)..].iter().collect(), col_idx)
}

fn get_free_vert(grid: Vec<&Vec<char>>, col_idx: usize) -> usize {
    grid.iter()
        .enumerate()
        .filter(|(_idx, row)| row[col_idx] == '#')
        .map(|(idx, _)| idx)
        .next()
        .unwrap_or(grid.len())
}

fn get_free_west(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    get_free_horiz(grid[row_idx][..col_idx].iter().rev().collect())
}

fn get_free_east(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    get_free_horiz(grid[row_idx][(col_idx+1)..].iter().collect())
}

fn get_free_horiz(row: Vec<&char>) -> usize {
    row.iter()
       .enumerate()
       .filter(|(_idx, val)| ***val == '#')
       .map(|(idx, _)| idx)
       .next()
       .unwrap_or(row.len())
}

fn get_stone_count_vert(grid: &Vec<Vec<char>>, low_row: usize, high_row: usize, col: usize) -> usize {
    return grid[low_row..=high_row].iter()
                                   .filter(|row| row[col] == 'O')
                                   .count();
}

fn get_stone_count_horiz(row: &Vec<char>, low_col: usize, high_col: usize) -> usize {
    row[low_col..=high_col].iter()
                           .filter(|x| **x == 'O')
                           .count()
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
#OO..#....".to_string()) == 64);
}
