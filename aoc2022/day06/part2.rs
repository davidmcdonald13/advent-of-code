use std::env;
use std::fs;

fn main() {
    test();

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let mut lines: std::str::Lines = contents.lines();

    let result = run(lines.next().unwrap());

    println!("result: {}", result);
}

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 14..len {
        let slice = &chars[(i-14)..i];
        if all_unique(slice) {
            return i;
        }
    }
    return 0;
}

fn all_unique(slice: &[char]) -> bool {
    for i in 0..14 {
        for j in (i+1)..14 {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    return true;
}

fn test() {
    assert!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19);
    assert!(run("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23);
    assert!(run("nppdvjthqldpwncqszvftbrmjlhg") == 23);
    assert!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29);
    assert!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26);
}
