use super::State;

pub fn solve(input: &str) -> eyre::Result<u32> {
    let mut state = State::from_input(input)?;

    for _ in 0..10 {
        state.simulate_round();
    }

    Ok(state.empty_ground_tiles())
}
