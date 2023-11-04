use std::collections::{HashMap, HashSet};

use eyre::Context;
use to_method::To;

pub mod p1;
pub mod p2;

enum Direction {
    North,
    South,
    West,
    East,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

#[derive(Debug, Clone)]
struct State {
    map: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    directions_index: usize,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(i32, i32),
    DoNothing,
}

impl State {
    fn from_input(input: &str) -> eyre::Result<Self> {
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .filter_map(move |(x, &spot)| {
                        let x = match x.try_to::<i32>().wrap_err("can't convert x to an i32") {
                            Ok(x) => x,
                            Err(err) => return Some(Err(err)),
                        };
                        let y = match y.try_to::<i32>().wrap_err("can't convert y to an i32") {
                            Ok(y) => y,
                            Err(err) => return Some(Err(err)),
                        };
                        (spot == b'#').then_some(Ok((x, y)))
                    })
            })
            .flatten()
            .collect::<eyre::Result<HashSet<(i32, i32)>>>()?;

        let (min_x, max_x, min_y, max_y) = map.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );

        Ok(Self {
            map,
            min_x,
            max_x,
            min_y,
            max_y,
            directions_index: 0,
        })
    }

    fn elf_proposals(&mut self) -> (Vec<(i32, i32, Action)>, HashMap<(i32, i32), u32>, bool) {
        let mut proposed_actions = Vec::new();
        let mut proposed_destinations = HashMap::new();
        let mut elf_moved = false;
        'outer: for &(x, y) in &self.map {
            let no_elves_around = itertools::iproduct!(-1..=1, -1..=1).all(|(dx, dy)| {
                if dx == 0 && dy == 0 {
                    true
                } else {
                    !self.map.contains(&(x + dx, y + dy))
                }
            });
            if no_elves_around {
                proposed_actions.push((x, y, Action::DoNothing));
                continue 'outer;
            }
            elf_moved = true;

            for directions_index in (self.directions_index..4).chain(0..self.directions_index) {
                match DIRECTIONS[directions_index] {
                    Direction::North => {
                        let can_move_north =
                            (-1..=1).all(|dx| !self.map.contains(&(x + dx, y - 1)));
                        if can_move_north {
                            proposed_actions.push((x, y, Action::Move(x, y - 1)));
                            *proposed_destinations.entry((x, y - 1)).or_insert(0) += 1;
                            continue 'outer;
                        }
                    }
                    Direction::South => {
                        let can_move_south =
                            (-1..=1).all(|dx| !self.map.contains(&(x + dx, y + 1)));
                        if can_move_south {
                            proposed_actions.push((x, y, Action::Move(x, y + 1)));
                            *proposed_destinations.entry((x, y + 1)).or_insert(0) += 1;
                            continue 'outer;
                        }
                    }
                    Direction::West => {
                        let can_move_west = (-1..=1).all(|dy| !self.map.contains(&(x - 1, y + dy)));
                        if can_move_west {
                            proposed_actions.push((x, y, Action::Move(x - 1, y)));
                            *proposed_destinations.entry((x - 1, y)).or_insert(0) += 1;
                            continue 'outer;
                        }
                    }
                    Direction::East => {
                        let can_move_east = (-1..=1).all(|dy| !self.map.contains(&(x + 1, y + dy)));
                        if can_move_east {
                            proposed_actions.push((x, y, Action::Move(x + 1, y)));
                            *proposed_destinations.entry((x + 1, y)).or_insert(0) += 1;
                            continue 'outer;
                        }
                    }
                }
            }

            proposed_actions.push((x, y, Action::DoNothing));
        }

        (proposed_actions, proposed_destinations, elf_moved)
    }

    /// Returns whether any elves moved this round
    fn simulate_round(&mut self) -> bool {
        let (proposed_actions, proposed_destinations, elf_moved) = self.elf_proposals();
        if !elf_moved {
            return false;
        }

        let mut new_min_x = i32::MAX;
        let mut new_max_x = i32::MIN;
        let mut new_min_y = i32::MAX;
        let mut new_max_y = i32::MIN;
        for (x, y, action) in proposed_actions {
            let (new_x, new_y) = match action {
                Action::Move(new_x, new_y) => {
                    if proposed_destinations[&(new_x, new_y)] == 1 {
                        self.map.remove(&(x, y));
                        self.map.insert((new_x, new_y));
                        (new_x, new_y)
                    } else {
                        (x, y)
                    }
                }
                Action::DoNothing => (x, y),
            };
            new_min_x = new_min_x.min(new_x);
            new_max_x = new_max_x.max(new_x);
            new_min_y = new_min_y.min(new_y);
            new_max_y = new_max_y.max(new_y);
        }

        self.min_x = new_min_x;
        self.max_x = new_max_x;
        self.min_y = new_min_y;
        self.max_y = new_max_y;
        self.directions_index = (self.directions_index + 1) % 4;

        true
    }

    fn empty_ground_tiles(&self) -> u32 {
        let mut num_tiles = 0;
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                if !self.map.contains(&(x, y)) {
                    num_tiles += 1;
                }
            }
        }

        num_tiles
    }
}
