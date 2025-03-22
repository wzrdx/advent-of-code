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

    fn is_adjacent(&self, other: &Self) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        x_diff <= 1 && y_diff <= 1
    }

    fn shift(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn follow(&mut self, other: &Point) {
        if self.y == other.y {
            self.x += if self.x < other.x { 1 } else { -1 };
        } else if self.x == other.x {
            self.y += if self.y < other.y { 1 } else { -1 };
        } else {
            self.x += if self.x < other.x { 1 } else { -1 };
            self.y += if self.y < other.y { 1 } else { -1 };
        }
    }
}

#[derive(Debug)]
struct Tail {
    points: [Point; 9],
    visited: HashSet<Point>,
}

impl Tail {
    fn new() -> Self {
        let position = Point::new(0, 0);
        let points = [position; 9];

        let mut visited = HashSet::new();
        visited.insert(position);

        Self { points, visited }
    }

    fn follow(&mut self, head: &Point) {
        let mut leader = *head;
        for i in 0..9 {
            if self.points[i].is_adjacent(&leader) {
                // Break early because the tail doesn't move
                return;
            }

            self.points[i].follow(&leader);
            leader = self.points[i];
        }

        self.visited.insert(self.points[8]);
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
