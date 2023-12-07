use crate::common::*;
use enum_map::{Enum, EnumMap};
use ndarray::prelude::*;

fn parse_input(lines: &[String]) -> Result<(Vec<usize>, Vec<Array2<char>>)> {
    let mut ids = vec![];
    let mut maps = vec![];
    let mut lines = lines.iter().map(|s| &**s);

    while let Some(line) = lines.next() {
        let index = find("^Tile ([0-9]+):$", line)
            .and_then(|c| c[1].parse::<usize>().ok())
            .ok_or(anyhow!("invalid line: {:?}", line))?;

        let mut map = vec![];
        for _ in 0..10 {
            let line = lines.next().unwrap_or_default();

            if line.len() != 10 {
                bail!("invalid line: {:?}", line);
            }

            map.extend(line.chars());
        }

        let map = Array::from(map).into_shape((10, 10))?;

        ids.push(index);
        maps.push(map);
        lines.next();
    }

    Ok((ids, maps))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

impl Side {
    fn opposite(self) -> Self {
        use Side::*;
        match self {
            Top => Bottom,
            Bottom => Top,
            Left => Right,
            Right => Left,
        }
    }
}

fn variations<'a>(array: ArrayView2<'a, char>) -> impl Iterator<Item = ArrayView2<'a, char>> {
    (0..8).map(move |i| {
        let mut m = array;

        if i & 0x1 == 0 {
            m = m.reversed_axes()
        }

        if i & 0x2 == 0 {
            m.invert_axis(Axis(0));
        }

        if i & 0x4 == 0 {
            m.invert_axis(Axis(1));
        }

        m
    })
}

fn rearrange_maps(maps: &mut [Array2<char>]) -> Array2<usize> {
    use Side::*;

    let n = maps.len();
    let mut neighbors = Vec::<EnumMap<_, Option<usize>>>::new();
    let mut remaining: HashSet<_> = (0..n).collect();
    let mut queue = vec![];

    neighbors.resize(n, default());
    queue.push(0);

    while let Some(i) = queue.pop() {
        if !remaining.remove(&i) {
            continue;
        }

        for &side in &[Top, Right, Bottom, Left] {
            // Side already found by a neighbor.
            if neighbors[i][side].is_some() {
                continue;
            }

            let a = maps[i].view();
            let mut result = None;

            'a: for &j in &remaining {
                for b in variations(maps[j].view()) {
                    let matches = match side {
                        Top => a.row(0) == b.row(9),
                        Right => a.column(9) == b.column(0),
                        Bottom => a.row(9) == b.row(0),
                        Left => a.column(0) == b.column(9),
                    };

                    if matches {
                        result = Some((j, b.to_owned()));
                        break 'a;
                    }
                }
            }

            if let Some((j, m)) = result {
                maps[j] = m;
                neighbors[i][side] = Some(j);
                neighbors[j][side.opposite()] = Some(i);
                queue.push(j);
            }
        }
    }

    let mut grid = Array2::<usize>::zeros((12, 12));
    let mut index = (0..n)
        .filter(|&i| neighbors[i][Top] == None && neighbors[i][Left] == None)
        .next()
        .unwrap();

    // Fill first column
    grid[[0, 0]] = index;
    for i in 0..11 {
        index = neighbors[index][Bottom].unwrap();
        grid[[i + 1, 0]] = index;
    }

    // Fill each row
    for i in 0..12 {
        let mut index = grid[[i, 0]];

        for j in 0..11 {
            index = neighbors[index][Right].unwrap();
            grid[[i, j + 1]] = index;
        }
    }

    grid
}

fn reassemble_maps(maps: &[Array2<char>], grid: ArrayView2<usize>) -> Array2<char> {
    let mut result = Array2::<char>::from_elem((96, 96), '0');

    for i in 0..12 {
        for j in 0..12 {
            let map = &maps[grid[[i, j]]];

            let borderless = map.slice(s![1..9, 1..9]);
            result
                .slice_mut(s![8 * i..8 * (i + 1), 8 * j..8 * (j + 1)])
                .assign(&borderless);
        }
    }

    result
}

fn find_sea_monsters(map: ArrayView2<char>) -> Array2<char> {
    let drawing = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let mut spots = vec![];
    let mut output = map.to_owned();

    for (di, line) in enumerate(&drawing) {
        for (dj, c) in enumerate(line.chars()) {
            if c == '#' {
                spots.push((di, dj));
            }
        }
    }

    for ((i, j), _) in map.indexed_iter() {
        let mut valid = true;

        for &(di, dj) in &spots {
            valid &= map.get([i + di, j + dj]) == Some(&'#');
        }

        if !valid {
            continue;
        }

        for &(di, dj) in &spots {
            output[[i + di, j + dj]] = 'O';
        }
    }

    output
}

fn print_map(map: ArrayView2<char>) {
    for row in map.genrows() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn run() -> Result {
    let (ids, mut maps) = parse_input(&read_input("day20")?)?;

    let grid = rearrange_maps(&mut maps);
    println!(
        "part A: {}",
        ids[grid[[0, 0]]] * ids[grid[[11, 0]]] * ids[grid[[0, 11]]] * ids[grid[[11, 11]]]
    );

    let result = reassemble_maps(&maps, grid.view());

    println!("result:");
    print_map(result.view());
    println!();

    for m in variations(result.view()) {
        let result = find_sea_monsters(m);

        if all(&result, |&c| c != 'O') {
            continue;
        }

        let count = result.iter().filter(|&&c| c == '#').count();

        println!("part B: {}", count);
        print_map(result.view());
        println!();
    }

    Ok(())
}
