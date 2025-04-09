use common::read_lines;
use regex::Regex;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Debug, Clone)]
struct Sensor {
    coverage: i32,
    coords: Point,
}

const MAX: i32 = 4_000_000;
const MULTIPLIER: u64 = 4_000_000;

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

    while index <= MAX {
        if let Some(x_position) = get_possible_position(index, &sensors) {
            println!("X: {}, Y: {}", x_position, index);
            println!(
                "Tuning Frequency: {}",
                (x_position as u64) * MULTIPLIER + (index as u64)
            );
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
    let mut ranges = sensors
        .iter()
        .filter(|s| s.coords.y + s.coverage >= row && s.coords.y - s.coverage <= row)
        .map(|s| {
            let diff = (s.coords.y - row).abs();
            let size = s.coverage - diff;

            let start = (s.coords.x - size).max(0);
            let end = (s.coords.x + size).min(MAX);

            Range { start, end }
        })
        .collect::<Vec<Range>>();

    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));

    let mut coverage = Range { start: 0, end: 0 };

    for range in &ranges {
        if range.start <= coverage.end && range.end > coverage.end {
            coverage.end = range.end;
        }
    }

    if coverage.end != MAX {
        return Some(coverage.end + 1);
    }

    None
}
