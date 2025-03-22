extern crate common;
use common::{first_and_last, read_lines};

// Day #10
pub fn main() {
    let lines: Vec<String> = read_lines("day10/input.txt");

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut stack = vec![20, 60, 100, 140, 180, 220];
    let mut values: Vec<i32> = Vec::new();

    lines.iter().for_each(|line| {
        let (instr, arg) = first_and_last!(line);

        if stack.is_empty() {
            return;
        }

        if instr == "addx" {
            let value = arg.parse::<i32>().expect("Failed to parse number.");

            if cycle + 2 >= stack[0] {
                println!(
                    "[{:?}] ({:?}) Value for {:?} is {:?}",
                    cycle,
                    x,
                    stack[0],
                    x * stack[0]
                );

                values.push(x * stack[0]);
                stack.remove(0);
            }

            cycle += 2;
            x += value;
            // println!("[{:?}] ({:?}) {:?}, {:?}", cycle, x, instr, value);
        } else {
            if cycle + 1 >= stack[0] {
                println!(
                    "[{:?}] ({:?}) Value for {:?} is {:?}",
                    cycle,
                    x,
                    stack[0],
                    x * stack[0]
                );

                values.push(x * stack[0]);
                stack.remove(0);
            }

            cycle += 1;
            // println!("[{:?}] ({:?}) {:?}", cycle, x, instr);
        }
    });

    println!("Sum: {}", values.iter().sum::<i32>());
}
