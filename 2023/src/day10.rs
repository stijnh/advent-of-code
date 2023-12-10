use crate::common::*;
use std::collections::VecDeque;

type Pos = [i64; 2];
type Map = HashMap<Pos, char>;

fn parse_map(lines: Lines) -> (Map, Pos) {
    let mut map = HashMap::default();

    for (y, line) in enumerate(lines) {
        for (x, c) in line.chars().enumerate() {
            map.insert([x as i64, y as i64], c);
        }
    }

    let (&start, _) = find(&map, |(_, &c)| c == 'S').unwrap();

    // Find the symbol for start
    for symbol in ['|', '-', 'L', 'J', '7', 'F'] {
        map.insert(start, symbol);

        if neighbors(&map, start).all(|nb| neighbors(&map, nb).contains(&start)) {
            break;
        }
    }

    (map, start)
}

fn neighbors(map: &Map, [x, y]: Pos) -> impl Iterator<Item = Pos> {
    const NORTH: Pos = [0, -1];
    const SOUTH: Pos = [0, 1];
    const WEST: Pos = [-1, 0];
    const EAST: Pos = [1, 0];

    let delta = match map.get(&[x, y]).copied().unwrap_or_default() {
        '|' => &[NORTH, SOUTH] as &[_],
        '-' => &[EAST, WEST],
        'L' => &[NORTH, EAST],
        'J' => &[NORTH, WEST],
        '7' => &[SOUTH, WEST],
        'F' => &[SOUTH, EAST],
        _ => &[],
    };

    delta.iter().map(move |[dx, dy]| [x + dx, y + dy])
}

fn visit_map(map: &Map, start: Pos) -> HashMap<Pos, usize> {
    let mut visited = HashMap::default();
    let mut queue = VecDeque::from_iter([(start, 0)]);

    while let Some((x, steps)) = queue.pop_front() {
        for neighbor in neighbors(map, x) {
            if !visited.contains_key(&neighbor) {
                visited.insert(neighbor, steps + 1);
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    visited
}

fn find_area(map: &Map, path: &HashMap<Pos, usize>) -> HashSet<Pos> {
    let (x_min, x_max) = map.keys().map(|p| p[0]).minmax().into_option().unwrap();
    let (y_min, y_max) = map.keys().map(|p| p[1]).minmax().into_option().unwrap();

    let mut inside = HashSet::default();

    for y in y_min..=y_max {
        let mut is_inside = false;

        for x in x_min..=x_max {
            if path.contains_key(&[x, y]) {
                let c = map.get(&[x, y]).unwrap_or(&'.');
                is_inside = is_inside ^ ['|', 'L', 'J'].contains(c);
            } else if is_inside {
                inside.insert([x, y]);
            }
        }
    }

    inside
}

pub(crate) fn run(lines: Lines) -> Result {
    let (map, start) = parse_map(lines);

    let path = visit_map(&map, start);
    println!("part A: {:?}", path.values().max());

    let inside = find_area(&map, &path);
    println!("part B: {}", inside.len());

    Ok(())
}
