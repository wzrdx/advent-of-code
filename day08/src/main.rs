use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Day #8
pub fn main() -> Result<()> {
    let file = File::open("day08/input.txt")?;
    let reader = BufReader::new(file);

    let matrix: Vec<Vec<u8>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    // Print the matrix
    for row in &matrix {
        println!("{:?}", row);
    }

    Ok(())
}
