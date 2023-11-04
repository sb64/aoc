use nom::{
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult,
};

pub mod p1;
pub mod p2;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32),
    geode_robot_cost: (u32, u32),
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    use nom::character::complete::u32;
    let (rem, id) = delimited(tag("Blueprint "), u32, tag(": "))(input)?;
    let (rem, ore_robot_cost) = delimited(tag("Each ore robot costs "), u32, tag(" ore. "))(rem)?;
    let (rem, clay_robot_cost) = delimited(tag("Each clay robot costs "), u32, tag(" ore. "))(rem)?;
    let (rem, obsidian_robot_cost) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(u32, tag(" ore and "), u32),
        tag(" clay. "),
    )(rem)?;
    let (rem, geode_robot_cost) = delimited(
        tag("Each geode robot costs "),
        separated_pair(u32, tag(" ore and "), u32),
        tag(" obsidian."),
    )(rem)?;
    Ok((
        rem,
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        },
    ))
}
