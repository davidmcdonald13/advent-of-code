use std::collections::HashSet;
use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let mut chunks: str::Split<&str> = contents.trim().split("\n\n");

    let moves: Vec<&str> = chunks.next().unwrap().split(",").collect();

    let boards: Vec<Vec<Vec<&str>>> = chunks.map(|chunk| {
        chunk.split("\n").map(|row| row.split_ascii_whitespace().collect()).collect()
    }).collect();

    let mut performed_moves: HashSet<&str> = HashSet::new();
    for this_move in moves {
        performed_moves.insert(this_move);
        for board in &boards {
            if winning_board(&board, &performed_moves) {
                let unused: usize = get_unused_sum(&board, &performed_moves);
                println!("unused: {}", unused);
                println!("last move: {}", this_move);
                println!("result: {}", unused * this_move.parse::<usize>().unwrap());
                return;
            }
        }
    }
}

fn winning_board(board: &Vec<Vec<&str>>, performed_moves: &HashSet<&str>) -> bool {
    // check all rows
    for row in board.iter() {
        if row.iter().all(|x| performed_moves.contains(x)) {
            return true;
        }
    }

    // check all columns
    for col in 0..5 {
        let mut col_filled: bool = true;
        for row in board.iter() {
            if !performed_moves.contains(row[col]) {
                col_filled = false;
                break;
            }
        }
        if col_filled {
            return true;
        }
    }

    return false;
}

fn get_unused_sum(board: &Vec<Vec<&str>>, performed_moves: &HashSet<&str>) -> usize {
    return board.iter().map(|row| {
        return row.iter().map(|val| {
            if performed_moves.contains(val) {
                return 0;
            } else {
                return val.parse::<usize>().unwrap();
            }
        }).sum::<usize>();
    }).sum::<usize>();
}
