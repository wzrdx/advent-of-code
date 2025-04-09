use std::collections::HashSet;

use common::read_lines;
use regex::Regex;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Sensor {
    coverage: i32,
    coords: Point,
}

const MAX: i32 = 4_000_000;
const MULTIPLIER: i32 = 4_000_000;

// Day #15
pub fn main() {
    let lines: Vec<String> = read_lines("day15/input.txt");
    let regex = Regex::new(r"-?\d+").unwrap();

    let sensors: Vec<Sensor> = lines
        .iter()
        .map(|line| {
            let numbers: Vec<i32> = regex
                .find_iter(line)
                .filter_map(|n| n.as_str().parse::<i32>().ok())
                .collect();

            let coords = Point {
                x: numbers[0],
                y: numbers[1],
            };

            let beacon = Point {
                x: numbers[2],
                y: numbers[3],
            };

            Sensor {
                coverage: get_distance(&coords, &beacon),
                coords,
            }
        })
        .collect();

    let mut index: i32 = 0;

    while index <= 0 {
        println!("{}", index);

        if let Some(x_position) = get_possible_position(index, &sensors) {
            println!("X: {}, Y: {}", x_position, index);
            println!("Tuning Frequency: {}", x_position * MULTIPLIER + index);
            break;
        } else {
            index += 1;
        }
    }
}

fn get_distance(a: &Point, b: &Point) -> i32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as i32
}

fn get_possible_position(row: i32, sensors: &Vec<Sensor>) -> Option<i32> {
    let mut set: HashSet<i32> = HashSet::new();

    sensors
        .iter()
        .filter(|s| s.coords.y + s.coverage >= row && s.coords.y - s.coverage <= row)
        .for_each(|s| {
            let diff = (s.coords.y - row).abs();
            let size = s.coverage - diff;

            let start = (s.coords.x - size).max(0);
            let end = (s.coords.x + size).min(MAX);

            println!("[{:?}] {} to {}", s.coords, start, end);

            // set.extend(start..=end);
        });

    // if (set.len() as i32) == MAX {
    //     // Find the missing position
    //     for i in 0..=MAX {
    //         if !set.contains(&i) {
    //             return Some(i);
    //         }
    //     }
    // }

    None
}
