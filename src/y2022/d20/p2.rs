use eyre::Context;
use to_method::To;

pub fn solve(input: &str) -> eyre::Result<isize> {
    let mut list = input
        .lines()
        .map(|line| line.trim().parse::<isize>().map(|num| num * 811589153))
        .collect::<Result<Vec<_>, _>>()?;

    let mut indexes = (0..list.len()).collect::<Vec<_>>();
    let len_minus_1 = list.len() - 1;
    for _ in 0..10 {
        for indexes_index in 0..indexes.len() {
            let og_idx = indexes[indexes_index];
            let delta = list[og_idx];
            if delta == 0 {
                continue;
            }
            let raw_new_idx = delta.checked_add_unsigned(og_idx).ok_or_else(|| {
                eyre::eyre!("can't add og_idx to delta (og_idx = {og_idx}, delta = {delta})")
            })?;
            let abs_raw_new_idx = raw_new_idx.abs().try_to::<usize>().wrap_err_with(|| {
                format!("can't take absolute value of raw_new_idx (raw_new_idx = {raw_new_idx})")
            })?;
            let next_multiple = abs_raw_new_idx - (abs_raw_new_idx % len_minus_1) + len_minus_1;
            let mut new_idx = next_multiple
            .checked_add_signed(raw_new_idx)
            .ok_or_else(|| eyre::eyre!("can't add raw_new_idx to next_multiple (raw_new_idx = {raw_new_idx}, next_multiple = {next_multiple}"))?
            % len_minus_1;
            if new_idx == 0 {
                new_idx = len_minus_1;
            }

            match og_idx.cmp(&new_idx) {
                std::cmp::Ordering::Less => {
                    list[og_idx..=new_idx].rotate_left(1);
                    for index in &mut indexes {
                        if *index == og_idx {
                            *index = new_idx;
                        } else if og_idx < *index && *index <= new_idx {
                            *index -= 1;
                        }
                    }
                }
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => {
                    list[new_idx..=og_idx].rotate_right(1);
                    for index in &mut indexes {
                        if *index == og_idx {
                            *index = new_idx;
                        } else if new_idx <= *index && *index < og_idx {
                            *index += 1;
                        }
                    }
                }
            }
        }
    }

    let idx_of_zero = list
        .iter()
        .position(|num| num == &0)
        .ok_or_else(|| eyre::eyre!("can't find the index of 0 (list = {list:?})"))?;
    Ok(list[(idx_of_zero + 1000) % list.len()]
        + list[(idx_of_zero + 2000) % list.len()]
        + list[(idx_of_zero + 3000) % list.len()])
}
