use prettytable::{row, Table};
use std::collections::HashMap;
use std::fs::read_to_string;

const EMPTY_LINE: &str = "\n\n";

#[derive(Debug, Clone)]
struct Base {
    operation: char,
    second_operand: Option<u16>,
    divisor: u8,
    true_monkey: u8,
    false_monkey: u8,
    inspected: usize,
}

#[derive(Debug, Clone)]
struct MonkeyWithItems {
    base: Base,
    items: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Monkey {
    base: Base,
    mods_vec: Vec<HashMap<u8, u16>>,
}

// Day #11
pub fn main() {
    let content = read_to_string("day11/input.txt").unwrap();

    let monkeys_w_items: Vec<MonkeyWithItems> = parse_input(
        content
            .split(EMPTY_LINE)
            .map(|block| block.lines().collect::<Vec<&str>>())
            .collect(),
    );

    let divisors = monkeys_w_items
        .iter()
        .map(|monkey| monkey.base.divisor)
        .collect::<Vec<u8>>();

    let mut monkeys: Vec<Monkey> = monkeys_w_items
        .iter()
        .map(|monkey| Monkey {
            base: monkey.base.clone(),
            mods_vec: monkey
                .items
                .iter()
                .map(|item| calculate_divisibility(*item, &divisors))
                .collect(),
        })
        .collect();

    for _ in 0..10_000 {
        monkeys = round(monkeys);
    }

    print_monkeys(&monkeys);

    let result = calculate_monkey_business(monkeys);
    println!("Monkey Business: {:?}", result);
}

fn round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();

        monkey.mods_vec.iter().for_each(|mods| {
            let updated_mods = apply_operation(
                monkey.base.operation,
                monkey.base.second_operand,
                mods.clone(),
            );

            let receiver_index = if updated_mods[&monkey.base.divisor] == 0 {
                monkey.base.true_monkey
            } else {
                monkey.base.false_monkey
            };

            monkeys[i].base.inspected += 1;
            monkeys[receiver_index as usize].mods_vec.push(updated_mods);
        });

        monkeys[i].mods_vec.clear();
    }

    monkeys
}

fn calculate_divisibility(item: u8, divisors: &Vec<u8>) -> HashMap<u8, u16> {
    let mut hash_map: HashMap<u8, u16> = HashMap::new();

    divisors.iter().for_each(|divisor| {
        hash_map.insert(*divisor, (item % divisor) as u16);
    });

    hash_map
}

fn calculate_monkey_business(mut monkeys: Vec<Monkey>) -> u64 {
    monkeys.sort_by(|a, b| b.base.inspected.cmp(&a.base.inspected));
    let top_two = &monkeys[0..2];
    (top_two[0].base.inspected as u64) * (top_two[1].base.inspected as u64)
}

fn apply_operation(
    operation: char,
    second_operand: Option<u16>,
    mut mods: HashMap<u8, u16>,
) -> HashMap<u8, u16> {
    for (divisor, value) in mods.iter_mut() {
        let result: u16 = match operation {
            '+' => *value + second_operand.unwrap(),
            '*' => {
                *value
                    * if second_operand.is_some() {
                        second_operand.unwrap()
                    } else {
                        *value
                    }
            }
            _ => panic!("Invalid operator."),
        };

        *value = result % (*divisor as u16);
    }

    mods
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    let mut table = Table::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        table.add_row(row![
            format!("Monkey {}", i),
            format!("{:?}", monkey.base.inspected)
        ]);
    }
    table.printstd();
}

fn parse_operation(operation: &str) -> (char, Option<u16>) {
    let items: Vec<&str> = operation.split(" ").collect();
    let second_operand = if items[1] == "old" {
        None
    } else {
        Some(
            items[1]
                .parse::<u16>()
                .expect("Failed to parse second operand."),
        )
    };

    (
        items[0].chars().next().expect("Invalid operation."),
        second_operand,
    )
}

fn parse_input(blocks: Vec<Vec<&str>>) -> Vec<MonkeyWithItems> {
    blocks
        .iter()
        .map(|monkey| {
            let (operation, second_operand) = parse_operation(&monkey[2][23..]);

            MonkeyWithItems {
                items: monkey[1][18..]
                    .split(',')
                    .map(|num| {
                        num.trim()
                            .parse::<u8>()
                            .expect("Failed to parse item value.")
                    })
                    .collect(),
                base: Base {
                    operation,
                    second_operand,
                    divisor: monkey[3][21..].parse::<u8>().expect("Failed to parse."),
                    true_monkey: monkey[4][29..].parse::<u8>().expect("Failed to parse."),
                    false_monkey: monkey[5][30..].parse::<u8>().expect("Failed to parse."),
                    inspected: 0,
                },
            }
        })
        .collect()
}
