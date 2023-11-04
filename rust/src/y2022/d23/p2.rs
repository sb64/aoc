use super::State;

pub fn solve(input: &str) -> eyre::Result<u32> {
    let mut state = State::from_input(input)?;

    let mut round = 1;
    while state.simulate_round() {
        round += 1;
    }

    Ok(round)
}
