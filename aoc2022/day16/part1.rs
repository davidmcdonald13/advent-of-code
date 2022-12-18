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

    let result: i64 = dfs(&mut HashMap::new(), "AA", &HashSet::new(), 30, &valves, &tunnels);

    println!("result: {}", result);
}

#[derive(PartialEq, Eq, Hash)]
struct MemoKey {
    val: String,
}

fn get_memo_key(curr: &str, open_valves: &HashSet<&str>, mins_remaining: i64) -> MemoKey {
    let mut v: Vec<&&str> = open_valves.into_iter().collect();
    v.sort();

    let mut result: String = curr.to_string();
    for val in v.iter() {
        result += val;
    }

    result += &mins_remaining.to_string();

    return MemoKey { val: result };
}

fn dfs(memo: &mut HashMap<MemoKey, i64>, curr: &str, open_valves: &HashSet<&str>, mins_remaining: i64,
       valves: &HashMap<&str, i64>, tunnels: &HashMap<&str, Vec<&str>>) -> i64 {
    let key: MemoKey = get_memo_key(curr, open_valves, mins_remaining);
    if memo.contains_key(&key) {
        return *(memo.get(&key).unwrap());
    }

    if mins_remaining <= 0 {
        return 0;
    }

    let current_rate: i64 = open_valves.iter().map(|valve| valves.get(valve).unwrap()).sum();

    let mut result: i64 = i64::MIN;

    if !open_valves.contains(curr) && valves.get(curr).unwrap() > &0 {
        let mut open_valves_copy: HashSet<&str> = open_valves.clone();
        open_valves_copy.insert(curr);

        result = current_rate + dfs(memo, curr, &open_valves_copy, mins_remaining - 1, valves, tunnels);
    }

    for dest in tunnels.get(curr).unwrap() {
        result = cmp::max(result, current_rate + dfs(memo, dest, open_valves, mins_remaining - 1, valves, tunnels));
    }

    memo.insert(key, result);
    return result;
}
