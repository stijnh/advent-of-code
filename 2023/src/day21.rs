use crate::common::*;
use ndarray::Array2;
use num::traits::Euclid;

type Pos = [i64; 2];
type Map = Array2<char>;
type State = HashSet<Pos>;

fn parse_map(lines: Lines) -> (Map, Pos) {
    let (width, height) = (lines[0].chars().count(), lines.len());
    let mut start = [0, 0];
    let mut map = Map::default((width, height));

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            map[[x, y]] = c;

            if c == 'S' {
                map[[x, y]] = '.';
                start = [x as i64, y as i64];
            }
        }
    }

    (map, start)
}

fn simulate_step(map: &Map, state: &State) -> State {
    let (width, height) = map.dim();
    let mut new_state = State::default();

    for [x, y] in state {
        for [dx, dy] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let ix = (x + dx).rem_euclid(width as i64);
            let iy = (y + dy).rem_euclid(height as i64);

            if map[[ix as usize, iy as usize]] == '.' {
                new_state.insert([x + dx, y + dy]);
            }
        }
    }

    new_state
}

fn count_plots(map: &Map, start: Pos, nsteps: usize) -> usize {
    let mut state = State::from_iter([start]);

    for step in 0..nsteps {
        state = simulate_step(&map, &state);
    }

    state.len()
}

pub(crate) fn run(lines: Lines) -> Result {
    let (map, start) = parse_map(lines);

    println!("part A: {:?}", count_plots(&map, start, 64));

    let size = map.dim().0;

    let x = 26501365;
    let nrounds = (x / size) as i64;

    let x0 = x % size;
    let x1 = x0 + size as usize;
    let x2 = x1 + size as usize;

    let y0 = count_plots(&map, start, x0) as i64;
    let y1 = count_plots(&map, start, x1) as i64;
    let y2 = count_plots(&map, start, x2) as i64;

    // y = start value
    // dy = increment each round
    // ddy = increment of increment each round
    let mut y = y0;
    let mut dy = y1 - y0;
    let ddy = (y2 - y1) - (y1 - y0);

    for i in 0..nrounds {
        y += dy;
        dy += ddy;
    }

    println!("part B: {}", y);

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
