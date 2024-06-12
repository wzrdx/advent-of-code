use itertools::join;
use std::fs::read_to_string;

// Day #5
fn main() {
    let lines: Vec<String> = read_lines("day05/input.txt");

    let index = lines
        .iter()
        .position(|s| s.starts_with("move"))
        .unwrap_or_default();
    let state: Vec<String> = lines[..index - 2].to_vec().into_iter().rev().collect();

    let first_line: String = state
        .get(0)
        .map_or_else(|| String::default(), String::clone);
    let count: usize = (first_line.len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); count];

    for line in state.into_iter() {
        for i in 0..count {
            let c = line.chars().nth((i * 4) + 1).unwrap_or_default();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    let moves = lines[index..].to_vec();
    let substrings_to_remove = ["move ", "from ", "to "];

    for line in moves.into_iter() {
        let line_numbers = substrings_to_remove
            .iter()
            .fold(line, |s, &substring| s.replace(substring, ""));

        let mut split_values = line_numbers.split_whitespace();

        let amount: usize = split_values.next().unwrap_or_default().parse().unwrap();
        let from: usize = split_values.next().unwrap_or_default().parse().unwrap();
        let to: usize = split_values.next().unwrap_or_default().parse().unwrap();

        // let mut cargo: Vec<char> = stacks[from - 1].iter().rev().take(amount).cloned().collect();
        let remaining_length: usize = stacks[from - 1].len() - amount;
        let cargo: Vec<char> = stacks[from - 1].drain(remaining_length..).collect();

        stacks[to - 1].extend(cargo);
    }

    let result: Vec<char> = stacks
        .iter()
        .map(|stack| {
            let last = stack.clone().pop().unwrap_or_default();
            last
        })
        .collect();

    let message = join(result.iter(), "");

    println!("{:?}", stacks);
    println!("{:?}", message);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // Panic on possible file-reading errors
        .lines() // Split the string into an iterator of string slices
        .map(String::from) // Make each slice into a string
        .collect() // Gather them together into a vector
}
