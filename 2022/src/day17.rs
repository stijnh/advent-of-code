use crate::common::*;

const CAVE_WIDTH: i64 = 7;
const ROCK_SIZE: i64 = 4;
type Rock = [[char; ROCK_SIZE as usize]; ROCK_SIZE as usize];

const ROCKS: [Rock; 5] = [
    [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        ['#', '#', '#', '#'],
    ],
    [
        [' ', ' ', ' ', ' '],
        [' ', '#', ' ', ' '],
        ['#', '#', '#', ' '],
        [' ', '#', ' ', ' '],
    ],
    [
        [' ', ' ', ' ', ' '],
        [' ', ' ', '#', ' '],
        [' ', ' ', '#', ' '],
        ['#', '#', '#', ' '],
    ],
    [
        ['#', ' ', ' ', ' '],
        ['#', ' ', ' ', ' '],
        ['#', ' ', ' ', ' '],
        ['#', ' ', ' ', ' '],
    ],
    [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        ['#', '#', ' ', ' '],
        ['#', '#', ' ', ' '],
    ],
];

fn rock_intersects_anything(rx: i64, ry: i64, rock: &Rock, solids: &HashSet<(i64, i64)>) -> bool {
    for dx in 0..ROCK_SIZE {
        for dy in 0..ROCK_SIZE {
            if rock[(ROCK_SIZE - dy - 1) as usize][dx as usize] == '#' {
                let (x, y) = (rx + dx, ry + dy);

                if x < 0 || x >= CAVE_WIDTH {
                    return true;
                }

                if y < 0 {
                    return true;
                }

                if solids.contains(&(x, y)) {
                    return true;
                }
            }
        }
    }

    false
}

fn insert_rock(rx: i64, ry: i64, rock: &Rock, solid: &mut HashSet<(i64, i64)>) -> i64 {
    let mut highest = 0;

    for dx in 0..ROCK_SIZE {
        for dy in 0..ROCK_SIZE {
            if rock[(ROCK_SIZE - dy - 1) as usize][dx as usize] == '#' {
                let (sx, sy) = (rx + dx, ry + dy);
                solid.insert((sx, sy));
                highest = highest.max(sy);
            }
        }
    }

    highest
}

fn drop_rock(
    mut x: i64,
    mut y: i64,
    jets: &[char],
    jet_index: &mut usize,
    rock: &Rock,
    solid: &HashSet<(i64, i64)>,
) -> (i64, i64) {
    loop {
        match jets[(*jet_index) % jets.len()] {
            '<' => {
                if !rock_intersects_anything(x - 1, y, rock, &solid) {
                    x -= 1;
                }
            }
            '>' => {
                if !rock_intersects_anything(x + 1, y, rock, &solid) {
                    x += 1;
                }
            }
            c => {
                panic!("invalid jet {}", c)
            }
        }

        *jet_index += 1;

        if rock_intersects_anything(x, y - 1, rock, &solid) {
            break (x, y);
        } else {
            y -= 1;
        }
    }
}

pub(crate) fn simulate(jets: &[char], nrocks: usize) -> i64 {
    let mut solid = HashSet::<(i64, i64)>::default();
    let mut highest_y = -1;
    let mut jet_index = 0;
    let mut states = vec![];
    let mut rock_index = 0;

    let cycle = 'outer: loop {
        let rock = &ROCKS[rock_index % ROCKS.len()];
        let (x, y) = drop_rock(2, highest_y + 4, jets, &mut jet_index, rock, &solid);
        let new_highest_y = insert_rock(x, y, rock, &mut solid);
        let delta = (new_highest_y - highest_y).max(0);
        highest_y += delta;

        states.push((jet_index % jets.len(), delta));
        let n = states.len();

        for cycle in (0..states.len() / 2).step_by(ROCKS.len()).skip(1) {
            if states[n - cycle..] == states[n - 2 * cycle..n - cycle] {
                break 'outer cycle;
            }
        }

        rock_index += 1;
    };

    let ncycles = nrocks / cycle;
    let cycle_height = states[states.len() - cycle..]
        .iter()
        .map(|(_, delta)| delta)
        .sum::<i64>();

    let remainder = nrocks % cycle;
    let remainder_height = states[..remainder]
        .iter()
        .map(|(_, delta)| delta)
        .sum::<i64>();

    ncycles as i64 * cycle_height + remainder_height
}

pub(crate) fn run(lines: Lines) -> Result {
    let jets = lines[0].chars().collect_vec();
    let height = simulate(&jets, 2022);
    println!("part A: {:?}", height);

    let height = simulate(&jets, 1000000000000);
    println!("part B: {:?}", height);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        //
    }

    #[test]
    fn test_b() {
        //
    }
}
