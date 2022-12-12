use std::env;
use std::fs;
use std::fmt::Debug;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64, i64) -> i64>,
    op_val: i64,
    divisor_check: i64,
    true_case: usize,
    false_case: usize,
    check_count: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let blocks = contents.split("\n\n");

    let mut monkeys: Vec<Monkey> = blocks.map(|block| {
        let lines: Vec<&str> = block.lines().collect();

        let items: Vec<i64> = lines[1].split(": ").nth(1).unwrap().split(", ").map(|val| val.parse::<i64>().unwrap()).collect();

        let op_tokens: Vec<&str> = lines[2].split_whitespace().collect();
        let operation: Box<dyn Fn(i64, i64) -> i64>;
        let op_val: i64;
        if op_tokens[5] == "old" {
            operation = Box::new(|x, _val| x * x);
            op_val = 0;
        } else {
            op_val = op_tokens[5].parse::<i64>().unwrap();
            if op_tokens[4] == "*" {
                operation = Box::new(|x, val| x * val);
            } else {
                operation = Box::new(|x, val| x + val);
            }
        }

        let divisor_check: i64 = parse_tail::<i64>(lines[3]);

        let true_case: usize = parse_tail::<usize>(lines[4]);
        let false_case: usize = parse_tail::<usize>(lines[5]);

        return Monkey {
            items: items,
            operation: operation,
            op_val: op_val,
            divisor_check: divisor_check,
            true_case: true_case,
            false_case: false_case,
            check_count: 0,
        }
    }).collect();

    for _ in 0..20 {
        let count: usize = monkeys.len();
        for i in 0..count {
            let mut monkey = &mut monkeys[i];

            monkey.check_count += monkey.items.len();

            let queue: Vec<(usize, i64)> = monkey.items.iter().map(|item| {
                let new_val: i64 = (monkey.operation)(*item, monkey.op_val) / 3;
                if new_val % monkey.divisor_check == 0 {
                    return (monkey.true_case, new_val);
                } else {
                    return (monkey.false_case, new_val);
                }
            }).collect();
            monkey.items = Vec::new();

            for (dest, val) in queue.iter() {
                let dest_monkey = &mut monkeys[*dest];
                dest_monkey.items.push(*val);
            }
        }
    }

    let mut counts: Vec<usize> = monkeys.iter().map(|m| m.check_count).collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();

    let result: usize = counts[0] * counts[1];

    println!("result: {}", result);
}

fn parse_tail<T: std::str::FromStr>(line: &str) -> T where <T as std::str::FromStr>::Err: Debug {
    return line.split_whitespace().last().unwrap().parse::<T>().unwrap();
}
