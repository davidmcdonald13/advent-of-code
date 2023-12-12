use std::env;
use std::fs;

fn main() {
    unit_tests();

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");

    let result: usize = run(contents);
    println!("result: {}", result);
}

fn unit_tests() -> () {
    assert!(run("??.### 1,1,3".to_string()) == 0);
    assert!(run("???.### 1,1,3".to_string()) == 1);
    assert!(run(".??..??...?##. 1,1,3".to_string()) == 4);
    assert!(run("?#?#?#?#?#?#?#? 1,3,1,6".to_string()) == 1);
    assert!(run("????.#...#... 4,1,1".to_string()) == 1);
    assert!(run("????.######..#####. 1,6,5".to_string()) == 4);
    assert!(run("?###???????? 3,2,1".to_string()) == 10);
    assert!(run("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".to_string()) == 21);
}

fn run(contents: String) -> usize {
    return contents.lines()
                   .map(|line| {
                       let mut split = line.split_ascii_whitespace();
                       (split.next().unwrap().chars().collect::<Vec<char>>(),
                        split.next()
                             .unwrap()
                             .split(',')
                             .map(|val| val.parse::<usize>().unwrap())
                             .collect::<Vec<usize>>())
                   })
                   .map(|(chars, counts)| possible_combos(&chars, &counts))
                   .sum();
}

fn possible_combos(chars: &Vec<char>, counts: &Vec<usize>) -> usize {
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
        return possible_combos(&chars[leading_empties..].to_vec(), counts);
    }

    if chars[0] == '#' {
        if can_grab_first_n(chars, counts[0]) {
            return possible_combos(&chars[(counts[0]+1)..].to_vec(), &counts[1..].to_vec());
        } else {
            return 0;
        }
    }

    let mut result = 0;
    if can_grab_first_n(chars, counts[0]) {
        result += possible_combos(&chars[(counts[0]+1)..].to_vec(), &counts[1..].to_vec());
    }
    result += possible_combos(&chars[1..].to_vec(), counts);
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
