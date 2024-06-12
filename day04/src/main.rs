use std::fs::read_to_string;

extern crate common;
use common::first_and_last;

// Day #4
fn main() {
    let lines: Vec<String> = read_lines("./input.txt");

    let result: u32 = lines
        .iter()
        .map(|line: &String| {
            let (first, second) = first_and_last!(line, ',');
            let value = get_overlap(first_and_last!(first, '-'), first_and_last!(second, '-'));

            value
        })
        .sum();

    println!("{:?}", result);
}

fn get_overlap(first: (&str, &str), second: (&str, &str)) -> u32 {
    let first_n: (u32, u32) = (
        (first.0).parse::<u32>().unwrap_or_default(),
        (first.1).parse::<u32>().unwrap_or_default(),
    );

    let second_n: (u32, u32) = (
        (second.0).parse::<u32>().unwrap_or_default(),
        (second.1).parse::<u32>().unwrap_or_default(),
    );

    if (first_n.1 < second_n.0) || (first_n.0 > second_n.1) {
        0
    } else {
        1
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // Panic on possible file-reading errors
        .lines() // Split the string into an iterator of string slices
        .map(String::from) // Make each slice into a string
        .collect() // Gather them together into a vector
}
