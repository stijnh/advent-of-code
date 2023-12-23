use crate::common::*;
use num::integer::lcm;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum ModuleKind<'a> {
    Broadcaster,
    Noop,
    FlipFlop { state: bool },
    Conjunction { memory: HashMap<&'a str, bool> },
}

#[derive(Debug, Clone)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    outgoing: Vec<&'a str>,
}

type Modules<'a> = HashMap<&'a str, Module<'a>>;

fn parse_modules(lines: Lines) -> Modules<'_> {
    let mut modules = HashMap::default();

    for line in lines {
        let (a, b) = line.split_once(" -> ").unwrap();

        let (name, kind) = if a == "broadcaster" {
            ("broadcaster", ModuleKind::Broadcaster)
        } else if a.starts_with("%") {
            (&a[1..], ModuleKind::FlipFlop { state: false })
        } else if a.starts_with("&") {
            (&a[1..], ModuleKind::Conjunction { memory: default() })
        } else {
            unreachable!()
        };

        modules.insert(
            name,
            Module {
                name,
                kind,
                outgoing: b.split(", ").collect(),
            },
        );
    }

    for (name, module) in modules.clone() {
        for out in module.outgoing {
            let target = modules.entry(out).or_insert(Module {
                name: out,
                kind: ModuleKind::Noop,
                outgoing: vec![],
            });

            if let ModuleKind::Conjunction { memory } = &mut target.kind {
                memory.insert(name, false);
            }
        }
    }

    modules
}

fn simulate_pulse<'a>(
    src: &'a str,
    dst: &'a str,
    pulse: bool,
    modules: &mut Modules<'a>,
    pulses: &mut VecDeque<(&'a str, &'a str, bool)>,
) {
    use ModuleKind::*;
    let module = modules.get_mut(dst).unwrap();

    let pulse = match &mut module.kind {
        Broadcaster => pulse,
        Noop => return,
        FlipFlop { state } => {
            if pulse {
                return;
            }

            *state = !*state;
            *state
        }
        Conjunction { memory } => {
            memory.insert(src, pulse);
            !memory.values().all(|&p| p)
        }
    };

    for &out in &module.outgoing {
        pulses.push_back((dst, out, pulse));
    }
}

fn count_pulses(presses: usize, modules: &mut Modules) -> [usize; 2] {
    let mut counts = [0, 0];
    let mut pulses = VecDeque::new();

    for _ in 0..presses {
        pulses.push_back(("button", "broadcaster", false));

        while let Some((src, dst, pulse)) = pulses.pop_front() {
            counts[pulse as usize] += 1;
            simulate_pulse(src, dst, pulse, modules, &mut pulses)
        }
    }

    counts
}

fn find_fewest_presses(modules: &mut Modules) -> usize {
    let (&target, _) = modules
        .iter()
        .find(|(name, module)| module.outgoing == &["rx"])
        .unwrap();
    let mut remaining: HashSet<_> = match &modules[target].kind {
        ModuleKind::Conjunction { memory } => memory.keys().copied().collect(),
        _ => unreachable!(),
    };

    let mut pulses = VecDeque::new();
    let mut presses = 0;
    let mut result = 1;

    loop {
        let Some((src, dst, pulse)) = pulses.pop_front() else {
            presses += 1;
            pulses.push_back(("button", "broadcaster", false));
            continue;
        };

        if dst == target && pulse && remaining.remove(src) {
            result = lcm(result, presses);

            if remaining.is_empty() {
                return result;
            }
        }

        simulate_pulse(src, dst, pulse, modules, &mut pulses);
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let mut modules = parse_modules(lines);

    let counts = count_pulses(1000, &mut modules.clone());
    println!("part A: {:}", counts[0] * counts[1]);

    let count = find_fewest_presses(&mut modules.clone());
    println!("part B: {:}", count);

    Ok(())
}
