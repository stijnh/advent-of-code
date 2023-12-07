use crate::common::*;
use std::cmp;

type Point = [i32; 2];

fn parse_points(line: &str) -> impl Iterator<Item = Point> + '_ {
    find_regex_all("([0-9]+),([0-9]+)", line)
        .into_iter()
        .map(|m| [m[1].parse().unwrap(), m[2].parse().unwrap()])
}

fn build_cave(lines: Lines) -> HashSet<Point> {
    let mut rocks = HashSet::default();

    for &line in lines {
        for (a, b) in parse_points(line).tuple_windows() {
            if a[0] == b[0] {
                let x = a[0];
                for y in cmp::min(a[1], b[1])..=cmp::max(a[1], b[1]) {
                    rocks.insert([x, y]);
                }
            } else if a[1] == b[1] {
                let y = a[1];
                for x in cmp::min(a[0], b[0])..=cmp::max(a[0], b[0]) {
                    rocks.insert([x, y]);
                }
            }
        }
    }

    rocks
}

fn drop_sand(objects: &HashSet<Point>, floor: i32, has_void: bool) -> Option<Point> {
    let [mut x, mut y] = [500, 0];

    if objects.contains(&[x, y]) {
        return None;
    }

    loop {
        if y + 1 == floor {
            break if has_void { None } else { Some([x, y]) };
        } else if !objects.contains(&[x, y + 1]) {
            y += 1;
        } else if !objects.contains(&[x - 1, y + 1]) {
            x -= 1;
            y += 1;
        } else if !objects.contains(&[x + 1, y + 1]) {
            x += 1;
            y += 1;
        } else {
            break Some([x, y]);
        }
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let rocks = build_cave(lines);
    let floor = rocks.iter().map(|&[_, y]| y).max().unwrap() + 2;

    let mut objects = rocks.clone();
    while let Some(p) = drop_sand(&objects, floor, true) {
        objects.insert(p);
    }

    println!("part A: {}", objects.len() - rocks.len());

    let mut objects = rocks.clone();
    while let Some(p) = drop_sand(&objects, floor, false) {
        objects.insert(p);
    }

    println!("part B: {}", objects.len() - rocks.len());

    Ok(())
}
