use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let search_range: i64 = args[2].parse::<i64>().unwrap();

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut sensors: HashMap<(i64, i64), i64> = HashMap::new();

    for line in lines {
        let (sensor, beacon): ((i64, i64), (i64, i64)) = parse_line(line);
        let distance: i64 = get_manhattan_distance(sensor, beacon);

        sensors.insert(sensor, distance);
    }

    for x in 0..(search_range + 1) {
        let mut y: i64 = 0;
        while y <= search_range {
            if x % 100 == 0 {
                println!("({}, {})", x, y);
            }
            let mut found: bool = false;
            for (sensor, range) in &sensors {
                let distance_to_sensor: i64 = get_manhattan_distance(*sensor, (x, y));
                if distance_to_sensor <= *range {
                    found = true;
                    y = sensor.1 + range - (x - sensor.0).abs() + 1;
                    break;
                }
            }
            if !found {
                println!("x={}, y={}, result={}", x, y, x * 4000000 + y);
                return;
            }
        }
    }
    println!("failed to find a result");
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
