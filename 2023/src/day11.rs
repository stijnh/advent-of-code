use crate::common::*;
use num::traits::real::Real;

type Pos = [i64; 2];

fn parse_galaxies(lines: Lines) -> HashSet<Pos> {
    let mut galaxies = HashSet::default();

    for (y, &line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                galaxies.insert([x as i64, y as i64]);
            }
        }
    }

    galaxies
}

fn expand_and_sum_1d(galaxies: &HashSet<Pos>, axis: usize, factor: i64) -> i64 {
    let mut xs = galaxies.iter().map(|p| p[axis]).collect::<HashSet<_>>();
    let max_x = *xs.iter().max().unwrap();
    let mut map = vec![0i64];
    let mut current = 0;

    for x in 0..max_x {
        current += if xs.remove(&x) { 1 } else { factor };
        map.push(current);
    }

    let mut dist = 0;

    for &p in galaxies {
        for &q in galaxies {
            dist += (map[p[axis] as usize] - map[q[axis] as usize]).abs();
        }
    }

    dist / 2
}

fn expand_and_sum(galaxies: &HashSet<Pos>, factor: i64) -> i64 {
    expand_and_sum_1d(&galaxies, 0, factor) + expand_and_sum_1d(&galaxies, 1, factor)
}

pub(crate) fn run(lines: Lines) -> Result {
    let g = parse_galaxies(lines);

    println!("part A: {}", expand_and_sum(&g, 2));
    println!("part B: {}", expand_and_sum(&g, 1000000));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const lines: &[&str] = &[
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn test() {
        let g = parse_galaxies(&lines);
        assert_eq!(expand_and_sum(&g, 2), 374);
        assert_eq!(expand_and_sum(&g, 10), 1030);
        assert_eq!(expand_and_sum(&g, 100), 8410);
    }
}
