use super::{parse_instructions, parse_map, Instruction, ProcessedMap, Space};

fn move_left_by(map: &ProcessedMap, mut x: usize, y: usize, steps: usize) -> (usize, usize) {
    for _ in 0..steps {
        let new_x = x.checked_sub(1);
        let x_range = &map.x_ranges[y];
        let new_x = match new_x {
            Some(new_x) => {
                if x_range.contains(&new_x) {
                    new_x
                } else {
                    *x_range.end()
                }
            }
            None => *x_range.end(),
        };

        if let Space::Open = map.map[y][new_x] {
            x = new_x;
        } else {
            break;
        }
    }

    (x, y)
}

fn move_right_by(map: &ProcessedMap, mut x: usize, y: usize, steps: usize) -> (usize, usize) {
    for _ in 0..steps {
        let mut new_x = x + 1;
        let x_range = &map.x_ranges[y];
        if !x_range.contains(&new_x) {
            new_x = *x_range.start()
        }

        if let Space::Open = map.map[y][new_x] {
            x = new_x;
        } else {
            break;
        }
    }

    (x, y)
}

fn move_up_by(map: &ProcessedMap, x: usize, mut y: usize, steps: usize) -> (usize, usize) {
    for _ in 0..steps {
        let new_y = y.checked_sub(1);
        let y_range = &map.y_ranges[x];
        let new_y = match new_y {
            Some(new_y) => {
                if y_range.contains(&new_y) {
                    new_y
                } else {
                    *y_range.end()
                }
            }
            None => *y_range.end(),
        };

        if let Space::Open = map.map[new_y][x] {
            y = new_y;
        } else {
            break;
        }
    }

    (x, y)
}

fn move_down_by(map: &ProcessedMap, x: usize, mut y: usize, steps: usize) -> (usize, usize) {
    for _ in 0..steps {
        let mut new_y = y + 1;
        let y_range = &map.y_ranges[x];
        if !y_range.contains(&new_y) {
            new_y = *y_range.start()
        }

        if let Space::Open = map.map[new_y][x] {
            y = new_y;
        } else {
            break;
        }
    }

    (x, y)
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
                (x, y) = match orientation {
                    0 => move_right_by(&map, x, y, steps),
                    1 => move_down_by(&map, x, y, steps),
                    2 => move_left_by(&map, x, y, steps),
                    3 => move_up_by(&map, x, y, steps),
                    _ => unreachable!(),
                }
            }
            Instruction::TurnLeft => orientation = orientation.checked_sub(1).unwrap_or(3),
            Instruction::TurnRight => orientation = (orientation + 1) % 4,
        }
    }

    Ok(1000 * (y + 1) + 4 * (x + 1) + orientation)
}
