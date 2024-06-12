use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::fs::read_to_string;

extern crate common;
use common::first_and_last;

lazy_static! {
    static ref OUTCOME_SCORES: HashMap<&'static str, u32> = {
        hashmap! {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
        }
    };
    static ref SHAPE_SCORES: HashMap<&'static str, u32> = {
        hashmap! {
            "A" => 1,
            "B" => 2,
            "C" => 3,
        }
    };
    static ref WIN: HashMap<&'static str, &'static str> = {
        hashmap! {
            "A" => "B",
            "B" => "C",
            "C" => "A",
        }
    };
    static ref LOSE: HashMap<&'static str, &'static str> = {
        hashmap! {
            "A" => "C",
            "B" => "A",
            "C" => "B",
        }
    };
}

// Day #2
fn main() {
    let lines: Vec<String> = read_lines("day02/input.txt");

    let result: u32 = lines
        .iter()
        .map(|line| {
            let (first, last) = first_and_last!(line);
            get_round_score(first, last)
        })
        .sum();

    println!("{:?}", result);
}

fn get_round_score(opponent_move: &str, outcome: &str) -> u32 {
    // println!("{:?}, {:?}", opponent_move, my_move);

    let shape_score = if outcome == "X" {
        SHAPE_SCORES[LOSE[opponent_move]]
    } else if outcome == "Z" {
        SHAPE_SCORES[WIN[opponent_move]]
    } else {
        SHAPE_SCORES[opponent_move]
    };

    shape_score + OUTCOME_SCORES[outcome]
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // Panic on possible file-reading errors
        .lines() // Split the string into an iterator of string slices
        .map(String::from) // Make each slice into a string
        .collect() // Gather them together into a vector
}
