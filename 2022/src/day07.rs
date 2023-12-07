use crate::common::*;

pub(crate) fn run(lines: Lines) -> Result {
    let mut path = vec![];
    let mut sizes = HashMap::<_, i32>::default();

    for &line in lines {
        if line == "$ cd /" {
            path = vec![];
        } else if line == "$ cd .." {
            path.pop();
        } else if let Some(matches) = find_regex("\\$ cd ([a-z]+)", line) {
            path.push(matches[1].to_string());
        } else if let Some(matches) = find_regex("([0-9]+) [a-z.]+", line) {
            let size = matches[1].parse::<i32>().unwrap();

            for i in 0..=path.len() {
                *sizes.entry(path[..i].to_vec()).or_default() += size;
            }
        }
    }

    let a = sizes.values().filter(|&&e| e <= 100000).sum::<i32>();
    println!("part A: {}", a);

    let capacity = 70000000;
    let required = 30000000;
    let used = sizes[&Vec::<String>::new()];
    let threshold = required + used - capacity;

    let b = sizes.values().filter(|&&e| e >= threshold).min();
    println!("part B: {:?}", b);

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
