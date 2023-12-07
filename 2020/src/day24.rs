use crate::common::*;

type Tile = (i32, i32);

fn decode_tile(line: &str) -> Result<Tile> {
    let (mut x, mut y) = (0, 0);
    let mut cmd = "".to_string();

    for c in line.chars() {
        cmd.push(c);

        match &*cmd {
            "e" => x += 1,
            "w" => x -= 1,
            "se" => y += 1,
            "nw" => y -= 1,
            "ne" => {
                x += 1;
                y -= 1;
            }
            "sw" => {
                x -= 1;
                y += 1;
            }
            _ => continue,
        }

        cmd.clear();
    }

    if !cmd.is_empty() {
        bail!("invalid command {:?}", cmd);
    }

    Ok((x, y))
}

fn parse_input(lines: &[String]) -> Result<HashSet<Tile>> {
    let mut tiles = HashSet::<Tile>::default();

    for line in lines {
        let coords = decode_tile(line)?;

        if !tiles.insert(coords) {
            tiles.remove(&coords);
        }
    }

    Ok(tiles)
}

fn flip_tiles(black_tiles: &mut HashSet<Tile>) {
    let mut count = HashMap::<Tile, usize>::default();

    for &(x, y) in &*black_tiles {
        let _ = *count.entry((x, y)).or_default();

        for &i in &[
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
        ] {
            *count.entry(i).or_default() += 1;
        }
    }

    for (i, n) in count {
        if n == 2 {
            black_tiles.insert(i);
        } else if n != 1 {
            black_tiles.remove(&i);
        }
    }
}

pub fn run() -> Result {
    let mut black_tiles = parse_input(&read_input("day24")?)?;

    println!("part A: {:?}", black_tiles.len());

    for _ in 0..100 {
        flip_tiles(&mut black_tiles);
    }

    println!("part B: {:?}", black_tiles.len());

    Ok(())
}
