#[macro_export]
macro_rules! first_and_last {
    ($input:expr, $delimiter:expr) => {{
        let mut parts = $input.split($delimiter);
        let first = parts.next().unwrap_or_default();
        let last = parts.last().unwrap_or_default();
        (first, last)
    }};
}
