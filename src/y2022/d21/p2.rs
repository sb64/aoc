use std::collections::HashMap;

use super::{calculate, parse_monkey, Monkey, Operator};

pub fn solve(input: &str) -> eyre::Result<i64> {
    let monkeys = input
        .lines()
        .filter_map(|line| {
            let (_, (name, monkey)) = match parse_monkey(line.trim()) {
                Ok(res) => res,
                Err(err) => return Some(Err(eyre::eyre!("can't parse monkey: {err}"))),
            };
            (name != "humn").then(|| Ok((name, monkey)))
        })
        .collect::<eyre::Result<HashMap<_, _>>>()?;

    let mut reversed = HashMap::new();
    for (name, monkey) in &monkeys {
        match monkey {
            Monkey::Num(_) => {}
            Monkey::Operation { left, right, .. } => {
                eyre::ensure!(
                    reversed.insert(left.clone(), name.clone()).is_none(),
                    "there is a monkey that two other monkeys depend on"
                );
                eyre::ensure!(
                    reversed.insert(right.clone(), name.clone()).is_none(),
                    "there is a monkey that two other monkeys depend on"
                );
            }
        }
    }

    let mut path = vec![reversed["humn"].clone()];
    while let Some(next) = reversed.get(path.last().unwrap()) {
        path.push(next.clone());
    }

    let mut calculated = HashMap::new();
    let mut iter = path[..(path.len() - 1)].iter().rev().peekable();

    let mut target = match &monkeys["root"] {
        Monkey::Num(_) => eyre::bail!("we got a number monkey for root"),
        Monkey::Operation { left, right, .. } => {
            let problem_monkey = iter
                .peek()
                .ok_or_else(|| eyre::eyre!("can't get problem monkey"))?;
            if *problem_monkey == left {
                calculate(right, &mut calculated, &monkeys)
            } else {
                calculate(left, &mut calculated, &monkeys)
            }
        }
    };

    while let Some(monkey) = iter.next() {
        match &monkeys[monkey] {
            Monkey::Num(_) => eyre::bail!("we got a number monkey"),
            Monkey::Operation { left, op, right } => {
                let problem_monkey = match iter.peek() {
                    Some(problem_monkey) => *problem_monkey,
                    None => "humn",
                };
                if problem_monkey == left {
                    let new_target = calculate(right, &mut calculated, &monkeys);
                    match op {
                        Operator::Add => target = target - new_target,
                        Operator::Subtract => target = target + new_target,
                        Operator::Multipy => target = target / new_target,
                        Operator::Divide => target = target * new_target,
                    }
                } else {
                    let new_target = calculate(left, &mut calculated, &monkeys);
                    match op {
                        Operator::Add => target = target - new_target,
                        Operator::Subtract => target = new_target - target,
                        Operator::Multipy => target = target / new_target,
                        Operator::Divide => target = new_target / target,
                    }
                }
            }
        }
    }

    Ok(target)
}
