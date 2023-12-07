use crate::common::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instr {
    pub fn parse(s: &str) -> Result<Self> {
        let mut parts = s.trim().split_whitespace();
        let instr = parts.next().unwrap_or_default();
        let arg = parts
            .next()
            .unwrap_or_default()
            .parse()
            .with_context(|| format!("while parsing line {:?}", s))?;

        Ok(match instr {
            "nop" => Instr::Nop(arg),
            "acc" => Instr::Acc(arg),
            "jmp" => Instr::Jmp(arg),
            _ => bail!("unknown instruction {:?}", instr),
        })
    }
}

#[derive(Clone)]
pub struct Program(Vec<Instr>);

impl Program {
    pub fn parse_input(filename: &str) -> Result<Self> {
        let instrs = read_input(filename)?
            .iter()
            .map(|s| Instr::parse(&s))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self(instrs.into()))
    }

    pub fn run(&self) -> Process {
        Process {
            program: self.clone(),
            pc: 0,
            acc: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Program {
    type Output = Instr;

    fn index(&self, i: usize) -> &Instr {
        &self.0[i]
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, i: usize) -> &mut Instr {
        &mut self.0[i]
    }
}

pub struct Process {
    program: Program,
    pc: i32,
    acc: i32,
}

impl Process {
    pub fn advance(&mut self) -> Result {
        use Instr::*;

        if self.pc < 0 || self.pc as usize >= self.program.len() {
            bail!("program counter out of bounds");
        }

        match self.program[self.pc as usize] {
            Nop(_) => {
                self.pc += 1;
            }
            Acc(v) => {
                self.acc += v;
                self.pc += 1;
            }
            Jmp(v) => {
                self.pc += v;
            }
        }

        Ok(())
    }

    pub fn done(&self) -> bool {
        self.pc == self.program.len() as i32
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }
}
