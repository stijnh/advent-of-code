use crate::common::*;

type CubeSet = [i32; 3];

struct Game(Vec<CubeSet>);

impl Game {
    fn all_less(&self, that: &CubeSet) -> bool {
        (0..3).all(|i| self.0.iter().all(|v| v[i] <= that[i]))
    }

    fn calculate_power(&self) -> i32 {
        (0..3)
            .map(|i| self.0.iter().map(|set| set[i]).max().unwrap_or_default())
            .product()
    }
}

fn parse_game(line: &str) -> Game {
    let sets = line
        .split(": ")
        .nth(1)
        .unwrap_or_default()
        .split("; ")
        .map(|round| {
            let mut set = CubeSet::default();

            for cubes in round.split(", ") {
                let (count, color) = cubes.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();

                match color {
                    "red" => set[0] += count,
                    "green" => set[1] += count,
                    "blue" => set[2] += count,
                    _ => panic!("invalid color: `{color}`"),
                }
            }

            set
        })
        .collect();

    Game(sets)
}

pub(crate) fn run(lines: Lines) -> Result {
    let games = lines.iter().map(|line| parse_game(line)).collect_vec();
    let target = [12, 13, 14];

    let sum: usize = games
        .iter()
        .enumerate()
        .filter_map(|(index, game)| game.all_less(&target).then_some(index + 1))
        .sum();

    println!("part A: {sum}");

    let sum: i32 = games.iter().map(Game::calculate_power).sum();

    println!("part B: {sum}");

    Ok(())
}
