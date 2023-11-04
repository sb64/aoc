use std::{collections::HashMap, hint::unreachable_unchecked};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, one_of},
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub mod p1;
pub mod p2;

#[derive(Debug, Clone)]
enum Monkey {
    Num(i64),
    Operation {
        left: String,
        op: Operator,
        right: String,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multipy,
    Divide,
}

fn parse_monkey(input: &str) -> IResult<&str, (String, Monkey)> {
    let (rem, name) = terminated(take(4_usize), tag(": "))(input)?;
    let name = name.to_string();

    let (rem, monkey) = alt((digit1, take(11_usize)))(rem)?;
    let monkey = match monkey.parse() {
        Ok(num) => Monkey::Num(num),
        Err(_) => {
            let (_, (left, op, right)) = tuple((
                take(4_usize),
                delimited(tag(" "), one_of("+-*/"), tag(" ")),
                take(4_usize),
            ))(monkey)?;

            let left = left.to_string();
            let op = match op {
                '+' => Operator::Add,
                '-' => Operator::Subtract,
                '*' => Operator::Multipy,
                '/' => Operator::Divide,
                _ => unsafe { unreachable_unchecked() },
            };
            let right = right.to_string();

            Monkey::Operation { left, op, right }
        }
    };

    Ok((rem, (name, monkey)))
}

fn calculate(
    monkey: impl AsRef<str>,
    calculated: &mut HashMap<String, i64>,
    monkeys: &HashMap<String, Monkey>,
) -> i64 {
    let mut stack = vec![monkey.as_ref().to_string()];
    while let Some(name) = stack.pop() {
        if calculated.contains_key(&name) {
            continue;
        }

        match &monkeys[&name] {
            Monkey::Num(num) => {
                calculated.insert(name, *num);
            }
            Monkey::Operation { left, op, right } => {
                match (calculated.get(left), calculated.get(right)) {
                    (None, None) => {
                        stack.push(name);
                        stack.push(left.clone());
                        stack.push(right.clone());
                    }
                    (None, Some(_)) => {
                        stack.push(name);
                        stack.push(left.clone());
                    }
                    (Some(_), None) => {
                        stack.push(name);
                        stack.push(right.clone());
                    }
                    (Some(left), Some(right)) => {
                        let result = match op {
                            Operator::Add => left + right,
                            Operator::Subtract => left - right,
                            Operator::Multipy => left * right,
                            Operator::Divide => left / right,
                        };
                        calculated.insert(name, result);
                    }
                }
            }
        }
    }

    calculated[monkey.as_ref()]
}
