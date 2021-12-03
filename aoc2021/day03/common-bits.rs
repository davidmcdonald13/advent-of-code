use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let measures: Vec<&str> = contents.lines().collect();

    let measure_length: usize = measures[0].len();
    let total: usize = measures.len();

    let mut one_counts: Vec<usize> = vec![0; measure_length];

    for measure in measures {
        measure.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                one_counts[i] += 1;
            }
        });
    }

    let mut gamma_vec: Vec<String> = vec!["0".to_string(); measure_length];
    let mut epsilon_vec: Vec<String> = vec!["0".to_string(); measure_length];
    one_counts.iter().enumerate().for_each(|(i, count)| {
        if count * 2 > total {
            gamma_vec[i] = "1".to_string();
        } else {
            epsilon_vec[i] = "1".to_string();
        }
    });

    let gamma: usize = vec_to_int(gamma_vec);
    let epsilon: usize = vec_to_int(epsilon_vec);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("result: {}", gamma * epsilon);
}

fn vec_to_int(v: Vec<String>) -> usize  {
    let s: String = v.join("");
    return usize::from_str_radix(&s, 2).unwrap();
}
