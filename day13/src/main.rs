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

        if compare_array(&left_array, &right_array) > 0 {
            pairs_in_order.push(i + 1);
        }
    }

    println!("Result: {:?}", pairs_in_order.iter().sum::<usize>());
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
        return compare_array(left_array, right_array);
    } else {
        // If one is an array and one is a number, convert the number to an array and compare
        let left_array = if left.is_number() {
            Value::Array(vec![left.clone()])
        } else {
            left.clone()
        };

        let right_array = if right.is_number() {
            Value::Array(vec![right.clone()])
        } else {
            right.clone()
        };

        return compare(&left_array, &right_array);
    }
}

fn compare_array(left_array: &Vec<Value>, right_array: &Vec<Value>) -> i16 {
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
}
