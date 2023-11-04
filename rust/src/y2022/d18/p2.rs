use std::collections::{HashSet, VecDeque};

use super::get_cubes;

const STEPS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub fn solve(input: &str) -> eyre::Result<u32> {
    let cubes = get_cubes(input)?;

    let (min_x, max_x, min_y, max_y, min_z, max_z) = cubes.iter().copied().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y, min_z, max_z), (x, y, z)| {
            (
                min_x.min(x),
                max_x.max(x),
                min_y.min(y),
                max_y.max(y),
                min_z.min(z),
                max_z.max(z),
            )
        },
    );

    let mut visited = HashSet::new();
    let x_range = (min_x - 1)..=(max_x + 1);
    let y_range = (min_y - 1)..=(max_y + 1);
    let z_range = (min_z - 1)..=(max_z + 1);
    let mut num_exposed_faces = 0;
    let mut queue = VecDeque::from([(min_x - 1, min_y - 1, min_z - 1)]);
    while let Some((x, y, z)) = queue.pop_front() {
        if !visited.contains(&(x, y, z)) {
            visited.insert((x, y, z));
            for (dx, dy, dz) in STEPS {
                if x_range.contains(&(x + dx))
                    && y_range.contains(&(y + dy))
                    && z_range.contains(&(z + dz))
                {
                    if cubes.contains(&(x + dx, y + dy, z + dz)) {
                        num_exposed_faces += 1;
                    } else if !visited.contains(&(x + dx, y + dy, z + dz)) {
                        queue.push_back((x + dx, y + dy, z + dz));
                    }
                }
            }
        }
    }

    Ok(num_exposed_faces)
}
