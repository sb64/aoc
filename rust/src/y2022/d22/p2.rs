use super::{parse_instructions, parse_map, Instruction, ProcessedMap, Space};

fn move_by(
    map: &ProcessedMap,
    mut x: usize,
    mut y: usize,
    mut orientation: usize,
    steps: usize,
) -> (usize, usize, usize) {
    for _ in 0..steps {
        let (new_x, new_y, new_orientation) = match (x, y, orientation) {
            // to the left
            (50, 0..=49, 2) => (0, 149 - y, 0),
            (51..=149, 0..=49, 2) => (x - 1, y, 2),
            (50, 50..=99, 2) => (y - 50, 100, 1),
            (51..=99, 50..=99, 2) => (x - 1, y, 2),
            (0, 100..=149, 2) => (50, 149 - y, 0),
            (1..=99, 100..=149, 2) => (x - 1, y, 2),
            (0, 150..=199, 2) => (y - 100, 0, 1),
            (1..=49, 150..=199, 2) => (x - 1, y, 2),

            // to the right
            (50..=148, 0..=49, 0) => (x + 1, y, 0),
            (149, 0..=49, 0) => (99, 149 - y, 2),
            (50..=98, 50..=99, 0) => (x + 1, y, 0),
            (99, 50..=99, 0) => (y + 50, 49, 3),
            (0..=98, 100..=149, 0) => (x + 1, y, 0),
            (99, 100..=149, 0) => (149, 149 - y, 2),
            (0..=48, 150..=199, 0) => (x + 1, y, 0),
            (49, 150..=199, 0) => (y - 100, 149, 3),

            // up
            (0..=49, 100, 3) => (50, x + 50, 0),
            (0..=49, 101..=199, 3) => (x, y - 1, 3),
            (50..=99, 0, 3) => (0, x + 100, 0),
            (50..=99, 1..=149, 3) => (x, y - 1, 3),
            (100..=149, 0, 3) => (x - 100, 199, 3),
            (100..=149, 1..=49, 3) => (x, y - 1, 3),

            // down
            (0..=49, 100..=198, 1) => (x, y + 1, 1),
            (0..=49, 199, 1) => (x + 100, 0, 1),
            (50..=99, 0..=148, 1) => (x, y + 1, 1),
            (50..=99, 149, 1) => (49, x + 100, 2),
            (100..=149, 0..=48, 1) => (x, y + 1, 1),
            (100..=149, 49, 1) => (99, x - 50, 2),

            _ => unreachable!(),
        };

        if let Space::Open = map.map[new_y][new_x] {
            (x, y, orientation) = (new_x, new_y, new_orientation);
        } else {
            break;
        }
    }

    (x, y, orientation)
}

pub fn solve(input: &str) -> eyre::Result<usize> {
    let mut line_iter = input.lines();
    let map = parse_map(&mut line_iter);

    let _ = line_iter.next();

    let mut x = 50;
    let mut y = 0;
    let mut orientation = 0_usize;
    for instruction in parse_instructions(
        line_iter
            .next()
            .ok_or_else(|| eyre::eyre!("can't get instruction line"))?
            .trim(),
    ) {
        match instruction {
            Instruction::Move(steps) => {
                (x, y, orientation) = move_by(&map, x, y, orientation, steps);
            }
            Instruction::TurnLeft => orientation = orientation.checked_sub(1).unwrap_or(3),
            Instruction::TurnRight => orientation = (orientation + 1) % 4,
        }
    }

    Ok(1000 * (y + 1) + 4 * (x + 1) + orientation)
}
