use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    sequence::{preceded, tuple},
    IResult,
};

pub mod p1;
pub mod p2;

fn parse_cube(input: &str) -> IResult<&str, (i32, i32, i32)> {
    use nom::character::complete::i32;

    tuple((i32, preceded(tag(","), i32), preceded(tag(","), i32)))(input)
}

fn get_cubes(input: &str) -> eyre::Result<HashSet<(i32, i32, i32)>> {
    input
        .lines()
        .map(|line| {
            let (_, coords) =
                parse_cube(line.trim()).map_err(|err| eyre::eyre!("can't parse coords: {err}"))?;
            Ok(coords)
        })
        .collect()
}
