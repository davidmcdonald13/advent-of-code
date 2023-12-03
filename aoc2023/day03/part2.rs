use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let lines_vec: Vec<&str> = lines.collect();

    let gears: HashSet<(i64, i64)> =
        lines_vec.iter().enumerate()
             .flat_map(|(row_idx, line)|
                 line.chars()
                     .enumerate()
                     .filter(|(_col_idx, val)| val == &'*')
                     .map(move |(col_idx, _)| (row_idx as i64, col_idx as i64))
             ).collect();

    let gear_number_mapping_vec: Vec<HashMap<(i64, i64), Vec<usize>>> =
        lines_vec.iter()
                 .enumerate()
                 .map(|(row_idx, line)| get_gear_numbers(row_idx as i64, line, &gears))
                 .collect();

    let mut gear_number_mapping: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    gear_number_mapping_vec.into_iter()
                           .for_each(|m| {
                               for (coords, values) in m {
                                   if gear_number_mapping.contains_key(&coords) {
                                       gear_number_mapping.get_mut(&coords).unwrap().extend(values);
                                   } else {
                                       gear_number_mapping.insert(coords, values);
                                   }
                               }
                           });

    let result: usize =
        gear_number_mapping.iter().map(|(_, values)| {
            if values.len() == 2 {
                return values[0] * values[1];
            }
            return 0;
        })
                           .sum();

    println!("result: {}", result);
}

fn get_gear_numbers(row_idx: i64, line: &str, gears: &HashSet<(i64, i64)>) -> HashMap<(i64, i64), Vec<usize>> {
    let line_vec: Vec<char> = line.chars().collect();

    let len: usize = line_vec.len();

    let mut result: HashMap<(i64, i64), Vec<usize>> = HashMap::new();

    let mut running: usize = 0;
    let mut adjacent_gears: HashSet<(i64, i64)> = HashSet::new();

    for col_idx in 0..len {
        let val: char = line_vec[col_idx];
        if val.is_numeric() {
            running = running * 10 + val.to_digit(10).unwrap() as usize;

            for gear in get_adjacent_gears(row_idx, col_idx as i64, gears) {
                adjacent_gears.insert(gear);
            }
        } else {
            for gear in &adjacent_gears {
                if result.contains_key(gear) {
                    result.get_mut(gear).unwrap().push(running);
                } else {
                    result.insert(*gear, vec![running]);
                }
            }

            running = 0;
            adjacent_gears = HashSet::new();
        }
    }

    for gear in &adjacent_gears {
        if result.contains_key(gear) {
            result.get_mut(gear).unwrap().push(running);
        } else {
            result.insert(*gear, vec![running]);
        }
    }

    return result;
}

fn get_adjacent_gears(row_idx: i64, col_idx: i64, gears: &HashSet<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for row in row_idx-1..row_idx+2 {
        for col in col_idx-1..col_idx+2 {
            if gears.contains(&(row, col)) {
                result.push((row, col));
            }
        }
    }
    return result;
}
