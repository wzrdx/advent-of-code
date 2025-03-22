use std::fs::read_to_string;

#[macro_export]
macro_rules! first_and_last {
    ($input:expr) => {
        $crate::first_and_last!($input, " ")
    };
    ($input:expr, $delimiter:expr) => {{
        let mut parts = $input.split($delimiter);
        let first = parts.next().unwrap_or_default();
        let last = parts.last().unwrap_or_default();
        (first, last)
    }};
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
