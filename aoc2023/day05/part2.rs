use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;
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

     let seed_tokens: Vec<i64> = chunks[0].split_ascii_whitespace()
                                          .skip(1)
                                          .map(|token| token.parse::<i64>().unwrap())
                                          .collect();

     let mut seeds_ranges: Vec<Range<i64>> = Vec::new();
     for i in (0..seed_tokens.len()).step_by(2) {
         let min: i64 = seed_tokens[i];
         let max: i64 = min + seed_tokens[i+1];

         seeds_ranges.push(min..max);
     }

     let ranges: Vec<Range<i64>> =
         mappers.into_iter()
                .fold(seeds_ranges, |acc, layer| combine_ranges(acc, layer));

     let result: i64 = ranges.iter().map(|range| range.start).min().unwrap();

    println!("result: {}", result);
}

fn combine_ranges(left: Vec<Range<i64>>, right: HashMap<Range<i64>, i64>) -> Vec<Range<i64>> {
    let mut result: Vec<Range<i64>> = Vec::new();

    let mut queue: VecDeque<Range<i64>> = left.into_iter().collect();

    while !queue.is_empty() {
        let src_range: Range<i64> = queue.pop_front().unwrap();
        if src_range.is_empty() {
            continue;
        }

        let mut found: bool = false;

        for (dest_range, dest_offset) in right.iter() {
            if ranges_overlap(&src_range, dest_range) {
                let (unused_before, used, unused_after): (Range<i64>, Range<i64>, Range<i64>) =
                    resolve_overlap(&src_range, dest_range, dest_offset);

                result.push(used);

                queue.push_back(unused_before);
                queue.push_back(unused_after);

                found = true;

                break;
            }
        }

        if !found {
            result.push(src_range);
        }
    }

    return result;
}

fn ranges_overlap(left: &Range<i64>, right: &Range<i64>) -> bool {
    if left.start > right.start {
        // ensure that left is to the left of right
        return ranges_overlap(right, left);
    }
    // end is exclusive, so this must be strictly greater
    return left.end > right.start;
}

fn resolve_overlap(src_range: &Range<i64>,
                   dest_range: &Range<i64>,
                   dest_offset: &i64) -> (Range<i64>, Range<i64>, Range<i64>) {
    let left: Range<i64> = src_range.start..dest_range.start;
    let right: Range<i64> = dest_range.end..src_range.end;

    let middle: Range<i64> = cmp::max(src_range.start, dest_range.start)..cmp::min(src_range.end, dest_range.end);

    let diff: i64 = middle.start - dest_range.start;
    let mapped_middle: Range<i64> = (dest_offset + diff)..(dest_offset + diff + middle.end - middle.start);

    return (left, mapped_middle, right);
}
