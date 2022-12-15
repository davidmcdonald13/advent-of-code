use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let target_row: i64 = args[2].parse::<i64>().unwrap();

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut result: HashSet<(i64, i64)> = HashSet::new();
    let mut known_beacons_target_row: HashSet<(i64, i64)> = HashSet::new();

    for line in lines {
        let (sensor, beacon): ((i64, i64), (i64, i64)) = parse_line(line);

        if beacon.1 == target_row {
            known_beacons_target_row.insert(beacon);
        }

        let distance: i64 = get_manhattan_distance(sensor, beacon);
        let min_x: i64 = sensor.0 - (distance - (sensor.1 - target_row).abs());
        let max_x: i64 = sensor.0 + (distance - (sensor.1 - target_row).abs());
        for x in min_x..(max_x+1) {
            result.insert((x, target_row));
        }
    }

    println!("result: {}", result.len() - known_beacons_target_row.len());
}

fn get_manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn parse_line(line: &str) -> ((i64, i64), (i64, i64)) {
    // example input: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // e.g. x=2,
    let sensor_x: i64 = tokens[2].split(&['=', ','][..]).nth(1).unwrap().parse::<i64>().unwrap();
    // e.g. y=18:
    let sensor_y: i64 = tokens[3].split(&['=', ':'][..]).nth(1).unwrap().parse::<i64>().unwrap();

    // e.g. x=-2,
    let beacon_x: i64 = tokens[8].split(&['=', ','][..]).nth(1).unwrap().parse::<i64>().unwrap();
    // e.g. y=15
    let beacon_y: i64 = tokens[9].split(&['='][..]).nth(1).unwrap().parse::<i64>().unwrap();

    return ((sensor_x, sensor_y), (beacon_x, beacon_y));
}
