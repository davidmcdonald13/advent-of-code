use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: str::Lines = contents.lines();

    let mut output_sum: usize = 0;
    let target_values: Vec<usize> = vec![1, 4, 7, 8];
    let mut ones_fours_sevens_eights: usize = 0;
    for line in lines {
        let mut halves: str::Split<&str> = line.split(" | ");

        let keys: Vec<HashSet<char>> = to_char_sets(halves.next().unwrap());
        let outputs: Vec<HashSet<char>> = to_char_sets(halves.last().unwrap());

        let mut letters_to_num: Vec<HashSet<char>> = vec![HashSet::new(); 10];
        let mut sixes: Vec<HashSet<char>> = Vec::new();
        let mut fives: Vec<HashSet<char>> = Vec::new();

        for key in keys {
            if key.len() == 2 {
                letters_to_num[1] = key;
            } else if key.len() == 3 {
                letters_to_num[7] = key;
            } else if key.len() == 4 {
                letters_to_num[4] = key;
            } else if key.len() == 5 {
                fives.push(key);
            } else if key.len() == 6 {
                sixes.push(key);
            } else {
                letters_to_num[8] = key;
            }
        }

        for six in sixes {
            if six.is_superset(letters_to_num.get(4).unwrap()) {
                letters_to_num[9] = six;
            } else if six.is_superset(letters_to_num.get(1).unwrap()) {
                letters_to_num[0] = six;
            } else {
                letters_to_num[6] = six;
            }
        }

        for five in fives {
            if five.is_superset(letters_to_num.get(1).unwrap()) {
                letters_to_num[3] = five;
            } else if five.is_subset(letters_to_num.get(9).unwrap()) {
                letters_to_num[5] = five;
            } else {
                letters_to_num[2] = five;
            }
        }

        let mut this_output_sum: usize = 0;
        for output in outputs {
            let numeric_value: usize = get_index(&letters_to_num, output);
            if target_values.contains(&numeric_value) {
                ones_fours_sevens_eights += 1;
            }
            this_output_sum *= 10;
            this_output_sum += numeric_value;
        }
        output_sum += this_output_sum;
    }

    println!("ones_fours_sevens_eights: {}", ones_fours_sevens_eights);
    println!("output_sum: {}", output_sum);
}

fn to_char_sets(values: &str) -> Vec<HashSet<char>> {
    return values.split_ascii_whitespace().map(|s| s.chars().collect()).collect();
}

fn get_index(arr: &Vec<HashSet<char>>, target: HashSet<char>) -> usize {
    for (i, val) in arr.iter().enumerate() {
        if val == &target {
            return i;
        }
    }
    println!("ERRORRR!");
    return 11;
}
