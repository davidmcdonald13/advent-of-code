use std::collections::HashMap;
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

fn unit_tests() -> () {
    assert!(run("???.### 1,1,3".to_string()) == 1);
    assert!(run(".??..??...?##. 1,1,3".to_string()) == 16384);
    assert!(run("?#?#?#?#?#?#?#? 1,3,1,6".to_string()) == 1);
    assert!(run("????.#...#... 4,1,1".to_string()) == 16);
    assert!(run("????.######..#####. 1,6,5".to_string()) == 2500);
    assert!(run("?###???????? 3,2,1".to_string()) == 506250);
    assert!(run("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string()) == 525152);
}

fn run(contents: String) -> usize {
    return contents.lines()
                   .map(|line| {
                       let mut split = line.split_ascii_whitespace();
                       (quintuple(split.next().unwrap(), '?').chars().collect::<Vec<char>>(),
                        quintuple(split.next().unwrap(), ',')
                             .split(',')
                             .map(|val| val.parse::<usize>().unwrap())
                             .collect::<Vec<usize>>())
                   })
                   .map(|(chars, counts)| possible_combos(&chars, &counts, &mut HashMap::new()))
                   .sum();
}

fn possible_combos(chars: &Vec<char>, counts: &Vec<usize>, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (chars.len(), counts.len());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }

    let result: usize = possible_combos_impl(chars, counts, memo);
    memo.insert(key, result);
    return result;
}

fn possible_combos_impl(chars: &Vec<char>, counts: &Vec<usize>, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if chars.is_empty() {
        if counts.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if counts.is_empty() {
        if chars.iter().all(|c| *c == '.' || *c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    if chars.len() < counts[0] {
        return 0;
    }

    if chars.len() == counts[0] {
        if counts.len() > 1 {
            return 0;
        } else if can_grab_first_n(chars, counts[0]) {
            return 1;
        }
    }

    let leading_empties: usize = get_leading_empties(chars);
    if leading_empties > 0 {
        return possible_combos(&chars[leading_empties..].to_vec(), counts, memo);
    }

    if chars[0] == '#' {
        if can_grab_first_n(chars, counts[0]) {
            return possible_combos(&chars[(counts[0]+1)..].to_vec(), &counts[1..].to_vec(), memo);
        } else {
            return 0;
        }
    }

    let mut result = 0;
    if can_grab_first_n(chars, counts[0]) {
        result += possible_combos(&chars[(counts[0]+1)..].to_vec(), &counts[1..].to_vec(), memo);
    }
    result += possible_combos(&chars[1..].to_vec(), counts, memo);
    return result;
}

fn get_leading_empties(chars: &Vec<char>) -> usize {
    for i in 0..chars.len() {
        if chars[i] != '.' {
            return i;
        }
    }
    return chars.len();
}

fn can_grab_first_n(chars: &Vec<char>, n: usize) -> bool {
    return chars.len() >= n &&
           chars[..n].iter().all(|c| *c == '#' || *c == '?') &&
           (chars.len() == n || chars[n] == '.' || chars[n] == '?');
}

fn quintuple(val: &str, sep: char) -> String {
    let mut result: String = String::from("");

    result.push_str(val);
    result.push(sep);
    result.push_str(val);
    result.push(sep);
    result.push_str(val);
    result.push(sep);
    result.push_str(val);
    result.push(sep);
    result.push_str(val);

    return result;
}
