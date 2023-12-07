use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::str::FromStr;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Recap, Deserialize)]
#[recap(
    regex = r#"Blueprint (?P<id>[0-9]+): Each ore robot costs (?P<ore_robot_ore_required>[0-9]+) ore. Each clay robot costs (?P<clay_robot_ore_required>[0-9]+) ore. Each obsidian robot costs (?P<obsidian_robot_ore_required>[0-9]+) ore and (?P<obsidian_robot_clay_required>[0-9]+) clay. Each geode robot costs (?P<geode_robot_ore_required>[0-9]+) ore and (?P<geode_robot_obsidian_required>[0-9]+) obsidian."#
)]
struct RawBlueprint {
    id: i32,
    ore_robot_ore_required: i32,
    clay_robot_ore_required: i32,
    obsidian_robot_ore_required: i32,
    obsidian_robot_clay_required: i32,
    geode_robot_ore_required: i32,
    geode_robot_obsidian_required: i32,
}

#[derive(Default, Debug)]
struct Blueprint {
    id: i32,
    robots: [[i32; 4]; 4],
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let input = s.parse::<RawBlueprint>()?;
        let mut result = Blueprint::default();

        result.id = input.id;
        result.robots[ORE][ORE] = input.ore_robot_ore_required;
        result.robots[CLAY][ORE] = input.clay_robot_ore_required;
        result.robots[OBSIDIAN][ORE] = input.obsidian_robot_ore_required;
        result.robots[OBSIDIAN][CLAY] = input.obsidian_robot_clay_required;
        result.robots[GEODE][ORE] = input.geode_robot_ore_required;
        result.robots[GEODE][OBSIDIAN] = input.geode_robot_obsidian_required;
        Ok(result)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    time: i32,
    resources: [i32; 4],
    robots: [i32; 4],
}

fn simulate(blueprint: &Blueprint, max_time: i32) -> i32 {
    let mut queue = vec![State {
        time: 0,
        resources: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    }];

    let mut max_bots = [0; 4];
    for i in 0..4 {
        for j in 0..4 {
            max_bots[i] = max_bots[i].max(blueprint.robots[j][i]);
        }
    }
    max_bots[GEODE] = i32::MAX;

    let mut seen = HashSet::default();
    seen.insert(queue[0]);

    while let Some(state) = queue.pop() {
        for i in 0..4 {
            if state.robots[i] >= max_bots[i] {
                continue;
            }

            if (0..4).any(|j| state.robots[j] == 0 && blueprint.robots[i][j] > 0) {
                continue;
            }

            let mut time_required = 0;

            for j in 0..4 {
                let required = blueprint.robots[i][j];
                while state.resources[j] + state.robots[j] * time_required < required {
                    time_required += 1;
                }
            }

            if state.time + time_required + 1 >= max_time {
                continue;
            }

            let mut new_state = state;
            new_state.robots[i] += 1;
            new_state.time += time_required + 1;

            for j in 0..4 {
                new_state.resources[j] += state.robots[j] * (time_required + 1);
                new_state.resources[j] -= blueprint.robots[i][j];
            }

            if seen.insert(new_state) {
                queue.push(new_state);
            }
        }
    }

    seen.iter()
        .map(|s| s.resources[GEODE] + s.robots[GEODE] * (max_time - s.time))
        .max()
        .unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let blueprints = lines
        .iter()
        .map(|l| l.parse::<Blueprint>().unwrap())
        .collect_vec();

    let max = blueprints
        .iter()
        .map(|b| simulate(b, 24) * b.id)
        .sum::<i32>();
    println!("part A: {:?}", max);

    let result = blueprints[..3]
        .iter()
        .map(|b| simulate(b, 32) as i64)
        .product::<i64>();
    println!("part B: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut b = Blueprint::default();
        b.robots[ORE][ORE] = 4;
        b.robots[CLAY][ORE] = 2;
        b.robots[OBSIDIAN][ORE] = 3;
        b.robots[OBSIDIAN][CLAY] = 14;
        b.robots[GEODE][ORE] = 2;
        b.robots[GEODE][OBSIDIAN] = 7;
        assert_eq!(simulate(&b, 24), 9);

        let mut b = Blueprint::default();
        b.robots[ORE][ORE] = 2;
        b.robots[CLAY][ORE] = 3;
        b.robots[OBSIDIAN][ORE] = 3;
        b.robots[OBSIDIAN][CLAY] = 8;
        b.robots[GEODE][ORE] = 3;
        b.robots[GEODE][OBSIDIAN] = 12;
        assert_eq!(simulate(&b, 24), 12);
    }

    #[test]
    fn test_b() {
        //
    }
}
