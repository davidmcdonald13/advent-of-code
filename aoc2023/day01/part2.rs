use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let result: i64 = lines.map(|line| combine_first_last_digit(line)).sum();

    println!("result: {}", result);
    println!("must be greater than 54740");
}

struct Number<'a> {
    name: &'a str,
    value: i64,
}

fn combine_first_last_digit(line: &str) -> i64 {
    let numbers: Vec<Number> = vec![
        Number {
            name: "1",
            value: 1,
        },
        Number {
            name: "2",
            value: 2,
        },
        Number {
            name: "3",
            value: 3,
        },
        Number {
            name: "4",
            value: 4,
        },
        Number {
            name: "5",
            value: 5,
        },
        Number {
            name: "6",
            value: 6,
        },
        Number {
            name: "7",
            value: 7,
        },
        Number {
            name: "8",
            value: 8,
        },
        Number {
            name: "9",
            value: 9,
        },
        Number {
            name: "one",
            value: 1,
        },
        Number {
            name: "two",
            value: 2,
        },
        Number {
            name: "three",
            value: 3,
        },
        Number {
            name: "four",
            value: 4,
        },
        Number {
            name: "five",
            value: 5,
        },
        Number {
            name: "six",
            value: 6,
        },
        Number {
            name: "seven",
            value: 7,
        },
        Number {
            name: "eight",
            value: 8,
        },
        Number {
            name: "nine",
            value: 9,
        },
    ];

    let first_value: i64 = find_first_value(line, &numbers);
    let last_value: i64 = find_last_value(line, &numbers);

    return 10 * first_value + last_value;
}

fn find_first_value(line: &str, numbers: &Vec<Number>) -> i64 {
    let mut result: i64 = 0;
    let mut result_index: usize = usize::max_value();
    for num in numbers {
        let index = line.find(num.name);
        if index.is_none() {
            continue;
        }

        if index.unwrap() < result_index {
            result_index = index.unwrap();
            result = num.value;
        }
    }

    return result;
}

fn find_last_value(line: &str, numbers: &Vec<Number>) -> i64 {
    let mut result: i64 = 0;
    let mut result_index: usize = usize::min_value();
    for num in numbers {
        let index = line.rfind(num.name);
        if index.is_none() {
            continue;
        }

        if index.unwrap() >= result_index {
            result_index = index.unwrap();
            result = num.value;
        }
    }

    return result;
}
