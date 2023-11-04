use super::{parse_scanners, solve_scanners};

pub fn solve(input: &str) -> eyre::Result<usize> {
    let scanners = parse_scanners(input)?;
    let (_, beacons) = solve_scanners(scanners)?;
    println!("{:?}", beacons);
    Ok(beacons.len())
}
