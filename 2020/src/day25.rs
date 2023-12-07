use crate::common::*;

fn decrypt(mut sub: usize, pubkey: usize) -> usize {
    let mut it = 1;

    while sub != pubkey {
        it += 1;
        sub = (sub * 7) % 20201227;
    }

    it
}

fn encrypt(sub: usize, it: usize) -> usize {
    (0..it).fold(1, |acc, _| (acc * sub) % 20201227)
}

pub fn run() -> Result {
    let numbers = read_input("day25")?
        .into_iter()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let card = numbers[0];
    let door = numbers[1];
    let answer = encrypt(door, decrypt(7, card));

    println!("answer A: {}", answer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_determine() {
        assert_eq!(decrypt(7, 5764801), 8);
        assert_eq!(decrypt(7, 17807724), 11);

        assert_eq!(encrypt(7, 8), 5764801);
        assert_eq!(encrypt(7, 11), 17807724);

        assert_eq!(encrypt(17807724, 8), 14897079);
        assert_eq!(encrypt(5764801, 11), 14897079);
    }
}
