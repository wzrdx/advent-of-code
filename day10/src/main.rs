extern crate common;

use common::{first_and_last, read_lines};

// Day #10
pub fn main() {
    let lines: Vec<String> = read_lines("day10/input.txt");

    // let mut x: u16 = 1;
    let mut cycle: u8 = 1;
    let mut index: u16 = 0;

    let array: Vec<i16> = lines
        .iter()
        .map(|line| {
            let (instr, arg) = first_and_last!(line);

            if instr == "addx" {
                let value = arg.parse::<i16>().expect("Failed to parse number.");
                value
            } else {
                0
            }
        })
        .collect();

    let mut queue: Option<i16> = None;

    while index < array.len() as u16 {
        let value = array[index as usize];

        if queue.is_none() {
            let instr_str = if value == 0 {
                "noop".to_string()
            } else {
                format!("addx {}", value)
            };

            println!("[{:?}] Starting: {:?}", cycle, instr_str);

            index += 1;
        } else {
            println!(
                "[{:?}] Finished: {:?}",
                cycle,
                format!("addx {}", queue.unwrap())
            );
            queue = None;
        }

        if value != 0 {
            queue = Some(value);
        }

        cycle += 1;
    }
}
