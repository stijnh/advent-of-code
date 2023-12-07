use crate::common::*;
use recap::Recap;
use serde::Deserialize;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Recap, Deserialize)]
#[recap(regex = r#"(?P<x>[0-9]+),(?P<y>[0-9]+),(?P<z>[0-9]+)"#)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

const FACES: [[i64; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn count_faces(cubes: &[Cube]) -> i64 {
    let mut total = 0;
    let cubes = HashSet::from_iter(cubes);

    for cube in &cubes {
        for [dx, dy, dz] in FACES {
            if !cubes.contains(&Cube {
                x: cube.x + dx,
                y: cube.y + dy,
                z: cube.z + dz,
            }) {
                total += 1;
            }
        }
    }

    total
}

fn count_exterior_faces(cubes: &[Cube]) -> i64 {
    let cubes = HashSet::from_iter(cubes);

    let (mut x0, mut x1) = (0, 0);
    let (mut y0, mut y1) = (0, 0);
    let (mut z0, mut z1) = (0, 0);

    for &cube in &cubes {
        x0 = x0.min(cube.x - 1);
        y0 = y0.min(cube.y - 1);
        z0 = z0.min(cube.z - 1);

        x1 = x1.max(cube.x + 1);
        y1 = y1.max(cube.y + 1);
        z1 = z1.max(cube.z + 1);
    }

    let mut queue = vec![];
    queue.push(Cube {
        x: x0,
        y: y0,
        z: z0,
    });

    let mut exterior = HashSet::<Cube>::default();
    exterior.insert(queue[0]);

    while let Some(Cube { x, y, z }) = queue.pop() {
        for [dx, dy, dz] in FACES {
            let p = Cube {
                x: x + dx,
                y: y + dy,
                z: z + dz,
            };

            if (x0..=x1).contains(&p.x)
                && (y0..=y1).contains(&p.y)
                && (z0..=z1).contains(&p.z)
                && !cubes.contains(&p)
                && exterior.insert(p)
            {
                queue.push(p);
            }
        }
    }

    let mut total = 0;
    let cubes = HashSet::from_iter(cubes);

    for cube in &cubes {
        for [dx, dy, dz] in FACES {
            if exterior.contains(&Cube {
                x: cube.x + dx,
                y: cube.y + dy,
                z: cube.z + dz,
            }) {
                total += 1;
            }
        }
    }

    total
}

pub(crate) fn run(lines: Lines) -> Result {
    let cubes = lines
        .iter()
        .map(|l| l.parse::<Cube>().unwrap())
        .collect_vec();

    println!("part A: {}", count_faces(&cubes));
    println!("part B: {}", count_exterior_faces(&cubes));

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
