use common::read_lines;
use serde_json::{self, Value};
use std::cmp::min;

// Day #13
pub fn main() {
    let mut lines: Vec<Value> = read_lines("day13/input.txt")
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(&line).expect("Failed to parse JSON"))
        .collect();

    // Add divider packets
    let divider_1_str: String = "[[2]]".to_owned();
    let divider_2_str: String = "[[6]]".to_owned();

    let divider_1_value = serde_json::from_str(divider_1_str.as_str()).unwrap();
    let divider_2_value = serde_json::from_str(divider_2_str.as_str()).unwrap();

    lines.push(divider_1_value);
    lines.push(divider_2_value);

    lines.sort_by(|a, b| {
        let result = compare(a, b);
        if result > 0 {
            std::cmp::Ordering::Less
        } else if result < 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let mut indices = Vec::new();

    for (i, value) in lines.iter().enumerate() {
        let value_str = serde_json::to_string(value).unwrap();

        if value_str == divider_1_str || value_str == divider_2_str {
            indices.push(i + 1);
        }
    }

    println!("Decoder key: {}", indices.iter().product::<usize>());
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
