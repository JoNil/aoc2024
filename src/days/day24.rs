#![allow(clippy::collapsible_if)]

use itertools::Itertools;
use rustc_hash::FxBuildHasher;

use crate::{AdventHashMap, AdventHashSet};
use std::{collections::HashMap, mem};

pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/24_test_2.txt");

#[derive(Clone, Copy)]
enum GateOp {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy)]
struct Gate<'a> {
    op: GateOp,
    in1: &'a str,
    in2: &'a str,
    out: &'a str,
}

impl<'a> Gate<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let (input, out) = s.split_once(" -> ").ok_or(())?;
        let mut input = input.split(' ');
        let in1 = input.next().ok_or(())?;
        let op = input.next().ok_or(())?;
        let in2 = input.next().ok_or(())?;

        let op = match op {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => Err(())?,
        };

        Ok(Gate { op, in1, in2, out })
    }
}

fn resolve_gate(
    gates: &AdventHashMap<&str, Gate>,
    wires: &AdventHashMap<&str, u8>,
    gate: &Gate,
) -> Option<u8> {
    let in1 = wires.get(gate.in1).copied().or_else(|| {
        gates
            .get(gate.in1)
            .and_then(|g| resolve_gate(gates, wires, g))
    })?;

    let in2 = wires.get(gate.in2).copied().or_else(|| {
        gates
            .get(gate.in2)
            .and_then(|g| resolve_gate(gates, wires, g))
    })?;

    Some(match gate.op {
        GateOp::And => in1 & in2,
        GateOp::Or => in1 | in2,
        GateOp::Xor => in1 ^ in2,
    })
}

pub fn a(input: &str) -> u64 {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let wires = wires
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, signal)| (name, signal.parse::<u8>().unwrap()))
        .collect::<AdventHashMap<_, _>>();

    let gates = gates
        .lines()
        .map(|l| {
            let g = Gate::from_str(l).unwrap();
            (g.out, g)
        })
        .collect::<AdventHashMap<_, _>>();

    let mut num = 0;

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, &wires, gate).unwrap();

        num |= (bit as u64) << i;
    }

    num
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 4);
    assert_eq!(a(TEST_INPUT_2), 2024);
    assert_eq!(a(INPUT), 36902370467952);
}

fn fetch_gates<'a>(
    gates: &AdventHashMap<&str, Gate<'a>>,
    gate: &Gate<'a>,
    res: &mut AdventHashSet<&'a str>,
) {
    res.insert(gate.out);

    if let Some(gate) = gates.get(gate.in1) {
        fetch_gates(gates, gate, res)
    }

    if let Some(gate) = gates.get(gate.in2) {
        fetch_gates(gates, gate, res)
    }
}

fn get_value(name: &str, wires: &AdventHashMap<&str, u8>) -> u64 {
    let mut num = 0;

    for i in 0.. {
        let name = format!("{name}{i:02}");

        let Some(bit) = wires.get(name.as_str()) else {
            break;
        };

        num |= (*bit as u64) << i;
    }

    num
}

fn find_loop(
    gates: &AdventHashMap<&str, Gate>,
    wires: &AdventHashMap<&str, u8>,
    gate: &Gate,
    original_gate: &str,
) -> bool {
    if gate.in1 == original_gate || gate.in2 == original_gate {
        return true;
    }

    let left = if wires.get(gate.in1).is_some() {
        false
    } else {
        gates
            .get(gate.in1)
            .map(|g| find_loop(gates, wires, g, original_gate))
            .unwrap()
    };

    let right = if wires.get(gate.in2).is_some() {
        false
    } else {
        gates
            .get(gate.in2)
            .map(|g| find_loop(gates, wires, g, original_gate))
            .unwrap()
    };

    left || right
}

fn has_loop(
    (a_str, b_str): (&str, &str),
    wires: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
) -> bool {
    let mut gates = gates.clone();

    {
        let [a, b] = gates.get_many_mut([a_str, b_str]);
        mem::swap(a.unwrap(), b.unwrap());
    }

    let a = gates.get(&a_str).unwrap();
    let b = gates.get(&b_str).unwrap();

    return find_loop(&gates, wires, a, a_str) || find_loop(&gates, wires, b, b_str);
}

fn test_combinations(
    combinations: &[Vec<&&str>],
    wires: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    z: u64,
) -> bool {
    let mut gates = gates.clone();

    for combination in combinations {
        let a = combination[0];
        let b = combination[1];

        if has_loop((a, b), wires, &gates) {
            return false;
        }

        let [a, b] = gates.get_many_mut([*a, *b]);
        mem::swap(a.unwrap(), b.unwrap());
    }

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, &wires, gate).unwrap();

        let should_be = (z >> i) & 1;

        if bit != should_be as u8 {
            return false;
        }
    }

    true
}

fn test_combination(
    (a_str, b_str): (&str, &str),
    wires: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    out: &str,
    should_be: u64,
) -> bool {
    let mut gates = gates.clone();

    {
        let [a, b] = gates.get_many_mut([a_str, b_str]);
        mem::swap(a.unwrap(), b.unwrap());
    }

    let Some(gate) = gates.get(out) else {
        panic!("Could not find");
    };

    let bit = resolve_gate(&gates, &wires, gate).unwrap();

    bit == should_be as u8
}

pub fn b(input: &str) -> i32 {
    let (wires, gates) = input.split_once("\n\n").unwrap();

    let wires = wires
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, signal)| (name, signal.parse::<u8>().unwrap()))
        .collect::<AdventHashMap<_, _>>();

    let gates = gates
        .lines()
        .map(|l| {
            let g = Gate::from_str(l).unwrap();
            (g.out, g)
        })
        .collect::<AdventHashMap<_, _>>();

    let x = get_value("x", &wires);
    let y = get_value("y", &wires);

    let z = x + y;

    println!("{x} + {y} = {z}");

    let mut right_bits = Vec::new();
    let mut wrong_bits = Vec::new();

    let mut all_wrong = AdventHashSet::default();
    let mut all_right = AdventHashSet::default();

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, &wires, gate).unwrap();

        let should_be = (z >> i) & 1;

        let mut possible_gates = AdventHashSet::default();
        fetch_gates(&gates, gate, &mut possible_gates);

        if bit != should_be as u8 {
            all_wrong.extend(possible_gates.iter().copied());
            wrong_bits.push((name, should_be, possible_gates));
        } else {
            all_right.extend(possible_gates.iter().copied());
            right_bits.push((name, should_be, possible_gates));
        }
    }

    let candidates = all_wrong
        .intersection(&all_right)
        .copied()
        .collect::<AdventHashSet<_>>();

    println!(
        "{:?}",
        wrong_bits.iter().map(|w| w.0.as_str()).collect::<Vec<_>>()
    );

    println!("{candidates:?}");

    let mut fixes = Vec::default();

    for (out, should_be, wrong) in &wrong_bits {
        let mut fix = AdventHashSet::default();
        for &wrong in wrong {
            for gate in candidates.iter().map(|g| gates.get(g).unwrap()) {
                if gate.out != wrong && out != wrong {
                    if !has_loop((wrong, gate.out), &wires, &gates) {
                        if test_combination((wrong, gate.out), &wires, &gates, out, *should_be) {
                            fix.insert((wrong, gate.out));
                        }
                    }
                }
            }
        }
        fixes.push((out.as_str(), fix));
    }

    for i in (0..fixes.len()).rev() {}

    1
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT), 0);
}
