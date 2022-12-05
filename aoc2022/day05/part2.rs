use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 0 {
            continue;
        } else if chars[0] == 'm' {
            // parse move instructions
            let tokens: Vec<&str> = line.split(' ').collect();

            let count: usize = tokens[1].parse::<usize>().unwrap();
            let src: usize = tokens[3].parse::<usize>().unwrap() - 1;
            let dest: usize = tokens[5].parse::<usize>().unwrap() - 1;

            let len = stacks[src].len();

            for i in (len-count)..len {
                let src_stack: &Vec<char> = &stacks[src];
                let val = src_stack[i];
                let dest_stack: &mut Vec<char> = &mut stacks[dest];
                dest_stack.push(val);
            }

            let src_stack: &mut Vec<char> = &mut stacks[src];
            src_stack.resize(len - count, 'x');
        } else if line.contains('[') {
            // parse stack contents
            let len = line.len();
            for i in (1..len).step_by(4) {
                let index = i / 4;
                if stacks.len() < (index + 1) {
                    stacks.push(Vec::new());
                }

                let c: char = line.chars().nth(i).unwrap();
                if c != ' ' {
                    let dest_stack: &mut Vec<char> = &mut stacks[index];
                    dest_stack.insert(0, c);
                }
            }
        }
    }

    let result: String = stacks.iter().map(|s| {
        let len = s.len();
        return s.get(len - 1).unwrap();
    }).collect();

    println!("result: {}", result);
}
