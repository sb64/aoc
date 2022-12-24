use std::{array, ops::RangeInclusive, str::Lines};

use nom::{branch::alt, bytes::complete::take, character::complete::digit1};

pub mod p1;
pub mod p2;

const MAP_WIDTH: usize = 150;
const MAP_HEIGHT: usize = 200;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Space {
    Open,
    Wall,
    Inaccessible,
}

type Map = [[Space; MAP_WIDTH]; MAP_HEIGHT];

#[derive(Debug, Clone)]
struct ProcessedMap {
    map: Map,
    x_ranges: [RangeInclusive<usize>; MAP_HEIGHT],
    y_ranges: [RangeInclusive<usize>; MAP_WIDTH],
}

fn parse_map(line_iter: &mut Lines) -> Box<ProcessedMap> {
    let map = array::from_fn(|_| {
        let line = line_iter.next().unwrap().as_bytes();
        array::from_fn(|x| match line.get(x) {
            Some(b'.') => Space::Open,
            Some(b'#') => Space::Wall,
            Some(b' ') | None => Space::Inaccessible,
            Some(_) => unreachable!(),
        })
    });

    let mut x_ranges = [None; MAP_HEIGHT];
    let mut y_ranges = [None; MAP_WIDTH];

    for (y, row) in map.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            match space {
                Space::Open | Space::Wall => {
                    x_ranges[y] = match x_ranges[y] {
                        Some((low, _)) => Some((low, x)),
                        None => Some((x, x)),
                    };
                    y_ranges[x] = match y_ranges[x] {
                        Some((low, _)) => Some((low, y)),
                        None => Some((y, y)),
                    };
                }
                Space::Inaccessible => {}
            }
        }
    }

    let x_ranges = {
        let mut iter = x_ranges.into_iter().map(|x_range| {
            let (low, high) = x_range.unwrap();
            low..=high
        });
        std::array::from_fn(|_| iter.next().unwrap())
    };
    let y_ranges = {
        let mut iter = y_ranges.into_iter().map(|y_range| {
            let (low, high) = y_range.unwrap();
            low..=high
        });
        std::array::from_fn(|_| iter.next().unwrap())
    };

    Box::new(ProcessedMap {
        map,
        x_ranges,
        y_ranges,
    })
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn parse_instructions<'a>(input: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    let mut rem = input;
    std::iter::from_fn(move || {
        let (new_rem, instruction) =
            alt::<_, _, nom::error::Error<&str>, _>((digit1, take(1_usize)))(rem).ok()?;
        rem = new_rem;
        Some(match instruction.parse::<usize>() {
            Ok(steps) => Instruction::Move(steps),
            Err(_) => match instruction {
                "L" => Instruction::TurnLeft,
                "R" => Instruction::TurnRight,
                _ => unreachable!(),
            },
        })
    })
}
