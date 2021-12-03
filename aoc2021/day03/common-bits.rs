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

    for measure in &measures {
        measure.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                one_counts[i] += 1;
            }
        });
    }

    let dominant_chars: Vec<char> = one_counts.into_iter().map(|count| {
        if (2 * count) >= total {
            return '1';
        } else {
            return '0';
        }
    }).collect();

    get_power(&dominant_chars);
    println!();
    get_life_support(measures);
}

// this function is overly repetitive, but I'm tired and want to go to bed
fn get_life_support(measures: Vec<&str>) {
    let mut oxygen_candidates: Vec<&str> = measures.clone();
    let mut index: usize = 0;
    while oxygen_candidates.len() > 1 {
        let ones: usize = oxygen_candidates.iter().map(|measure| if measure.chars().nth(index).unwrap() == '1' { 1 } else { 0 }).sum();
        let target_char: char = if (2 * ones) >= oxygen_candidates.len() { '1' } else { '0' };
        oxygen_candidates = oxygen_candidates.into_iter().filter(|measure| measure.chars().nth(index).unwrap() == target_char).collect();
        index += 1
    }
    let oxygen: usize = usize::from_str_radix(&oxygen_candidates[0], 2).unwrap();

    let mut co2_candidates: Vec<&str> = measures.clone();
    index = 0;
    while co2_candidates.len() > 1 {
        let ones: usize = co2_candidates.iter().map(|measure| if measure.chars().nth(index).unwrap() == '1' { 1 } else { 0 }).sum();
        let target_char: char = if (2 * ones) < co2_candidates.len() { '1' } else { '0' };
        co2_candidates = co2_candidates.into_iter().filter(|measure| measure.chars().nth(index).unwrap() == target_char).collect();
        index += 1
    }
    let co2: usize = usize::from_str_radix(&co2_candidates[0], 2).unwrap();

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    println!("life support: {}", oxygen * co2);
}

fn get_power(dominant_chars: &Vec<char>) {
    let gamma_vec: Vec<String> = dominant_chars.iter().map(|dom| dom.to_string()).collect();
    let epsilon_vec: Vec<String> = dominant_chars.iter().map(|dom| if *dom == '1' { '0'.to_string() } else { '1'.to_string() }).collect();

    let gamma: usize = vec_to_int(gamma_vec);
    let epsilon: usize = vec_to_int(epsilon_vec);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("power consumption: {}", gamma * epsilon);
}

fn vec_to_int(v: Vec<String>) -> usize  {
    let s: String = v.join("");
    return usize::from_str_radix(&s, 2).unwrap();
}
