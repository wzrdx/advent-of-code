use serde_json::{self, Value};
use std::{cmp::min, fs::read_to_string};

const EMPTY_LINE: &str = "\n\n";

// Day #13
pub fn main() {
    let content = read_to_string("day13/input.txt").unwrap();

    let pairs = content
        .split(EMPTY_LINE)
        .map(|block| {
            let lines: Vec<&str> = block.lines().collect();
            [lines[0], lines[1]]
        })
        .collect::<Vec<[&str; 2]>>();

    let mut pairs_in_order: Vec<usize> = Vec::new();

    for (i, pair) in pairs.iter().enumerate() {
        let result = check_pair(*pair);

        if result > 0 {
            pairs_in_order.push(i + 1);
        }
    }

    println!("Result: {:?}", pairs_in_order.iter().sum::<usize>());
}

fn check_pair(pair: [&str; 2]) -> i16 {
    let left: Value = serde_json::from_str(pair[0]).expect("Failed to parse left value.");
    let right: Value = serde_json::from_str(pair[1]).expect("Failed to parse right value.");

    let left_array: Vec<Value> = match &left {
        Value::Array(arr) => arr.to_vec(),
        _ => panic!("Left value is not an array."),
    };

    let right_array: Vec<Value> = match &right {
        Value::Array(arr) => arr.to_vec(),
        _ => panic!("Right value is not an array."),
    };

    let max_length = min(left_array.len(), right_array.len());

    let mut index = 0;
    let mut result = 0;

    while index < max_length && result == 0 {
        result = compare(&left, &right);
        index += 1;
    }

    result = if result == 0 {
        (right_array.len() as i16) - (left_array.len() as i16)
    } else {
        result
    };

    result
}

fn compare(left: &Value, right: &Value) -> i16 {
    // If both are numbers
    if let (Value::Number(left_n), Value::Number(right_n)) = (&left, &right) {
        let left_value = left_n.as_u64().unwrap();
        let right_value = right_n.as_u64().unwrap();

        return (right_value as i16) - (left_value as i16);
    }
    // If both are arrays
    else if let (Value::Array(left_array), Value::Array(right_array)) = (&left, &right) {
        let max_length = min(left_array.len(), right_array.len());

        let mut index = 0;
        let mut result = 0;

        while index < max_length && result == 0 {
            result = compare(&left_array[index], &right_array[index]);
            index += 1;
        }

        result = if result == 0 {
            (right_array.len() as i16) - (left_array.len() as i16)
        } else {
            result
        };

        result
    } else {
        // If one is an array and one is a number, convert the number to an array and compare
        if left.is_array() && right.is_number() {
            let right_as_array = Value::Array(vec![right.clone()]);
            return compare(left, &right_as_array);
        } else if left.is_number() && right.is_array() {
            let left_as_array = Value::Array(vec![left.clone()]);
            return compare(&left_as_array, right);
        } else {
            panic!("Unexpected mixed types.");
        }
    }
}
