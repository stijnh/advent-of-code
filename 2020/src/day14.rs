use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

use Instr::*;
#[derive(Debug, Clone, Copy)]
enum Instr {
    Mask(u64, u64),
    Assign(u64, u64),
}

fn parse_input(lines: &[String]) -> Result<Vec<Instr>> {
    #[derive(Recap, Deserialize)]
    #[recap(regex = "mask = (?P<mask>[01X]+)")]
    struct Mask {
        mask: String,
    };

    #[derive(Recap, Deserialize)]
    #[recap(regex = "mem\\[(?P<addr>[0-9]+)\\] = (?P<value>[0-9]+)")]
    struct Assign {
        addr: u64,
        value: u64,
    };

    let mut output = vec![];
    for line in lines {
        if let Ok(Mask { mask }) = line.parse() {
            // X,0 -> 0, 1 -> 1
            let value = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();

            // 0,1 -> 0, X -> 1
            let mask = u64::from_str_radix(&mask.replace('1', "0").replace('X', "1"), 2).unwrap();

            output.push(Mask(mask, value));
        } else if let Ok(Assign { addr, value }) = line.parse() {
            output.push(Assign(addr, value));
        } else {
            bail!("failed to parse line: {}", line);
        }
    }

    Ok(output)
}

fn execute_v1(instrs: &[Instr]) -> HashMap<u64, u64> {
    let mut mem = HashMap::new();
    let (mut mask, mut bits) = (0, 0);

    for &instr in instrs {
        match instr {
            Mask(m, v) => {
                mask = m;
                bits = v;
            }
            Assign(addr, value) => {
                mem.insert(addr, (value & mask) | bits);
            }
        }
    }

    mem
}

// mask indicates X bits
fn address_decoder(addr: u64, mask: u64) -> Vec<u64> {
    if mask == 0 {
        return vec![addr];
    }

    let last_bit = mask & (-(mask as i64) as u64);
    let mut addrs = address_decoder(addr & !last_bit, mask & !last_bit);

    for i in 0..addrs.len() {
        addrs.push(addrs[i] | last_bit);
    }

    addrs
}

fn execute_v2(instrs: &[Instr]) -> HashMap<u64, u64> {
    let mut mem = HashMap::new();
    let (mut mask, mut bits) = (0, 0);

    for &instr in instrs {
        match instr {
            Mask(m, v) => {
                mask = m; // 0,1 -> 0, X -> 1
                bits = v; // X,0 -> 0, 1 -> 1
            }
            Assign(addr, value) => {
                for addr in address_decoder(addr | bits, mask) {
                    mem.insert(addr, value);
                }
            }
        }
    }

    mem
}

pub fn run() -> Result {
    let instrs = parse_input(&read_input("day14")?)?;

    let mem = execute_v1(&instrs);
    let total = sum(mem.values().copied());
    println!("part A: {:?}", total);

    let mem = execute_v2(&instrs);
    let total = sum(mem.values().copied());
    println!("part B: {:?}", total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let program = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];

        let program = parse_input(&program).unwrap();
        let _mem = execute_v1(&program);
    }
}
