use std::collections::HashSet;

extern crate common;
use common::{first_and_last, read_lines};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "R" => Some(Direction::Right),
            "L" => Some(Direction::Left),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn shift(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }
}

#[derive(Debug)]
struct Tail {
    position: Point,
    visited: HashSet<Point>,
}

impl Tail {
    fn new() -> Self {
        let position = Point::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(position);

        Self { position, visited }
    }

    fn follow(&mut self, other: &Point) {
        if self.position.is_adjacent(other) {
            return;
        }

        if self.position.y == other.y {
            self.position.x += if self.position.x < other.x { 1 } else { -1 };
        } else if self.position.x == other.x {
            self.position.y += if self.position.y < other.y { 1 } else { -1 };
        } else {
            self.position.x += if self.position.x < other.x { 1 } else { -1 };
            self.position.y += if self.position.y < other.y { 1 } else { -1 };
        }

        self.visited.insert(self.position);
    }
}

// Day #9
pub fn main() {
    let lines: Vec<String> = read_lines("day09/input.txt");

    let mut head: Point = Point::new(0, 0);
    let mut tail: Tail = Tail::new();

    lines.iter().for_each(|line| {
        let (d, s) = first_and_last!(line);
        let direction = Direction::from_str(d).expect("Failed to parse direction.");
        let steps = s.parse::<i16>().expect("Failed to parse steps as i16.");

        for _ in 0..steps {
            head.shift(&direction);
            tail.follow(&head);
        }
    });

    println!("Visited: {:?}", tail.visited.len());
}
