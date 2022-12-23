use std::collections::HashMap;

use super::get_cubes;

pub fn solve(input: &str) -> eyre::Result<i32> {
    let cubes = get_cubes(input)?;

    let mut num_exposed_faces = cubes
        .iter()
        .copied()
        .map(|cube| (cube, 6))
        .collect::<HashMap<_, _>>();

    for (x, y, z) in cubes.iter().copied() {
        const STEPS: [(i32, i32, i32); 6] = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ];

        for (dx, dy, dz) in STEPS {
            if cubes.contains(&(x + dx, y + dy, z + dz)) {
                num_exposed_faces
                    .entry((x, y, z))
                    .and_modify(|exposed_faces| *exposed_faces -= 1);
            }
        }
    }

    Ok(num_exposed_faces.values().copied().sum())
}
