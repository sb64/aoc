use to_method::To;

use super::{find_shortest_path_to, parse_states, MAP_HEIGHT, MAP_WIDTH};

pub fn solve(input: &str) -> eyre::Result<u32> {
    let mut lines = input.lines();
    let _ = lines.next();
    let blizzard_states = parse_states(&mut lines)?;
    let steps_to_end =
        find_shortest_path_to(0, u64::MAX, MAP_WIDTH - 1, MAP_HEIGHT, 0, &blizzard_states)?;
    let steps_back_to_start = find_shortest_path_to(
        MAP_WIDTH - 1,
        MAP_HEIGHT,
        0,
        u64::MAX,
        steps_to_end.try_into()?,
        &blizzard_states,
    )?;
    let steps_back_to_end = find_shortest_path_to(
        0,
        u64::MAX,
        MAP_WIDTH - 1,
        MAP_HEIGHT,
        steps_to_end.try_to::<usize>()? + steps_back_to_start.try_to::<usize>()?,
        &blizzard_states,
    )?;
    Ok(steps_to_end + steps_back_to_start + steps_back_to_end)
}
