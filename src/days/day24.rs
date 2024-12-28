#![allow(clippy::collapsible_if)]

use crate::{AdventHashMap, AdventHashSet};
use itertools::Itertools;
use std::{cmp::Ordering, mem};

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

    find_loop(&gates, wires, a, a_str) || find_loop(&gates, wires, b, b_str)
}

fn test_combination(
    combinations: &[(&str, &str)],
    wires: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    out: &str,
    should_be: u64,
) -> bool {
    let mut gates = gates.clone();

    for combination in combinations {
        let a = combination.0;
        let b = combination.1;

        if has_loop((a, b), wires, &gates) {
            return false;
        }

        let [a, b] = gates.get_many_mut([a, b]);
        mem::swap(a.unwrap(), b.unwrap());
    }

    let Some(gate) = gates.get(out) else {
        panic!("Could not find");
    };

    let bit = resolve_gate(&gates, wires, gate).unwrap();

    bit == should_be as u8
}

fn test_combinations(
    combinations: &[(&str, &str)],
    wires: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    z: u64,
) -> bool {
    let mut gates = gates.clone();

    for combination in combinations {
        let a = combination.0;
        let b = combination.1;

        if has_loop((a, b), wires, &gates) {
            return false;
        }

        let [a, b] = gates.get_many_mut([a, b]);
        mem::swap(a.unwrap(), b.unwrap());
    }

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, wires, gate).unwrap();

        let should_be = (z >> i) & 1;

        if bit != should_be as u8 {
            return false;
        }
    }

    true
}

fn find_bad_gates<'a>(
    gates: &AdventHashMap<&'a str, Gate<'a>>,
    wires: &AdventHashMap<&'a str, u8>,
    gates_for_output: &AdventHashMap<i32, AdventHashSet<&'a str>>,
    swapped_gates: &[(&'a str, &'a str)],
    outputs_to_correct: &[(String, i32, u64)],
    z: u64,
) -> Option<Vec<(&'a str, &'a str)>> {
    if swapped_gates.len() == 4 {
        return test_combinations(swapped_gates, wires, gates, z).then_some(swapped_gates.to_vec());
    }

    println!("{:?} {:?}", swapped_gates, outputs_to_correct);

    let (output_to_correct, gate_index, should_be) = &outputs_to_correct[0];
    let last = outputs_to_correct.last().unwrap().1;

    let set_of_gates_to_not_touch = (0..*gate_index)
        .flat_map(|i| gates_for_output.get(&i).unwrap())
        .copied()
        .collect::<AdventHashSet<_>>();

    let gates_to_try_against = ((*gate_index + 1)..=last)
        .flat_map(|i| gates_for_output.get(&i).unwrap())
        .copied()
        .collect::<AdventHashSet<_>>()
        .difference(&set_of_gates_to_not_touch)
        .copied()
        .collect::<AdventHashSet<_>>();

    let mut possible_pairs = AdventHashSet::default();

    for gate in gates_for_output
        .get(gate_index)
        .unwrap()
        .difference(&set_of_gates_to_not_touch)
        .copied()
    {
        for other in gates_to_try_against.iter().copied() {
            match gate.cmp(other) {
                Ordering::Less => {
                    possible_pairs.insert((gate, other));
                }
                Ordering::Equal => (),
                Ordering::Greater => {
                    possible_pairs.insert((other, gate));
                }
            }
        }
    }

    for candidate in &possible_pairs {
        let mut swapped = swapped_gates.to_vec();
        swapped.push(*candidate);

        if test_combination(
            &swapped,
            wires,
            gates,
            output_to_correct.as_str(),
            *should_be,
        ) {
            let found = find_bad_gates(
                gates,
                wires,
                gates_for_output,
                &swapped,
                &outputs_to_correct[1..],
                z,
            );

            if found.is_some() {
                return found;
            }
        }
    }

    None
}

pub fn b(input: &str) -> String {
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

    let mut gates_for_output = AdventHashMap::default();
    let mut outputs_to_correct = Vec::new();

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, &wires, gate).unwrap();

        let should_be = (z >> i) & 1;

        let mut possible_gates = AdventHashSet::default();
        fetch_gates(&gates, gate, &mut possible_gates);

        gates_for_output.insert(i, possible_gates);

        if bit != should_be as u8 {
            outputs_to_correct.push((name, i, should_be))
        }
    }

    find_bad_gates(
        &gates,
        &wires,
        &gates_for_output,
        &[],
        &outputs_to_correct,
        z,
    )
    .unwrap()
    .iter()
    .copied()
    .flat_map(|p| vec![p.0, p.1])
    .sorted()
    .join(",")
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT), "");
}
