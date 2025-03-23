extern crate common;

use common::{first_and_last, read_lines};

// Day #10
pub fn main() {
    let lines: Vec<String> = read_lines("day10/input.txt");

    let mut x: i16 = 1;
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
    let mut pixels: Vec<char> = vec!['.'; 240];

    while cycle <= 240 && index < array.len() as u16 {
        // Handle drawing the pixels first, process instructions afterwards.
        let position = cycle - 1;

        if should_draw(x, position as i16) {
            pixels[position as usize] = '#';
        }

        let value = array[index as usize];

        if queue.is_none() {
            // Parse the next instruction.
            // If it's an 'addx' add it to the queue to be processed in the next cycle.
            if value != 0 {
                queue = Some(value);
            }

            index += 1;
        } else {
            // Finish processing the 'addx' instruction from the queue.
            x += queue.unwrap();
            queue = None;
        }

        cycle += 1;
    }

    for row in pixels.chunks(40) {
        println!("{}", row.iter().collect::<String>());
    }
}

fn should_draw(x: i16, position: i16) -> bool {
    let position_mod = position % 40;
    position_mod >= x - 1 && position_mod <= x + 1
}
