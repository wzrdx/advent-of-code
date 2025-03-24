use prettytable::{row, Table};
use std::fs::read_to_string;

#[derive(Debug)]
struct Monkey {
    items: Vec<u16>,
    operation: String,
    divisible_by: u8,
    true_monkey: u8,
    false_monkey: u8,
}

// Day #11
pub fn main() {
    let content = read_to_string("day11/input.txt").unwrap();

    let blocks: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|block| block.lines().collect())
        .collect();

    println!("{:?}", blocks);

    let monkeys: Vec<Monkey> = blocks
        .iter()
        .map(|monkey| Monkey {
            items: monkey[1][18..]
                .split(',')
                .map(|num| {
                    num.trim()
                        .parse::<u16>()
                        .expect("Failed to parse item value.")
                })
                .collect(),
            operation: monkey[2][23..].to_string(),
            divisible_by: monkey[3][21..].parse::<u8>().expect("Failed to parse."),
            true_monkey: monkey[4][29..].parse::<u8>().expect("Failed to parse."),
            false_monkey: monkey[5][30..].parse::<u8>().expect("Failed to parse."),
        })
        .collect();

    let mut table = Table::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        table.add_row(row![
            format!("Monkey {}", i),
            format!("{:?}", monkey.items),
            format!("Operation: {}", monkey.operation),
            format!("Divisible by: {}", monkey.divisible_by),
            format!("True: {}", monkey.true_monkey),
            format!("False: {}", monkey.false_monkey),
        ]);
    }
    table.printstd();
}

fn apply_operation(s: String, old: u16) -> u16 {
    let items: Vec<&str> = s.split(" ").collect();
    let second_operand = if items[1] == "old" {
        old
    } else {
        items[1]
            .parse::<u16>()
            .expect("Failed to parse second operand.")
    };

    match items[0] {
        "+" => old + second_operand,
        "*" => old * second_operand,
        _ => panic!("Invalid operator."),
    }
}
