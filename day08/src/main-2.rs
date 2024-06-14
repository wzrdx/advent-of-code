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

    let size = matrix.len();

    println!("{:?}", matrix);

    let mut max = 0;

    for i in 1..(size - 1) {
        for j in 1..(size - 1) {
            let current = matrix[i][j];

            println!("Current: {:?}", current);

            let south = ((i + 1)..size)
                .map(|k| matrix[k][j])
                .position(|n| n >= current)
                .unwrap_or_else(|| size - 2 - i)
                + 1;

            println!("South: {:?}", south);

            let north = (0..i)
                .rev()
                .map(|k| matrix[k][j])
                .position(|n| n >= current)
                .unwrap_or_else(|| i - 1)
                + 1;

            println!("North: {:?}", north);

            let west = (0..j)
                .rev()
                .map(|k| matrix[i][k])
                .position(|n| n >= current)
                .unwrap_or_else(|| j - 1)
                + 1;

            println!("West: {:?}", west);

            let east = ((j + 1)..size)
                .map(|k| matrix[i][k])
                .position(|n| n >= current)
                .unwrap_or_else(|| size - 2 - j)
                + 1;

            println!("East: {:?}", east);

            println!();
        }
    }

    Ok(())
}
