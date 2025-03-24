use prettytable::{row, Table};
use std::fs::read_to_string;

const EMPTY_LINE: &str = "\n\n";

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    divisible_by: u8,
    true_monkey: u8,
    false_monkey: u8,
    inspected: u8,
}

// Day #11
pub fn main() {
    let content = read_to_string("day11/input.txt").unwrap();

    let mut monkeys: Vec<Monkey> = parse_input(
        content
            .split(EMPTY_LINE)
            .map(|block| block.lines().collect())
            .collect(),
    );

    for _ in 0..20 {
        monkeys = round(monkeys);
    }

    print_monkeys(&monkeys);

    let result = calculate_monkey_business(&monkeys);
    println!("Monkey Business: {}", result);
}

fn round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();

        monkey.items.iter().for_each(|item| {
            let mut worry_level: usize;
            worry_level = apply_operation(monkey.operation.as_str(), *item);
            worry_level = decrease_worry_level(worry_level);

            let is_divisible = worry_level % monkey.divisible_by as usize == 0;

            let receiver_index = if is_divisible {
                monkey.true_monkey
            } else {
                monkey.false_monkey
            };

            monkeys[i].inspected += 1;
            monkeys[receiver_index as usize].items.push(worry_level);
        });

        monkeys[i].items.clear();
    }

    monkeys
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut array = monkeys.clone();
    array.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let top_two = &array[0..2];
    (top_two[0].inspected as usize) * (top_two[1].inspected as usize)
}

fn apply_operation(s: &str, old: usize) -> usize {
    let items: Vec<&str> = s.split(" ").collect();
    let second_operand = if items[1] == "old" {
        old
    } else {
        items[1]
            .parse::<usize>()
            .expect("Failed to parse second operand.")
    };

    match items[0] {
        "+" => old + second_operand,
        "*" => old * second_operand,
        _ => panic!("Invalid operator."),
    }
}

fn decrease_worry_level(worry_level: usize) -> usize {
    worry_level / 3
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    let mut table = Table::new();
    for (i, monkey) in monkeys.iter().enumerate() {
        table.add_row(row![
            format!("Monkey {}", i),
            format!("{:?}", monkey.items),
            format!("{:?}", monkey.inspected)
        ]);
    }
    table.printstd();
}

fn parse_input(blocks: Vec<Vec<&str>>) -> Vec<Monkey> {
    blocks
        .iter()
        .map(|monkey| Monkey {
            items: monkey[1][18..]
                .split(',')
                .map(|num| {
                    num.trim()
                        .parse::<usize>()
                        .expect("Failed to parse item value.")
                })
                .collect(),
            operation: monkey[2][23..].to_string(),
            divisible_by: monkey[3][21..].parse::<u8>().expect("Failed to parse."),
            true_monkey: monkey[4][29..].parse::<u8>().expect("Failed to parse."),
            false_monkey: monkey[5][30..].parse::<u8>().expect("Failed to parse."),
            inspected: 0,
        })
        .collect()
}
