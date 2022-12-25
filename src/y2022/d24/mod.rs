use std::{
    collections::{HashSet, VecDeque},
    mem::MaybeUninit,
    str::Lines,
};

use to_method::To;

pub mod p1;
pub mod p2;

const MAP_WIDTH: u64 = 150;
const MAP_HEIGHT: u64 = 20;
const NUM_BLIZZARD_STATES: usize = 300;

type BlizzardState = [Row; MAP_HEIGHT as usize];
type BlizzardStates = [BlizzardState; NUM_BLIZZARD_STATES];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
enum BlizzardDir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Row([u64; 3]);

impl Row {
    fn from_blizzard_state(y: u64, state: &HashSet<(u64, u64, BlizzardDir)>) -> eyre::Result<Self> {
        let mut row = [0; 3];

        for x in 0..MAP_WIDTH {
            if state.contains(&(x, y, BlizzardDir::Left))
                || state.contains(&(x, y, BlizzardDir::Right))
                || state.contains(&(x, y, BlizzardDir::Up))
                || state.contains(&(x, y, BlizzardDir::Down))
            {
                let index = x / 64;
                let to_shift = 63 - (x % 64);
                let mask = 1 << to_shift;
                row[index.try_to::<usize>()?] |= mask;
            }
        }

        Ok(Self(row))
    }

    fn blizzard_at(&self, x: u64) -> eyre::Result<bool> {
        let index = (x / 64).try_to::<usize>()?;
        let to_shift = 63 - (x % 64);
        let mask = 1 << to_shift;
        Ok(self.0[index] & mask != 0)
    }
}

fn parse_states(lines: &mut Lines) -> eyre::Result<Box<BlizzardStates>> {
    // SAFETY: it's safe to assume the array is initialized since it contains
    // `MaybeUninit`s
    let mut map_states = Box::new(
        [unsafe {
            MaybeUninit::new([MaybeUninit::<Row>::uninit(); MAP_HEIGHT as usize]).assume_init()
        }; NUM_BLIZZARD_STATES],
    );

    let mut cur_state = lines
        .enumerate()
        .take(MAP_HEIGHT.try_to::<usize>()?)
        .map(|(y, line)| {
            let line = line.trim().as_bytes();
            line[1..(line.len() - 1)]
                .iter()
                .enumerate()
                .filter_map(move |(x, &spot)| {
                    let direction = match spot {
                        b'.' => return None,
                        b'<' => BlizzardDir::Left,
                        b'>' => BlizzardDir::Right,
                        b'^' => BlizzardDir::Up,
                        b'v' => BlizzardDir::Down,
                        _ => unreachable!(),
                    };
                    let x = match x.try_to::<u64>() {
                        Ok(x) => x,
                        Err(err) => return Some(Err(err)),
                    };
                    let y = match y.try_to::<u64>() {
                        Ok(y) => y,
                        Err(err) => return Some(Err(err)),
                    };
                    Some(Ok((x, y, direction)))
                })
        })
        .flatten()
        .collect::<Result<HashSet<(u64, u64, BlizzardDir)>, _>>()?;

    for uninit_map in map_states.iter_mut() {
        write_map(uninit_map, &cur_state)?;
        simulate_state_step(&mut cur_state);
    }

    let raw = Box::into_raw(map_states);
    // SAFETY: we initialized everything
    Ok(unsafe { Box::from_raw(raw.cast()) })
}

fn write_map(
    uninit_map: &mut [MaybeUninit<Row>; MAP_HEIGHT as usize],
    state: &HashSet<(u64, u64, BlizzardDir)>,
) -> eyre::Result<()> {
    for y in 0..MAP_HEIGHT {
        uninit_map[y.try_to::<usize>()?].write(Row::from_blizzard_state(y, state)?);
    }

    Ok(())
}

fn simulate_state_step(state: &mut HashSet<(u64, u64, BlizzardDir)>) {
    let new_coords = state
        .drain()
        .map(|(x, y, direction)| match direction {
            BlizzardDir::Left => (x.checked_sub(1).unwrap_or(MAP_WIDTH - 1), y, direction),
            BlizzardDir::Right => ((x + 1) % MAP_WIDTH, y, direction),
            BlizzardDir::Up => (x, y.checked_sub(1).unwrap_or(MAP_HEIGHT - 1), direction),
            BlizzardDir::Down => (x, (y + 1) % MAP_HEIGHT, direction),
        })
        .collect::<Vec<_>>();
    for new_coord in new_coords {
        state.insert(new_coord);
    }
}

fn try_move_left(x: u64, y: u64) -> Option<u64> {
    if y == u64::MAX || y == MAP_HEIGHT {
        None
    } else {
        (x > 0).then(|| x - 1)
    }
}

fn try_move_right(x: u64, y: u64) -> Option<u64> {
    if y == u64::MAX || y == MAP_HEIGHT {
        None
    } else {
        (x < MAP_WIDTH - 1).then(|| x + 1)
    }
}

fn try_move_up(x: u64, y: u64) -> Option<u64> {
    if x == 0 && y == 0 {
        Some(u64::MAX)
    } else if y == u64::MAX {
        None
    } else {
        (y > 0).then(|| y - 1)
    }
}

fn try_move_down(x: u64, y: u64) -> Option<u64> {
    if x == 0 && y == u64::MAX {
        Some(0)
    } else if x == MAP_WIDTH - 1 && y == MAP_HEIGHT - 1 {
        Some(MAP_HEIGHT)
    } else {
        (y < MAP_HEIGHT - 1).then(|| y + 1)
    }
}

fn will_be_safe(
    x: u64,
    y: u64,
    new_blizzard_state_index: usize,
    blizzard_states: &BlizzardStates,
) -> eyre::Result<bool> {
    if x == 0 && y == u64::MAX {
        Ok(true)
    } else if x == MAP_WIDTH - 1 && y == MAP_HEIGHT {
        Ok(true)
    } else {
        Ok(!blizzard_states[new_blizzard_state_index][y.try_to::<usize>()?].blizzard_at(x)?)
    }
}

fn find_shortest_path_to(
    start_x: u64,
    start_y: u64,
    end_x: u64,
    end_y: u64,
    start_blizzard_state_index: usize,
    blizzard_states: &BlizzardStates,
) -> eyre::Result<u32> {
    let mut queue = VecDeque::from([(start_x, start_y, start_blizzard_state_index, 0)]);
    let mut visited = HashSet::new();
    while let Some((x, y, blizzard_state_index, time_spent)) = queue.pop_front() {
        if x == end_x && y == end_y {
            return Ok(time_spent);
        }
        if !visited.contains(&(x, y, blizzard_state_index)) {
            visited.insert((x, y, blizzard_state_index));
            let new_blizzard_state_index = (blizzard_state_index + 1) % NUM_BLIZZARD_STATES;
            if let Some(new_x) = try_move_left(x, y) {
                if !visited.contains(&(new_x, y, new_blizzard_state_index))
                    && will_be_safe(new_x, y, new_blizzard_state_index, blizzard_states)?
                {
                    queue.push_back((new_x, y, new_blizzard_state_index, time_spent + 1));
                }
            }
            if let Some(new_x) = try_move_right(x, y) {
                if !visited.contains(&(new_x, y, new_blizzard_state_index))
                    && will_be_safe(new_x, y, new_blizzard_state_index, blizzard_states)?
                {
                    queue.push_back((new_x, y, new_blizzard_state_index, time_spent + 1));
                }
            }
            if let Some(new_y) = try_move_up(x, y) {
                if !visited.contains(&(x, new_y, new_blizzard_state_index))
                    && will_be_safe(x, new_y, new_blizzard_state_index, blizzard_states)?
                {
                    queue.push_back((x, new_y, new_blizzard_state_index, time_spent + 1));
                }
            }
            if let Some(new_y) = try_move_down(x, y) {
                if !visited.contains(&(x, new_y, new_blizzard_state_index))
                    && will_be_safe(x, new_y, new_blizzard_state_index, blizzard_states)?
                {
                    queue.push_back((x, new_y, new_blizzard_state_index, time_spent + 1));
                }
            }
            if !visited.contains(&(x, y, new_blizzard_state_index))
                && will_be_safe(x, y, new_blizzard_state_index, blizzard_states)?
            {
                queue.push_back((x, y, new_blizzard_state_index, time_spent + 1));
            }
        }
    }

    Err(eyre::eyre!("didn't find a shortest path"))
}
