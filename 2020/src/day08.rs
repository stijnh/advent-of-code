use crate::common::*;
use crate::gbcode::*;
use std::collections::HashSet;

fn run_until_done(program: &Program) -> Result<Process> {
    let mut visited = HashSet::new();
    let mut p = program.run();

    while !p.done() && visited.insert(p.pc()) {
        p.advance()?;
    }

    Ok(p)
}

fn fix_program(program: &Program) -> Result<Program> {
    use Instr::*;

    for i in 0..program.len() {
        let mut p = program.clone();

        // Flip nop and jmp, skip others
        match p[i] {
            Nop(v) => p[i] = Jmp(v),
            Jmp(v) => p[i] = Nop(v),
            _ => continue,
        }

        if let Ok(result) = run_until_done(&p) {
            if result.done() {
                return Ok(p);
            }
        }
    }

    bail!("failed to fix program");
}

pub fn run() -> Result {
    let p = Program::parse_input("day08")?;
    println!("part A: {}", run_until_done(&p)?.acc());

    let fixed_p = fix_program(&p)?;
    println!("part B: {}", run_until_done(&fixed_p)?.acc());

    Ok(())
}
