use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let total_disk_size: i64 = 70000000;
    let needed_free_space: i64 = 30000000;

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut direct_sizes: HashMap<String, i64> = HashMap::new();
    let mut child_dirs: HashMap<String, HashSet<String>> = HashMap::new();

    let mut path: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let tokens: Vec<&str> = lines[i].split_ascii_whitespace().collect();

        i += 1;
        if tokens[1] == "cd" {
            if tokens[2] == "/" {
                continue;
            } else if tokens[2] == ".." {
                path.pop();
            } else {
                path.push(tokens[2]);
            }
        } else {
            let dir_path: String = path_to_string(&path);
            if !direct_sizes.contains_key(&dir_path) {
                direct_sizes.insert(dir_path.clone(), 0);
            }
            if !child_dirs.contains_key(&dir_path) {
                child_dirs.insert(dir_path.clone(), HashSet::new());
            }

            while i < lines.len() && !lines[i].starts_with('$') {
                let vals: Vec<&str> = lines[i].split_ascii_whitespace().collect();
                if vals[0].starts_with('d') {
                    let child_path: String;
                    if dir_path == "/" {
                        child_path = "/".to_owned() + vals[1];
                    } else {
                        child_path = dir_path.clone() + "/" + vals[1];
                    }
                    child_dirs.get_mut(&dir_path).unwrap().insert(child_path);
                } else {
                    let size: i64 = vals[0].parse::<i64>().unwrap();
                    *direct_sizes.get_mut(&dir_path).unwrap() += size;
                }
                i += 1;
            }
        }
    }

    let current_used_space: i64 = direct_sizes.values().sum();

    let mut result: i64 = i64::MAX;
    for dir in direct_sizes.keys() {
        let size: i64 = get_directory_size(dir.to_string(), &direct_sizes, &child_dirs);
        if current_used_space - size < total_disk_size - needed_free_space {
            result = cmp::min(result, size);
        }
    }

    println!("result: {}", result);
}

fn path_to_string(path: &Vec<&str>) -> String {
    return "/".to_owned() + &path.join("/");
}

fn get_directory_size(root: String, sizes: &HashMap<String, i64>, children: &HashMap<String, HashSet<String>>) -> i64 {
    if !sizes.contains_key(&root) {
        println!("ignoring directory: {}", root);
        return 0;
    }

    let mut result: i64 = *sizes.get(&root).unwrap();
    for child in children.get(&root).unwrap() {
        println!("descending into: {}", child);
        result += get_directory_size(child.to_string(), sizes, children);
    }
    return result;
}
