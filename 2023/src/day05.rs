use crate::common::*;
use std::ops::Range;

type Mapping = Vec<(i64, i64, i64)>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Mapping>,
}

fn parse_almanac(lines: Lines) -> Almanac {
    let (before, after) = lines[0].split_once(':').unwrap();
    assert_eq!(before, "seeds");
    let seeds = parse_list::<i64>(after, ' ').unwrap();
    let mut maps = vec![];

    let mut index = 3;
    while index < lines.len() {
        let mut map = vec![];

        while let Some(line) = lines.get(index) {
            if line.is_empty() {
                break;
            }

            let mapping = parse_list::<i64>(lines[index], ' ').unwrap();
            map.push((mapping[0], mapping[1], mapping[2]));

            index += 1;
        }

        maps.push(map);
        index += 2;
    }

    Almanac { seeds, maps }
}

fn map_seed(mapping: &Mapping, input: i64) -> i64 {
    for &(dst_start, src_start, length) in mapping {
        if input >= src_start && input - src_start <= length {
            return input - src_start + dst_start;
        }
    }

    input
}

fn map_range(mappings: &[Mapping], range: Range<i64>) -> Vec<Range<i64>> {
    let Some((mapping, rest)) = mappings.split_first() else {
        return vec![range];
    };

    for &(dst_start, src_start, length) in mapping {
        let src_end = src_start + length;
        let overlap_start = i64::max(src_start, range.start);
        let overlap_end = i64::min(src_end, range.end);

        if overlap_start < overlap_end {
            let new_start = overlap_start - src_start + dst_start;
            let new_end = overlap_end - src_start + dst_start;
            let mut results = map_range(rest, new_start..new_end);

            if overlap_start != range.start {
                results.extend(map_range(mappings, range.start..overlap_start));
            }

            if overlap_end != range.end {
                results.extend(map_range(mappings, overlap_end..range.end));
            }

            return results;
        }
    }

    map_range(rest, range)
}

pub(crate) fn run(lines: Lines) -> Result {
    let almanac = parse_almanac(lines);

    let locations = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.maps.iter().fold(seed, |x, m| map_seed(m, x)))
        .collect_vec();

    println!("part A: {:?}", locations.iter().min());

    let ranges = almanac
        .seeds
        .iter()
        .tuples()
        .map(|(&x, &n)| map_range(&almanac.maps, x..(x + n)))
        .flatten();

    println!("part B: {:?}", ranges.min_by_key(|x| x.start));

    Ok(())
}
