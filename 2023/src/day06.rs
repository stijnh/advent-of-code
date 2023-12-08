use crate::common::*;
use std::iter::zip;

fn parse_races(times: &str, dists: &str) -> Vec<(i64, i64)> {
    let times = parse_list(times.strip_prefix("Time:").unwrap(), ' ').unwrap();
    let dists = parse_list(dists.strip_prefix("Distance:").unwrap(), ' ').unwrap();

    zip(times, dists).collect()
}

fn number_of_ways(total_time: i64, distance: i64) -> usize {
    // `distance + 1` since we want to exceed the distance
    let discriminant = total_time * total_time - 4 * (distance + 1);
    if discriminant < 0 {
        return 0;
    }

    let sqrt_discriminant = (discriminant as f64).sqrt();
    let root1 = ((total_time as f64 - sqrt_discriminant) / 2.0).ceil() as i64;
    let root2 = ((total_time as f64 + sqrt_discriminant) / 2.0).floor() as i64;

    // Count integer values between the roots within the range 0 to total_time
    (0.max(root1)..=total_time.min(root2)).count()
}

pub(crate) fn run(lines: Lines) -> Result {
    let races = parse_races(lines[0], lines[1]);

    let product: usize = races.iter().map(|&(t, d)| number_of_ways(t, d)).product();
    println!("part A: {}", product);

    // remove whitespaces between times
    let races = parse_races(&lines[0].replace(' ', ""), &lines[1].replace(' ', ""));
    let product: usize = races.iter().map(|&(t, d)| number_of_ways(t, d)).product();
    println!("part B: {}", product);

    Ok(())
}
