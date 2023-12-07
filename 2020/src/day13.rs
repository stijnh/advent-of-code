use crate::common::*;

fn parse_input(lines: &[String]) -> Result<(i32, Vec<(i32, i32)>)> {
    let timestamp = lines[0].parse()?;
    let busses = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| Some((i as i32, x.parse().ok()?)))
        .collect();
    Ok((timestamp, busses))
}

fn find_earliest_bus(timestamp: i32, busses: &[(i32, i32)]) -> (i32, i32) {
    busses
        .iter()
        .map(|&(_, b)| (b, (-timestamp).rem_euclid(b)))
        .min_by_key(|(_, t)| *t)
        .unwrap()
}

fn find_earliest_time(busses: &[(i32, i32)]) -> i128 {
    let mut result = 0 as i128;
    let mut factor = 1 as i128;

    for &(index, bus_id) in busses {
        let (bus_id, index) = (bus_id as i128, index as i128);
        let remaining = (-index).rem_euclid(bus_id);

        while result % bus_id != remaining {
            result += factor;
        }

        factor *= bus_id;
    }

    result
}

pub fn run() -> Result {
    let (timestamp, busses) = parse_input(&read_input("day13")?)?;
    let (bus_id, remaining) = find_earliest_bus(timestamp, &busses);

    println!(
        "part A: {} * {} = {}",
        bus_id,
        remaining,
        bus_id * remaining
    );

    let t = find_earliest_time(&busses);
    println!("part B: {}", t);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![(0, 17), (2, 13), (3, 19)];
        assert_eq!(find_earliest_time(&input), 3417);

        let input = vec![(0, 67), (1, 7), (2, 59), (3, 61)];
        assert_eq!(find_earliest_time(&input), 754018);

        let input = vec![(0, 67), (2, 7), (3, 59), (4, 61)];
        assert_eq!(find_earliest_time(&input), 779210);

        let input = vec![(0, 67), (1, 7), (3, 59), (4, 61)];
        assert_eq!(find_earliest_time(&input), 1261476);

        let input = vec![(0, 1789), (1, 37), (2, 47), (3, 1889)];
        assert_eq!(find_earliest_time(&input), 1202161486);
    }
}
