use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Range;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let chunks: Vec<&str> = contents.split("\n\n").collect();

    let mappers: Vec<HashMap<Range<i64>, i64>> =
        chunks[1..].iter().map(|chunk| {
            let lines: std::str::Lines = chunk.lines();
            let result: HashMap<Range<i64>, i64> =
                lines.skip(1)
                     .map(|line| {
                         let vals: Vec<i64> = line.split_ascii_whitespace()
                                                  .map(|token| token.parse::<i64>().unwrap())
                                                  .collect();
                         return (vals[1]..(vals[1]+vals[2]), vals[0]);
                     }).collect();
            return result;
        }).collect();

     let seeds = chunks[0].split_ascii_whitespace()
                          .skip(1)
                          .map(|token| token.parse::<i64>().unwrap());

     let result: i64 = seeds.map(|seed| {
         return mappers.iter()
                       .fold(seed, |val, mapper| {
                           for (range, offset) in mapper {
                               if range.contains(&val) {
                                   return offset + (val - range.start);
                               }
                           }
                           return val;
                       });
     }).min().unwrap();

    println!("result: {}", result);
}
