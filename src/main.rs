use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./input.txt");

    let mut calories: Vec<u32> = Vec::new();
    let mut value: u32 = 0;

    lines.iter().enumerate().for_each(|(i, s)| {
        if s.is_empty() || i == lines.len() - 1 {
            calories.push(value);
            value = 0;
        } else {
            let number: u32 = s.parse::<u32>().unwrap();
            value += number;
        }
    });

    calories.sort();

    let values = &calories[calories.len() - 3..];
    let sum: u32 = values.iter().sum();

    println!("{:?}", calories);
    println!("{:?}", values);
    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // Panic on possible file-reading errors
        .lines() // Split the string into an iterator of string slices
        .map(String::from) // Make each slice into a string
        .collect() // Gather them together into a vector
}
