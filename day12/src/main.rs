use std::collections::HashMap;
use std::collections::VecDeque;

use common::read_lines;

#[derive(Debug, Clone)]
struct Matrix {
    lines: Vec<String>,
    width: u8,
    height: u8,
}

// Day #12
pub fn main() {
    let lines: Vec<String> = read_lines("day12/input.txt");
    let width = lines[0].len() as u8;
    let height = lines.len() as u8;

    let mut matrix = Matrix {
        lines,
        width,
        height,
    };

    let mut visited: Vec<(u8, u8)> = Vec::new();
    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();
    let mut came_from: HashMap<(u8, u8), (u8, u8)> = HashMap::new();

    let starting_position: (u8, u8) = find_starting_position(&matrix.lines);
    let mut position: (u8, u8) = starting_position;

    queue.push_back(position);
    visited.push(position);

    while !queue.is_empty() {
        position = queue.pop_front().unwrap();

        if get_char(position, &matrix.lines) == 'a' {
            println!("Found the target at {:?}", position);
            break;
        }

        let neighbors = get_visitable_neighbors(position, &matrix);

        for neighbor in &neighbors {
            if !visited.contains(neighbor) {
                visited.push(*neighbor);
                queue.push_back(*neighbor);
                came_from.insert(*neighbor, position);
            }
        }
    }

    let mut moves: u16 = 0;

    // Reconstruct path
    while position != starting_position {
        matrix.lines[position.1 as usize]
            .replace_range(position.0 as usize..position.0 as usize + 1, "#");

        position = came_from[&position];

        moves += 1;
    }

    for line in matrix.lines {
        println!("{}", line);
    }

    println!("Moves {:?}", moves);
}

fn get_visitable_neighbors(position: (u8, u8), matrix: &Matrix) -> Vec<(u8, u8)> {
    let (x, y) = position;
    let char = get_char(position, &matrix.lines);
    let value: u8 = if char == 'S' {
        'a' as u8
    } else if char == 'E' {
        'z' as u8
    } else {
        char as u8
    };

    let mut neighbors = Vec::new();

    // Check up
    if y > 0 {
        if check_neighbor((x, y - 1), value, &matrix.lines) {
            neighbors.push((x, y - 1));
        }
    }

    // Check right
    if x < matrix.width - 1 {
        if check_neighbor((x + 1, y), value, &matrix.lines) {
            neighbors.push((x + 1, y));
        }
    }

    // Check down
    if y < matrix.height - 1 {
        if check_neighbor((x, y + 1), value, &matrix.lines) {
            neighbors.push((x, y + 1));
        }
    }
    // Check left
    if x > 0 {
        if check_neighbor((x - 1, y), value, &matrix.lines) {
            neighbors.push((x - 1, y));
        }
    }

    neighbors
}

fn check_neighbor(position: (u8, u8), value: u8, lines: &Vec<String>) -> bool {
    let neighbor_char = get_char(position, lines);
    let neighbor_value: u8 = if neighbor_char == 'S' {
        'a' as u8
    } else if neighbor_char == 'E' {
        'z' as u8
    } else {
        neighbor_char as u8
    };

    (value >= neighbor_value && value - neighbor_value <= 1) || value < neighbor_value
}

fn get_char(position: (u8, u8), lines: &Vec<String>) -> char {
    lines[position.1 as usize]
        .chars()
        .nth(position.0 as usize)
        .unwrap()
}

fn find_starting_position(lines: &Vec<String>) -> (u8, u8) {
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'E' {
                return (x as u8, y as u8);
            }
        }
    }

    panic!("No starting position found in the input.");
}
