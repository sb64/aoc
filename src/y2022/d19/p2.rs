use std::collections::HashMap;

use super::{parse_blueprint, Blueprint};

fn max_increase_in_geodes_opened(
    blueprint: Blueprint,
    time_remaining: u32,
    ore: u32,
    ore_robots: u32,
    max_ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    max_clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    max_obsidian_robots: u32,
    geode_robots: u32,
    cache: &mut HashMap<(u32, u32, u32, u32, u32, u32, u32, u32), u32>,
    max_additional_geodes_opened_for_remaining_time: &mut HashMap<u32, u32>,
) -> u32 {
    if time_remaining == 0 {
        return 0;
    }

    if let Some(&max_geodes) = cache.get(&(
        time_remaining,
        ore,
        ore_robots,
        clay,
        clay_robots,
        obsidian,
        obsidian_robots,
        geode_robots,
    )) {
        return max_geodes;
    }

    if let Some(&max_additional_geodes_opened) =
        max_additional_geodes_opened_for_remaining_time.get(&time_remaining)
    {
        if geode_robots * time_remaining + time_remaining * (time_remaining + 1) / 2
            < max_additional_geodes_opened
        {
            return 0;
        }
    }

    let mut max_additional_geodes_opened = 0;

    if ore >= blueprint.geode_robot_cost.0 && obsidian >= blueprint.geode_robot_cost.1 {
        max_additional_geodes_opened =
            max_additional_geodes_opened.max(max_increase_in_geodes_opened(
                blueprint,
                time_remaining - 1,
                ore + ore_robots - blueprint.geode_robot_cost.0,
                ore_robots,
                max_ore_robots,
                clay + clay_robots,
                clay_robots,
                max_clay_robots,
                obsidian + obsidian_robots - blueprint.geode_robot_cost.1,
                obsidian_robots,
                max_obsidian_robots,
                geode_robots + 1,
                cache,
                max_additional_geodes_opened_for_remaining_time,
            ));
    }

    if obsidian_robots < max_obsidian_robots
        && ore >= blueprint.obsidian_robot_cost.0
        && clay >= blueprint.obsidian_robot_cost.1
    {
        max_additional_geodes_opened =
            max_additional_geodes_opened.max(max_increase_in_geodes_opened(
                blueprint,
                time_remaining - 1,
                ore + ore_robots - blueprint.obsidian_robot_cost.0,
                ore_robots,
                max_ore_robots,
                clay + clay_robots - blueprint.obsidian_robot_cost.1,
                clay_robots,
                max_clay_robots,
                obsidian + obsidian_robots,
                obsidian_robots + 1,
                max_obsidian_robots,
                geode_robots,
                cache,
                max_additional_geodes_opened_for_remaining_time,
            ));
    }

    if ore_robots < max_ore_robots && ore >= blueprint.ore_robot_cost {
        max_additional_geodes_opened =
            max_additional_geodes_opened.max(max_increase_in_geodes_opened(
                blueprint,
                time_remaining - 1,
                ore + ore_robots - blueprint.ore_robot_cost,
                ore_robots + 1,
                max_ore_robots,
                clay + clay_robots,
                clay_robots,
                max_clay_robots,
                obsidian + obsidian_robots,
                obsidian_robots,
                max_obsidian_robots,
                geode_robots,
                cache,
                max_additional_geodes_opened_for_remaining_time,
            ));
    }

    if clay_robots < max_clay_robots && ore >= blueprint.clay_robot_cost {
        max_additional_geodes_opened =
            max_additional_geodes_opened.max(max_increase_in_geodes_opened(
                blueprint,
                time_remaining - 1,
                ore + ore_robots - blueprint.clay_robot_cost,
                ore_robots,
                max_ore_robots,
                clay + clay_robots,
                clay_robots + 1,
                max_clay_robots,
                obsidian + obsidian_robots,
                obsidian_robots,
                max_obsidian_robots,
                geode_robots,
                cache,
                max_additional_geodes_opened_for_remaining_time,
            ));
    }

    max_additional_geodes_opened = max_additional_geodes_opened.max(max_increase_in_geodes_opened(
        blueprint,
        time_remaining - 1,
        ore + ore_robots,
        ore_robots,
        max_ore_robots,
        clay + clay_robots,
        clay_robots,
        max_clay_robots,
        obsidian + obsidian_robots,
        obsidian_robots,
        max_obsidian_robots,
        geode_robots,
        cache,
        max_additional_geodes_opened_for_remaining_time,
    ));

    max_additional_geodes_opened += geode_robots;

    cache.insert(
        (
            time_remaining,
            ore,
            ore_robots,
            clay,
            clay_robots,
            obsidian,
            obsidian_robots,
            geode_robots,
        ),
        max_additional_geodes_opened,
    );

    max_additional_geodes_opened_for_remaining_time
        .entry(time_remaining)
        .and_modify(|max| *max = max_additional_geodes_opened.max(*max))
        .or_insert(max_additional_geodes_opened);

    max_additional_geodes_opened
}

pub fn solve(input: &str) -> eyre::Result<u32> {
    let mut product_of_geodes_opened = 1;
    for line in input.lines().take(3) {
        let (_, blueprint) =
            parse_blueprint(line).map_err(|err| eyre::eyre!("can't parse blueprint: {err}"))?;
        let mut cache = HashMap::new();
        let mut max_additional_geodes_opened_for_remaining_time = HashMap::new();
        let max_geodes = max_increase_in_geodes_opened(
            blueprint,
            32,
            0,
            1,
            blueprint
                .ore_robot_cost
                .max(blueprint.clay_robot_cost)
                .max(blueprint.obsidian_robot_cost.0)
                .max(blueprint.geode_robot_cost.0),
            0,
            0,
            blueprint.obsidian_robot_cost.1,
            0,
            0,
            blueprint.geode_robot_cost.1,
            0,
            &mut cache,
            &mut max_additional_geodes_opened_for_remaining_time,
        );
        product_of_geodes_opened *= max_geodes;
    }
    Ok(product_of_geodes_opened)
}
