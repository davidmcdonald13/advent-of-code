use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let values: Vec<Vec<usize>> = lines.map(|line| {
        return line.chars().map(|val| val.to_string().parse::<usize>().unwrap()).collect();
    }).collect();

    let mut result: usize = 0;
    for i in 0..values.len() {
        for j in 0..values[i].len() {
            let val: usize = values[i][j];
            if (i == 0 || val < values[i-1][j]) && (i == values.len() - 1 || val < values[i+1][j]) && (j == 0 || val < values[i][j-1]) && (j == values[i].len() - 1 || val < values[i][j+1]) {
                result += val + 1;
            }
        }
    }

    println!("result: {}", result);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basin_sizes: Vec<usize> = Vec::new();

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            let basin_size: usize = dfs(&values, &mut visited, i, j);
            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort();

    let largest: usize = basin_sizes.pop().unwrap();
    println!("largest: {}", largest);
    let second_largest: usize = basin_sizes.pop().unwrap();
    println!("second_largest: {}", second_largest);
    let third_largest: usize = basin_sizes.pop().unwrap();
    println!("third_largest: {}", third_largest);

    println!("product: {}", largest * second_largest * third_largest);
}

fn dfs(values: &Vec<Vec<usize>>, visited: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> usize {
    let key: (usize, usize) = (i, j);
    if i >= values.len() || j >= values[i].len() || visited.contains(&key) || values[i][j] == 9 {
        return 0;
    }

    visited.insert(key);

    let mut result: usize = 1;
    result += dfs(values, visited, i + 1, j);
    result += if i > 0 { dfs(values, visited, i - 1, j) } else { 0 };
    result += dfs(values, visited, i, j + 1);
    result += if j > 0 { dfs(values, visited, i, j - 1) } else { 0 };

    return result;
}
