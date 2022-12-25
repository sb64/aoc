use super::{int_to_snafu, str_to_snafu};

pub fn solve(input: &str) -> String {
    let num = input.lines().map(|line| str_to_snafu(line.trim())).sum();
    int_to_snafu(num)
}
