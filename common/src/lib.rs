#[macro_export]
macro_rules! first_and_last {
    ($input:expr) => {{
        let mut parts = $input.split_whitespace();
        let first = parts.next().unwrap_or_default();
        let last = parts.last().unwrap_or_default();
        (first, last)
    }};
}
