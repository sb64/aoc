use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub mod p1;
pub mod p2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Offset(i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beacon(i64, i64, i64);

impl Beacon {
    fn fingerprint(self, other: Self) -> u64 {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = other;
        x1.abs_diff(x2) + y1.abs_diff(y2) + z1.abs_diff(z2)
    }

    fn rotation_iter() -> impl Iterator<Item = fn(Beacon) -> Beacon> {
        static ITER: [fn(Beacon) -> Beacon; 24] = [
            |Beacon(x, y, z)| Beacon(x, y, z),
            |Beacon(x, y, z)| Beacon(x, z, -y),
            |Beacon(x, y, z)| Beacon(x, -y, -z),
            |Beacon(x, y, z)| Beacon(x, -z, y),
            |Beacon(x, y, z)| Beacon(-x, -y, z),
            |Beacon(x, y, z)| Beacon(-x, z, y),
            |Beacon(x, y, z)| Beacon(-x, y, -z),
            |Beacon(x, y, z)| Beacon(-x, -z, -y),
            |Beacon(x, y, z)| Beacon(y, z, x),
            |Beacon(x, y, z)| Beacon(y, x, -z),
            |Beacon(x, y, z)| Beacon(y, -z, -x),
            |Beacon(x, y, z)| Beacon(y, -x, z),
            |Beacon(x, y, z)| Beacon(-y, -z, x),
            |Beacon(x, y, z)| Beacon(-y, x, z),
            |Beacon(x, y, z)| Beacon(-y, z, -x),
            |Beacon(x, y, z)| Beacon(-y, -x, -z),
            |Beacon(x, y, z)| Beacon(z, x, y),
            |Beacon(x, y, z)| Beacon(z, y, -x),
            |Beacon(x, y, z)| Beacon(z, -x, -y),
            |Beacon(x, y, z)| Beacon(z, -y, x),
            |Beacon(x, y, z)| Beacon(-z, -x, y),
            |Beacon(x, y, z)| Beacon(-z, y, x),
            |Beacon(x, y, z)| Beacon(-z, x, -y),
            |Beacon(x, y, z)| Beacon(-z, -y, -x),
        ];
        ITER.iter().copied()
    }
}

impl Add<Offset> for Beacon {
    type Output = Beacon;

    fn add(self, offset: Offset) -> Self::Output {
        let Beacon(x1, y1, z1) = self;
        let Offset(x2, y2, z2) = offset;
        Beacon(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl AddAssign<Offset> for Beacon {
    fn add_assign(&mut self, offset: Offset) {
        *self = *self + offset
    }
}

impl Sub<Offset> for Beacon {
    type Output = Beacon;

    fn sub(self, offset: Offset) -> Self::Output {
        let Beacon(x1, y1, z1) = self;
        let Offset(x2, y2, z2) = offset;
        Beacon(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl Sub<Beacon> for Beacon {
    type Output = Offset;

    fn sub(self, other: Self) -> Self::Output {
        let Beacon(x1, y1, z1) = self;
        let Beacon(x2, y2, z2) = other;
        Offset(x1 - x2, y1 - y2, z1 - z2)
    }
}

impl SubAssign<Offset> for Beacon {
    fn sub_assign(&mut self, offset: Offset) {
        *self = *self - offset;
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
    fingerprints: HashMap<u64, usize>,
}

impl Scanner {
    fn new(beacons: Vec<Beacon>) -> Self {
        let mut fingerprints = HashMap::new();
        for i in 0..(beacons.len() - 1) {
            for j in (i + 1)..beacons.len() {
                let beacon1 = beacons[i];
                let beacon2 = beacons[j];
                let fingerprint = beacon1.fingerprint(beacon2);
                *fingerprints.entry(fingerprint).or_insert(0) += 1;
            }
        }

        Self {
            beacons,
            fingerprints,
        }
    }

    fn could_overlap(&self, other: &Self) -> bool {
        self.fingerprints
            .iter()
            .filter_map(|(fingerprint, count1)| {
                other
                    .fingerprints
                    .get(fingerprint)
                    .map(|count2| count1.min(count2))
            })
            .sum::<usize>()
            >= 66
    }

    fn offset(&self, other_beacons: impl Iterator<Item = Beacon>) -> Option<Offset> {
        let mut offsets = HashMap::new();
        for other_beacon in other_beacons {
            for &my_beacon in &self.beacons {
                let offset = my_beacon - other_beacon;
                if offset == Offset(68, -1246, -43) {
                    dbg!(&my_beacon);
                    dbg!(&other_beacon);
                }
                *offsets.entry(offset).or_insert(0_usize) += 1;
            }
        }

        offsets
            .into_iter()
            .find_map(|(offset, count)| (count >= 12).then_some(offset))
    }

    fn try_transform_to_find_offset(
        &self,
        potential_solved_scanner: &mut Scanner,
    ) -> Option<Offset> {
        for transform in Beacon::rotation_iter() {
            if let Some(offset) = self.offset(
                potential_solved_scanner
                    .beacons
                    .iter()
                    .copied()
                    .map(transform),
            ) {
                for beacon in &mut potential_solved_scanner.beacons {
                    *beacon = transform(*beacon);
                }
                return Some(offset);
            }
        }

        None
    }
}

fn parse_scanners(input: &str) -> eyre::Result<Vec<Scanner>> {
    let mut scanners = Vec::new();
    let mut cur_beacons = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            scanners.push(Scanner::new(cur_beacons));
            cur_beacons = Vec::new();
            continue;
        }

        if line.starts_with("---") {
            continue;
        }

        use nom::{
            character::complete::{char, i64},
            sequence::{preceded, tuple},
        };
        let (_, (x, y, z)) = tuple::<&str, _, nom::error::Error<_>, _>((
            i64,
            preceded(char(','), i64),
            preceded(char(','), i64),
        ))(line)
        .map_err(|err| eyre::eyre!("parsing error: {err:?}"))?;

        cur_beacons.push(Beacon(x, y, z));
    }

    scanners.rotate_left(1);

    Ok(scanners)
}

fn solve_scanners(mut scanners: Vec<Scanner>) -> eyre::Result<(Vec<Offset>, HashSet<Beacon>)> {
    let (mut solved, mut beacons) = match scanners.pop() {
        Some(first_scanner) => {
            let beacons = first_scanner
                .beacons
                .iter()
                .copied()
                .collect::<HashSet<_>>();
            (vec![first_scanner], beacons)
        }
        None => return Ok((Vec::new(), HashSet::new())),
    };
    let mut offsets = vec![Offset(0, 0, 0)];

    let mut cur_ref_scanner_index = 0;
    let mut cur_ref_scanner;
    while cur_ref_scanner_index < solved.len() && scanners.len() > 0 {
        cur_ref_scanner = &solved[cur_ref_scanner_index];

        let scanners_found = scanners
            .iter_mut()
            .enumerate()
            .filter(|(_, scanner)| cur_ref_scanner.could_overlap(scanner))
            .filter_map(|(i, pot_next_scanner)| {
                cur_ref_scanner
                    .try_transform_to_find_offset(pot_next_scanner)
                    .map(|offset| (i, offset))
            })
            .collect::<Vec<_>>();

        for (i, offset) in scanners_found.into_iter().rev() {
            offsets.push(offset);

            let mut new_scanner = scanners.swap_remove(i);
            for beacon in &mut new_scanner.beacons {
                *beacon += offset;
                beacons.insert(*beacon);
            }
            solved.push(new_scanner);
        }

        cur_ref_scanner_index += 1;
    }

    Ok((offsets, beacons))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scanner0 = Scanner::new(vec![
            Beacon(404, -588, -901),
            Beacon(528, -643, 409),
            Beacon(-838, 591, 734),
            Beacon(390, -675, -793),
            Beacon(-537, -823, -458),
            Beacon(-485, -357, 347),
            Beacon(-345, -311, 381),
            Beacon(-661, -816, -575),
            Beacon(-876, 649, 763),
            Beacon(-618, -824, -621),
            Beacon(553, 345, -567),
            Beacon(474, 580, 667),
            Beacon(-447, -329, 318),
            Beacon(-584, 868, -557),
            Beacon(544, -627, -890),
            Beacon(564, 392, -477),
            Beacon(455, 729, 728),
            Beacon(-892, 524, 684),
            Beacon(-689, 845, -530),
            Beacon(423, -701, 434),
            Beacon(7, -33, -71),
            Beacon(630, 319, -379),
            Beacon(443, 580, 662),
            Beacon(-789, 900, -551),
            Beacon(459, -707, 401),
        ]);

        let mut scanner1 = Scanner::new(vec![
            Beacon(686, 422, 578),
            Beacon(605, 423, 415),
            Beacon(515, 917, -361),
            Beacon(-336, 658, 858),
            Beacon(95, 138, 22),
            Beacon(-476, 619, 847),
            Beacon(-340, -569, -846),
            Beacon(567, -361, 727),
            Beacon(-460, 603, -452),
            Beacon(669, -402, 600),
            Beacon(729, 430, 532),
            Beacon(-500, -761, 534),
            Beacon(-322, 571, 750),
            Beacon(-466, -666, -811),
            Beacon(-429, -592, 574),
            Beacon(-355, 545, -477),
            Beacon(703, -491, -529),
            Beacon(-328, -685, 520),
            Beacon(413, 935, -424),
            Beacon(-391, 539, -444),
            Beacon(586, -435, 557),
            Beacon(-364, -763, -893),
            Beacon(807, -499, -711),
            Beacon(755, -354, -619),
            Beacon(553, 889, -390),
        ]);

        println!("{}", scanner0.could_overlap(&scanner1));
        println!("{:?}", scanner0.try_transform_to_find_offset(&mut scanner1));
    }

    #[test]
    fn test2() {
        let scanners = vec![
            Scanner::new(vec![
                Beacon(686, 422, 578),
                Beacon(605, 423, 415),
                Beacon(515, 917, -361),
                Beacon(-336, 658, 858),
                Beacon(95, 138, 22),
                Beacon(-476, 619, 847),
                Beacon(-340, -569, -846),
                Beacon(567, -361, 727),
                Beacon(-460, 603, -452),
                Beacon(669, -402, 600),
                Beacon(729, 430, 532),
                Beacon(-500, -761, 534),
                Beacon(-322, 571, 750),
                Beacon(-466, -666, -811),
                Beacon(-429, -592, 574),
                Beacon(-355, 545, -477),
                Beacon(703, -491, -529),
                Beacon(-328, -685, 520),
                Beacon(413, 935, -424),
                Beacon(-391, 539, -444),
                Beacon(586, -435, 557),
                Beacon(-364, -763, -893),
                Beacon(807, -499, -711),
                Beacon(755, -354, -619),
                Beacon(553, 889, -390),
            ]),
            Scanner::new(vec![
                Beacon(404, -588, -901),
                Beacon(528, -643, 409),
                Beacon(-838, 591, 734),
                Beacon(390, -675, -793),
                Beacon(-537, -823, -458),
                Beacon(-485, -357, 347),
                Beacon(-345, -311, 381),
                Beacon(-661, -816, -575),
                Beacon(-876, 649, 763),
                Beacon(-618, -824, -621),
                Beacon(553, 345, -567),
                Beacon(474, 580, 667),
                Beacon(-447, -329, 318),
                Beacon(-584, 868, -557),
                Beacon(544, -627, -890),
                Beacon(564, 392, -477),
                Beacon(455, 729, 728),
                Beacon(-892, 524, 684),
                Beacon(-689, 845, -530),
                Beacon(423, -701, 434),
                Beacon(7, -33, -71),
                Beacon(630, 319, -379),
                Beacon(443, 580, 662),
                Beacon(-789, 900, -551),
                Beacon(459, -707, 401),
            ]),
        ];
        dbg!(solve_scanners(scanners));
    }
}
