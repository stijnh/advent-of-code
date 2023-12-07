use crate::common::*;

fn play_game(mut cups: Vec<usize>, rounds: usize) -> Vec<usize> {
    let n = cups.len();
    let mut cur = cups[0] - 1;
    let mut next = vec![!0; n]; // next to cup i is cup `next[i]`

    // 1-indexing to 0-indexing
    for i in 0..n {
        next[cups[i] - 1] = cups[(i + 1) % n] - 1;
    }

    for _ in 0..rounds {
        let a = next[cur];
        let b = next[a];
        let c = next[b];

        let selected = [cur, a, b, c];
        let mut dest = cur;

        while selected.contains(&dest) {
            dest = (dest + n - 1) % n;
        }

        next[cur] = next[c];
        next[c] = next[dest];
        next[dest] = a;

        cur = next[cur];
    }

    // 0-indexing to 1-indexing
    cur = 0;
    for i in 0..n {
        cups[i] = cur + 1;
        cur = next[cur];
    }

    cups
}

pub fn run() -> Result {
    let mut cups = vec![];
    for c in read_input("day23")?[0].chars() {
        cups.push(c.to_digit(10).unwrap() as _);
    }

    let output = play_game(cups.clone(), 100);
    println!("part A: {}", output.iter().join(""));

    while cups.len() < 1_000_000 {
        cups.push(cups.len() + 1);
    }

    let output = play_game(cups.clone(), 10_000_000);
    println!("part B: {}", output[1] * output[2]);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game() {
        let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

        let output = play_game(input.clone(), 10);
        assert_eq!(&output, &[1, 9, 2, 6, 5, 8, 3, 7, 4]);

        let output = play_game(input.clone(), 100);
        assert_eq!(&output, &[1, 6, 7, 3, 8, 4, 5, 2, 9]);

        let mut input = input.clone();
        while input.len() < 1_000_000 {
            input.push(input.len() + 1);
        }

        // Takes to long to run for CI
        // let mut output = play_game(input, 10_000_000);
        //assert_eq!(&output[..3], &[1, 934001, 159792]);
    }
}
