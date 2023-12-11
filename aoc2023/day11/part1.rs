use std::cmp;
use std::env;
use std::fs;

fn main() {
    unit_tests();

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let result = run(contents);
    println!("result: {}", result);
}

fn run(input: String) -> usize {
    let arr: Vec<Vec<char>> = input.lines()
                                   .map(|line| line.chars().collect())
                                   .collect();

    let galaxies: Vec<(usize, usize)> = arr.iter()
                                           .enumerate()
                                           .flat_map(|(i, row)| row.iter()
                                                                   .enumerate()
                                                                   .filter(|(_, val)| **val == '#')
                                                                   .map(move |(j, _)| (i, j))
                                           )
                                           .collect();

    let empty_rows: Vec<usize> = arr.iter()
                                    .enumerate()
                                    .filter(|(_, row)| row.iter().all(|c| *c == '.'))
                                    .map(|(i, _)| i)
                                    .collect();
    let empty_cols: Vec<usize> = (0..arr[0].len()).filter(|j| arr.iter()
                                                                 .all(|row| row[*j] == '.'))
                                                  .collect();

    let mut result: usize = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let row_iter: std::ops::Range<usize> = cmp::min(galaxies[i].0, galaxies[j].0)..cmp::max(galaxies[i].0, galaxies[j].0);
            let col_iter: std::ops::Range<usize> = cmp::min(galaxies[i].1, galaxies[j].1)..cmp::max(galaxies[i].1, galaxies[j].1);

            let row_dist: usize = get_size(&row_iter) + empty_rows.iter().filter(|x| row_iter.contains(x)).count();
            let col_dist: usize = get_size(&col_iter) + empty_cols.iter().filter(|x| col_iter.contains(x)).count();
            result += row_dist + col_dist;
        }
    }

    return result;
}

fn get_size(range: &std::ops::Range<usize>) -> usize {
    return range.end - range.start;
}

fn unit_tests() -> () {
    assert!(run("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string()) == 374);

    println!("Unit tests passed");
}
