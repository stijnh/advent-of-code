use crate::common::*;
use std::collections::VecDeque;
use std::iter::zip;

type Pos = [i64; 3];

#[derive(Copy, Clone, PartialEq)]
struct Cube {
    begin: Pos,
    end: Pos,
}

impl Cube {
    fn top(&self) -> impl Iterator<Item = Pos> {
        itertools::iproduct!(
            self.begin[0]..=self.end[0],
            self.begin[1]..=self.end[1],
            self.end[2]..=self.end[2]
        )
        .map(|(x, y, z)| [x, y, z])
    }

    fn bottom(&self) -> impl Iterator<Item = Pos> {
        itertools::iproduct!(
            self.begin[0]..=self.end[0],
            self.begin[1]..=self.end[1],
            self.begin[2]..=self.begin[2]
        )
        .map(|(x, y, z)| [x, y, z])
    }
}

fn parse_cubes(lines: Lines) -> Vec<Cube> {
    lines
        .iter()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let begin = parse_list(start, ',').unwrap().try_into().unwrap();
            let end = parse_list(end, ',').unwrap().try_into().unwrap();

            Cube { begin, end }
        })
        .collect()
}

fn drop_cubes(mut cubes: Vec<Cube>) -> Vec<Cube> {
    loop {
        let mut taken = HashSet::default();
        for cube in &cubes {
            for p in cube.top() {
                taken.insert(p);
            }
        }

        let mut has_updated = false;

        for cube in &mut cubes {
            let mut can_fall = cube
                .bottom()
                .all(|[x, y, z]| z > 1 && !taken.contains(&[x, y, z - 1]));

            if can_fall {
                has_updated = true;
                cube.begin[2] -= 1;
                cube.end[2] -= 1;
            }
        }

        if !has_updated {
            break cubes;
        }
    }
}

struct SupportGraph {
    supports: Vec<HashSet<usize>>,
    supported_by: Vec<HashSet<usize>>,
}

fn count_falling_if_removed(index: usize, cubes: &[Cube]) -> usize {
    let n = cubes.len();
    let mut supports: Vec<HashSet<usize>> = vec![default(); n];
    let mut supported_by: Vec<HashSet<usize>> = vec![default(); n];
    let mut taken = HashMap::default();

    for (i, cube) in enumerate(cubes) {
        for p in cube.top() {
            taken.insert(p, i);
        }
    }

    for (i, cube) in enumerate(cubes) {
        for [x, y, z] in cube.bottom() {
            if let Some(&j) = taken.get(&[x, y, z - 1]) {
                supported_by[i].insert(j);
                supports[j].insert(i);
            }
        }
    }

    let mut falls = HashSet::default();
    let mut queue = VecDeque::from_iter([index]);

    while let Some(index) = queue.pop_front() {
        if falls.insert(index) {
            for &above in &supports[index] {
                if supported_by[above].is_subset(&falls) {
                    queue.push_back(above);
                }
            }
        }
    }

    falls.len() - 1
}

pub(crate) fn run(lines: Lines) -> Result {
    let cubes = parse_cubes(lines);

    let cubes = drop_cubes(cubes);

    let mut sum: usize = (0..cubes.len())
        .filter(|&i| count_falling_if_removed(i, &cubes) == 0)
        .count();
    println!("part A: {}", sum);

    let mut sum: usize = (0..cubes.len())
        .map(|i| count_falling_if_removed(i, &cubes))
        .sum();
    println!("part B: {}", sum);

    Ok(())
}
