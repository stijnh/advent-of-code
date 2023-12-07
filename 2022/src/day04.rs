use crate::common::*;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"(?P<a0>[0-9]+)-(?P<a1>[0-9]+),(?P<b0>[0-9]+)-(?P<b1>[0-9]+)"#)]
struct Line {
    a0: i32,
    a1: i32,
    b0: i32,
    b1: i32,
}

impl Line {
    fn is_fully_contained(&self) -> bool {
        self.a0 >= self.b0 && self.a1 <= self.b1 || self.b0 >= self.a0 && self.b1 <= self.a1
    }

    fn is_overlapping(&self) -> bool {
        self.a0 <= self.b1 && self.a1 >= self.b0
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let lines = lines
        .iter()
        .map(|&l| Line::try_from(l))
        .collect::<Result<Vec<_>, _>>()?;

    let count = lines.iter().filter(|l| l.is_fully_contained()).count();
    println!("part A: {}", count);

    let count = lines.iter().filter(|l| l.is_overlapping()).count();
    println!("part A: {}", count);

    Ok(())
}
