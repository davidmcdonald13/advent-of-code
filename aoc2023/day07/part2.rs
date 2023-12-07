use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename).expect("failed to read file");
    let lines: std::str::Lines = contents.lines();

    let mut hands: Vec<(Hand, usize)> = lines.map(|line| parse_hand(line))
                                             .collect();
    hands.sort();

    let result: usize = hands.into_iter()
                             .enumerate()
                             .map(|(i, (_hand, bet))| (i + 1) * bet)
                             .sum();

    println!("result: {}", result);
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    rank: HandRank,
    cards: Vec<i8>,
}

fn parse_hand(s: &str) -> (Hand, usize) {
    let tokens: Vec<&str> = s.split_ascii_whitespace().collect();

    let cards: Vec<i8> = tokens[0].chars().map(|x| parse_card(x)).collect();

    let mut card_counts: HashMap<i8, usize> = HashMap::new();
    cards.iter()
         .for_each(|card| {
             if card_counts.contains_key(&card) {
                 card_counts.insert(*card, card_counts.get(&card).unwrap() + 1);
             } else {
                 card_counts.insert(*card, 1);
             }
         });

    if card_counts.len() > 1 && card_counts.contains_key(&0) {
        // remove the wildcard count from the map, then add it to the count of the most common card
        let wildcard_count: usize = card_counts.remove(&0).unwrap();

        let high_value: usize = *card_counts.values().max().unwrap();

        for (key, value) in &card_counts {
            if *value == high_value {
                card_counts.insert(*key, value + wildcard_count);
                break;
            }
        }
    }

    let rank: HandRank = match card_counts.len() {
        5 => HandRank::HighCard,
        4 => HandRank::OnePair,
        3 =>  {
            if card_counts.values().max().unwrap() == &3_usize {
                HandRank::ThreeOfAKind
            } else {
                HandRank::TwoPair
            }
        },
        2 => {
            if card_counts.values().max().unwrap() == &4_usize {
                HandRank::FourOfAKind
            } else {
                HandRank::FullHouse
            }
        },
        1 => HandRank::FiveOfAKind,
        _ => HandRank::HighCard,
    };

    (Hand {
        rank,
        cards,
    }, tokens[1].parse::<usize>().unwrap())
}

fn parse_card(card: char) -> i8 {
    return "J23456789TQKA".chars().position(|x| x == card).unwrap() as i8;
}
