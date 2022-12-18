use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let jets: Vec<char> = contents.lines().next().unwrap().chars().collect();

    let mut blocked_coords: HashSet<(i64, i64)> = HashSet::new();
    let mut pile_height: i64 = 0;

    let mut jet_index: usize = 0;

    for rock_index in 0..2022 {
        let start_height: i64 = pile_height + 3;
        let start_x: i64 = 2;
        let mut falling_rock: Vec<(i64, i64)> = match rock_index % 5 {
            // ####
            0 => vec![(start_x, start_height),
                      (start_x + 1, start_height),
                      (start_x + 2, start_height),
                      (start_x + 3, start_height),
            ],
            /*
               .#.
               ###
               .#.
            */
            1 => vec![(start_x + 1, start_height),
                      (start_x, start_height + 1),
                      (start_x + 1, start_height + 1),
                      (start_x + 2, start_height + 1),
                      (start_x + 1, start_height + 2),
            ],
            /*
               ..#
               ..#
               ###
            */
            2 => vec![(start_x, start_height),
                      (start_x + 1, start_height),
                      (start_x + 2, start_height),
                      (start_x + 2, start_height + 1),
                      (start_x + 2, start_height + 2),
            ],
            /*
               #
               #
               #
               #
            */
            3 => vec![(start_x, start_height),
                      (start_x, start_height + 1),
                      (start_x, start_height + 2),
                      (start_x, start_height + 3),
            ],
            /*
               ##
               ##
            */
            4 => vec![(start_x, start_height),
                      (start_x + 1, start_height),
                      (start_x, start_height + 1),
                      (start_x + 1, start_height + 1),
            ],
            _ => vec![],
        };

        loop {
            let jet_move_valid: bool = falling_rock.iter().all(|(x, y)| {
                let new_x: i64;
                if jets[jet_index] == '<' {
                    new_x = x - 1;
                } else {
                    new_x = x + 1;
                }
                return new_x >= 0 && new_x < 7 && !blocked_coords.contains(&(new_x, *y));
            });

            if jet_move_valid {
                falling_rock = falling_rock.iter().map(|(x, y)| {
                    if jets[jet_index] == '<' {
                        return (x - 1, *y);
                    } else {
                        return (x + 1, *y);
                    }
                }).collect();
            }

            jet_index = (jet_index + 1) % jets.len();

            let downward_move_valid: bool = falling_rock.iter().all(|(x, y)| {
                return *y - 1 >= 0 && !blocked_coords.contains(&(*x, *y - 1));
            });

            if downward_move_valid {
                falling_rock = falling_rock.iter().map(|(x, y)| (*x, y - 1)).collect();
            } else {
                for pair in &falling_rock {
                    blocked_coords.insert(*pair);
                }
                break;
            }
        }

        pile_height = *blocked_coords.iter().map(|(_, y)| y).max().unwrap() + 1;
    }
    for row in (0..pile_height).rev() {
        print!("|");
        for col in 0..7 {
            if blocked_coords.contains(&(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }

//    println!("result: {}", pile_height);
}
