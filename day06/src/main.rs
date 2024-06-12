use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

const COUNT: usize = 14;

#[derive(Debug)]
struct Queue {
    max_length: usize,
    array: VecDeque<char>,
}

impl Queue {
    fn new(max_length: usize, load: &[char]) -> Self {
        let mut array = VecDeque::new();
        array.extend(load);

        Self { max_length, array }
    }

    fn push(&mut self, value: char) {
        if self.array.len() == self.max_length {
            self.array.pop_front();
        }

        self.array.push_back(value);
    }

    fn are_all_unique(&self) -> bool {
        let mut seen = HashSet::new();

        for c in &self.array {
            if !seen.insert(c) {
                return false;
            }
        }

        true
    }
}

// Day #6
fn main() {
    let mut str: String = read_to_string("day06/input.txt").unwrap();
    let mut load: Vec<char> = Vec::new();

    for _ in 0..(COUNT - 1) {
        load.push(str.remove(0));
    }

    let mut queue = Queue::new(COUNT, &load);

    let mut chars = str.chars();
    let mut processed = load.len();

    while let Some(c) = chars.next() {
        queue.push(c);
        processed += 1;

        if queue.are_all_unique() {
            println!("{:?}, {:?}", processed, &queue);
            break;
        }
    }

    println!("End");
}
