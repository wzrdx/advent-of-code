use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Each property represents the max. height of a tree in that direction
#[derive(Debug, Clone)]
struct Info {
    north: u8,
    east: u8,
    south: u8,
    west: u8,
}

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

    // The max heights in each direction of the inner trees
    let mut max_heights: Vec<Vec<Option<Info>>> =
        vec![vec![None; matrix.len() - 2]; matrix.len() - 2];

    // let edge_trees = (matrix.len() - 1) * 4;
    // println!("Edge trees: {:?}", edge_trees);

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            // println!("{}", matrix[i][j]);

            // North
            let north_max_height = if i == 1 {
                matrix[i - 1][j]
            } else {
                if let Some(ref info) = max_heights[i - 2][j - 1] {
                    info.north
                } else {
                    panic!("North: Index out of bounds");
                }
            };

            // East
            let east_max_height = if j == 1 {
                matrix[i][j - 1]
            } else {
                if let Some(ref info) = max_heights[i - 1][j - 2] {
                    info.east
                } else {
                    panic!("North: Index out of bounds");
                }
            };

            let pos_n_max_height = cmp::max(north_max_height, matrix[i][j]);
            let pos_e_max_height = cmp::max(east_max_height, matrix[i][j]);

            if let Some(ref mut info) = max_heights[i - 1][j - 1] {
                info.north = pos_n_max_height;
                info.east = pos_e_max_height;
            } else {
                max_heights[i - 1][j - 1] = Some(Info {
                    north: pos_n_max_height,
                    east: pos_e_max_height,
                    south: 0,
                    west: 0,
                });
            }
        }
    }

    for row in &max_heights {
        for elem in row {
            print!("{:?} ", elem);
        }

        println!();
    }

    Ok(())
}
