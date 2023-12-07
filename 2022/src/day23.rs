use crate::common::*;

type Pos = [i64; 2];

fn parse_elves(lines: Lines) -> HashSet<Pos> {
    let mut output = HashSet::default();

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                output.insert([x as i64, y as i64]);
            }
        }
    }

    output
}

fn simulate(input: &HashSet<Pos>, rounds: usize) -> HashSet<Pos> {
    let mut elves = input.clone();

    for round in 0..rounds {
        elves = simulate_once(&elves, round);
    }

    elves
}

fn simulate_convergence(input: &HashSet<Pos>) -> usize {
    let mut elves = input.clone();

    for round in 0..10000000 {
        let old_elves = elves;
        elves = simulate_once(&old_elves, round);

        if elves == old_elves {
            return round + 1;
        }
    }

    panic!("did not converge");
}

fn simulate_once(elves: &HashSet<Pos>, round: usize) -> HashSet<Pos> {
    let dirs = ['N', 'S', 'W', 'E'];
    let mut proposes = HashMap::default();
    let mut occupied = HashMap::<Pos, usize>::default();

    for &[x, y] in elves {
        let mut neighbors = 0;

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if elves.contains(&[x + dx, y + dy]) {
                    neighbors += 1;
                }
            }
        }

        let [mut nx, mut ny] = [x, y];

        // only itself
        if neighbors > 1 {
            for index in 0..dirs.len() {
                let [dx, dy] = match dirs[(index + round) % dirs.len()] {
                    'N' => [0, -1],
                    'E' => [1, 0],
                    'S' => [0, 1],
                    'W' => [-1, 0],
                    _ => unreachable!(),
                };

                if !elves.contains(&[x + dx, y + dy])
                    && !elves.contains(&[x + dx + dy, y + dy - dx])
                    && !elves.contains(&[x + dx - dy, y + dy + dx])
                {
                    nx += dx;
                    ny += dy;
                    break;
                }
            }
        }

        proposes.insert([x, y], [nx, ny]);
        *occupied.entry([nx, ny]).or_default() += 1;
    }

    proposes
        .into_iter()
        .map(|([x, y], [nx, ny])| {
            if occupied[&[nx, ny]] == 1 {
                [nx, ny]
            } else {
                [x, y]
            }
        })
        .collect()
}

fn compute_bounds(elves: &HashSet<Pos>) -> ([i64; 2], [i64; 2]) {
    let (min_x, max_x) = elves
        .iter()
        .map(|&[x, _]| x)
        .minmax()
        .into_option()
        .unwrap();

    let (min_y, max_y) = elves
        .iter()
        .map(|&[_, y]| y)
        .minmax()
        .into_option()
        .unwrap();

    ([min_x, max_x], [min_y, max_y])
}

fn compute_empty_tiles(elves: &HashSet<Pos>) -> i64 {
    let ([x0, x1], [y0, y1]) = compute_bounds(&elves);
    let area = (x1 - x0 + 1) * (y1 - y0 + 1);
    area - (elves.len() as i64)
}

#[allow(dead_code)]
fn visualize(elves: &HashSet<Pos>) {
    let ([min_x, max_x], [min_y, max_y]) = compute_bounds(elves);

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            if elves.contains(&[x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let elves = parse_elves(lines);

    println!("part A: {}", compute_empty_tiles(&simulate(&elves, 10)));
    println!("part B: {}", simulate_convergence(&elves));
    Ok(())
}
