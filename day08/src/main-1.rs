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

    let mut visible_trees = 0;

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
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
                    panic!("East: Index out of bounds");
                }
            };

            // South
            let south_max_height = if i == matrix.len() - 2 {
                matrix[matrix.len() - 1][j]
            } else {
                if i == 1 {
                    // Compute the whole column from end to i = 2
                    for k in (1..(matrix.len() - 2)).rev() {
                        let temp_south_max_height = if k == max_heights.len() - 1 {
                            matrix[matrix.len() - 1][j]
                        } else {
                            if let Some(ref info) = max_heights[k + 1][j - 1] {
                                info.south
                            } else {
                                panic!("South (temporary): Index out of bounds");
                            }
                        };

                        let temp_pos_s_max_height =
                            cmp::max(temp_south_max_height, matrix[k + 1][j]);

                        max_heights[k][j - 1] = Some(Info {
                            north: 0,
                            east: 0,
                            south: temp_pos_s_max_height,
                            west: 0,
                        });
                    }
                }

                if let Some(ref info) = max_heights[i][j - 1] {
                    info.south
                } else {
                    panic!("South: Index out of bounds");
                }
            };

            // West
            let west_max_height = if j == matrix.len() - 2 {
                matrix[i][matrix.len() - 1]
            } else {
                if j == 1 {
                    // Compute the whole row from end to j = 2
                    for k in (1..(matrix.len() - 2)).rev() {
                        let temp_west_max_height = if k == max_heights.len() - 1 {
                            matrix[i][matrix.len() - 1]
                        } else {
                            if let Some(ref info) = max_heights[i - 1][k + 1] {
                                info.west
                            } else {
                                panic!("West (temporary): Index out of bounds");
                            }
                        };

                        let temp_pos_w_max_height =
                            cmp::max(temp_west_max_height, matrix[i][k + 1]);

                        max_heights[i - 1][k] = Some(Info {
                            north: 0,
                            east: 0,
                            south: 0,
                            west: temp_pos_w_max_height,
                        });
                    }
                }

                if let Some(ref info) = max_heights[i - 1][j] {
                    info.west
                } else {
                    panic!("West: Index out of bounds");
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
            };

            if north_max_height < matrix[i][j]
                || east_max_height < matrix[i][j]
                || west_max_height < matrix[i][j]
                || south_max_height < matrix[i][j]
            {
                visible_trees += 1;
            }
        }
    }

    let edge_trees = (matrix.len() - 1) * 4;
    println!("{:?}", visible_trees + edge_trees);

    Ok(())
}
