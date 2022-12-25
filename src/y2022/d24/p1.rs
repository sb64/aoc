use super::{find_shortest_path_to, parse_states, MAP_HEIGHT, MAP_WIDTH};

pub fn solve(input: &str) -> eyre::Result<u32> {
    let mut lines = input.lines();
    let _ = lines.next();
    let blizzard_states = parse_states(&mut lines)?;
    find_shortest_path_to(0, u64::MAX, MAP_WIDTH - 1, MAP_HEIGHT, 0, &blizzard_states)
}
