use std::collections::HashSet;
use std::fs::read_to_string;

// Day #3
fn main() {
    let lines: Vec<String> = read_lines("day03/input.txt");

    let result: u32 = lines
        .chunks(3)
        .map(|chunk| {
            let character = get_intersection(chunk);
            let priority = get_priority(character);

            priority as u32
        })
        .sum();

    println!("{:?}", result);
}

fn get_priority(character: char) -> u8 {
    let ascii_value: u8 = character as u8;

    if character.is_lowercase() {
        ascii_value - 96
    } else {
        ascii_value - 38
    }
}

fn get_intersection(chunk: &[String]) -> char {
    let (x, y, z) = (chunk[0].clone(), chunk[1].clone(), chunk[2].clone());

    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();
    let z_chars: Vec<char> = z.chars().collect();

    let x_set: HashSet<_> = x_chars.into_iter().collect();
    let y_set: HashSet<_> = y_chars.into_iter().collect();
    let z_set: HashSet<_> = z_chars.into_iter().collect();

    let xy_intersection: HashSet<_> = x_set.intersection(&y_set).cloned().collect();
    let xyz_intersection: &char = xy_intersection.intersection(&z_set).next().unwrap_or(&'$');

    xyz_intersection.clone()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // Panic on possible file-reading errors
        .lines() // Split the string into an iterator of string slices
        .map(String::from) // Make each slice into a string
        .collect() // Gather them together into a vector
}
