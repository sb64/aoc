use std::collections::HashMap;

use super::{parse_monkey, calculate};

pub fn solve(input: &str) -> eyre::Result<i64> {
    let monkeys = input
        .lines()
        .map(|line| {
            parse_monkey(line.trim())
                .map(|(_, name_and_operation)| name_and_operation)
                .map_err(|err| eyre::eyre!("can't parse operation: {err}"))
        })
        .collect::<eyre::Result<HashMap<_, _>>>()?;

    let mut calculated = HashMap::new();
    Ok(calculate("root", &mut calculated, &monkeys))
}
