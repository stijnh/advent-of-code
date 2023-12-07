use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::ops::RangeInclusive;

#[derive(Recap, Deserialize)]
#[recap(
    regex = r#"Sensor at x=(?P<sx>-?[0-9]+), y=(?P<sy>-?[0-9]+): closest beacon is at x=(?P<bx>-?[0-9]+), y=(?P<by>-?[0-9]+)"#
)]
struct Line {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    radius: i64,
}

fn parse_sensors(lines: Lines) -> Vec<Sensor> {
    lines
        .iter()
        .map(|&l| l.parse::<Line>().unwrap())
        .map(|l| Sensor {
            x: l.sx,
            y: l.sy,
            radius: (l.sx - l.bx).abs() + (l.sy - l.by).abs(),
        })
        .collect()
}

fn nonbeacon_position(sensors: &[Sensor], y: i64) -> (i64, Vec<RangeInclusive<i64>>) {
    let mut d = vec![];
    d.reserve(sensors.len() * 2);

    for sensor in sensors {
        let dy = (y - sensor.y).abs();
        let dx = sensor.radius - dy;

        if dx > 0 {
            d.push((sensor.x - dx, 1));
            d.push((sensor.x + dx, -1));
        }
    }

    d.sort();

    if d.is_empty() {
        return (0, vec![]);
    }

    let mut overlaps = 0;
    let mut last_x = d[0].0;
    let mut total = 0;
    let mut segments = Vec::with_capacity(1);
    let mut segment_start = 0;

    for &(x, p) in &d {
        if overlaps > 0 {
            total += x - last_x;
            last_x = x;
        } else {
            segment_start = x;
        }

        overlaps += p;

        if overlaps == 0 {
            segments.push(segment_start..=x);
        }
    }

    (total, segments)
}

pub(crate) fn run(lines: Lines) -> Result {
    let sensors = parse_sensors(lines);

    println!("part A: {:?}", nonbeacon_position(&sensors, 2000000));

    for y in 0..=4000000 {
        let (_, segments) = nonbeacon_position(&sensors, y);

        if segments.len() > 1 {
            let x = (segments[0].end() + segments[1].start()) / 2;
            println!("part B: x={:?} y={:?} freq={}", x, y, x * 4000000 + y);
        }
    }

    Ok(())
}
