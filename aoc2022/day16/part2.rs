use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut valves: HashMap<&str, i64> = HashMap::new();
    let mut tunnels: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let node: &str = tokens[1];

        // e.g. rate=13;
        let rate: i64 = tokens[4].split(&['=', ';'][..]).nth(1).unwrap().parse::<i64>().unwrap();

        let paths: Vec<&str> = tokens[9..].iter().map(|token| token.split(',').next().unwrap()).collect();

        valves.insert(node, rate);
        tunnels.insert(node, paths);
    }

    let result: i64 = dfs(&mut HashMap::new(), "AA", "AA", &HashSet::new(), 26, &valves, &tunnels);

    println!("result: {}", result);
}

#[derive(PartialEq, Eq, Hash)]
struct MemoKey {
    val: String,
}

fn get_memo_key(person: &str, elephant: &str, open_valves: &HashSet<&str>, mins_remaining: i64) -> MemoKey {
    let mut v: Vec<&&str> = open_valves.into_iter().collect();
    v.sort();

    let mut result: String = cmp::max(person, elephant).to_string();
    result += &cmp::min(person, elephant).to_string();
    for val in v.iter() {
        result += val;
    }

    result += &mins_remaining.to_string();

    return MemoKey { val: result };
}

fn dfs(memo: &mut HashMap<MemoKey, i64>, person: &str, elephant: &str, open_valves: &HashSet<&str>, mins_remaining: i64,
       valves: &HashMap<&str, i64>, tunnels: &HashMap<&str, Vec<&str>>) -> i64 {
    let key: MemoKey = get_memo_key(person, elephant, open_valves, mins_remaining);
    if memo.contains_key(&key) {
        return *(memo.get(&key).unwrap());
    }

    if mins_remaining <= 0 {
        return 0;
    }

    let current_rate: i64 = open_valves.iter().map(|valve| valves.get(valve).unwrap()).sum();

    let mut result: i64 = 0;

    if !open_valves.contains(person) && valves.get(person).unwrap() > &0 {
        let mut open_valves_copy: HashSet<&str> = open_valves.clone();
        open_valves_copy.insert(person);

        for elephant_dest in tunnels.get(elephant).unwrap() {
            if mins_remaining == 26 {
                println!("recursing into {}, {}", person, elephant_dest);
            }
            result = cmp::max(result, current_rate + dfs(memo, person, elephant_dest, &open_valves_copy, mins_remaining - 1, valves, tunnels));
        }
    }
    if !open_valves.contains(elephant) && valves.get(elephant).unwrap() > &0 {
        let mut open_valves_copy: HashSet<&str> = open_valves.clone();
        open_valves_copy.insert(elephant);

        for person_dest in tunnels.get(person).unwrap() {
            if mins_remaining == 26 {
                println!("recursing into {}, {}", person_dest, elephant);
            }
            result = cmp::max(result, current_rate + dfs(memo, person_dest, elephant, &open_valves_copy, mins_remaining - 1, valves, tunnels));
        }
    }
    if person != elephant && !open_valves.contains(person) && !open_valves.contains(elephant) {
        let mut open_valves_copy: HashSet<&str> = open_valves.clone();
        open_valves_copy.insert(person);
        open_valves_copy.insert(elephant);

        result = cmp::max(result, current_rate + dfs(memo, person, elephant, &open_valves_copy, mins_remaining - 1, valves, tunnels));
    }

    for person_dest in tunnels.get(person).unwrap() {
        for elephant_dest in tunnels.get(elephant).unwrap() {
            if mins_remaining == 26 {
                println!("recursing into {}, {}", person_dest, elephant_dest);
            }
            result = cmp::max(result, current_rate + dfs(memo, person_dest, elephant_dest, open_valves, mins_remaining - 1, valves, tunnels));
        }
    }

    memo.insert(key, result);
    //println!("found partial result: ({}, {}, {:?}, {}) = {}", person, elephant, open_valves, mins_remaining, result);
    return result;
}
