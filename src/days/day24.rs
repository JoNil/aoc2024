#![allow(clippy::collapsible_if)]

use crate::{AdventHashMap, AdventHashSet};
use itertools::Itertools;
use std::{cmp::Ordering, mem, ops::RangeInclusive};

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

fn fetch_wires<'a>(
    gates: &AdventHashMap<&str, Gate<'a>>,
    wires: &AdventHashMap<&str, u8>,
    gate: &Gate<'a>,
    res: &mut AdventHashSet<&'a str>,
) {
    if wires.get(gate.in1).is_some() {
        res.insert(gate.in1);
    } else if let Some(gate) = gates.get(gate.in1) {
        fetch_wires(gates, wires, gate, res);
    }

    if wires.get(gate.in2).is_some() {
        res.insert(gate.in2);
    } else if let Some(gate) = gates.get(gate.in2) {
        fetch_wires(gates, wires, gate, res);
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
    wires2: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    out: &str,
    should_be: u64,
    should_be_2: u64,
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
    let bit2 = resolve_gate(&gates, wires2, gate).unwrap();

    bit == should_be as u8 && bit2 == should_be_2 as u8
}

fn test_combinations(
    combinations: &[(&str, &str)],
    wires: &AdventHashMap<&str, u8>,
    wires2: &AdventHashMap<&str, u8>,
    gates: &AdventHashMap<&str, Gate>,
    z: u64,
    z2: u64,
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
        let bit2 = resolve_gate(&gates, wires2, gate).unwrap();

        let should_be = (z >> i) & 1;
        let should_be_2 = (z2 >> i) & 1;

        if bit != should_be as u8 || bit2 != should_be_2 as u8 {
            return false;
        }
    }

    true
}

#[allow(clippy::too_many_arguments)]
fn find_bad_gates<'a>(
    could_affect_output_upto: &AdventHashMap<&'a str, RangeInclusive<i32>>,
    gates: &AdventHashMap<&'a str, Gate<'a>>,
    wires: &AdventHashMap<&'a str, u8>,
    wires2: &AdventHashMap<&'a str, u8>,
    gates_for_output: &AdventHashMap<i32, AdventHashSet<&'a str>>,
    swapped_gates: &[(&'a str, &'a str)],
    outputs_to_correct: &[(String, i32, u64, u64)],
    z: u64,
    z2: u64,
) -> Option<Vec<(&'a str, &'a str)>> {
    if swapped_gates.len() == 4 {
        return test_combinations(swapped_gates, wires, wires2, gates, z, z2)
            .then_some(swapped_gates.to_vec());
    }

    if outputs_to_correct.is_empty() {
        return None;
    }

    let (output_to_correct, gate_index, should_be, should_be_2) = &outputs_to_correct[0];

    let mut possible_pairs = AdventHashSet::default();

    for &gate in gates_for_output.get(gate_index).unwrap() {
        for other in gates.keys().copied() {
            if could_affect_output_upto.contains_key(other) {
                let range = could_affect_output_upto.get(other).unwrap();

                if !range.contains(gate_index) {
                    continue;
                }
            }

            if other.starts_with('z') && gate.starts_with('z') {
                continue;
            }

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
            wires2,
            gates,
            output_to_correct.as_str(),
            *should_be,
            *should_be_2,
        ) {
            if swapped_gates.len() <= 2 {
                println!("{swapped:?}: {}", possible_pairs.len());
            }

            let found = find_bad_gates(
                could_affect_output_upto,
                gates,
                wires,
                wires2,
                gates_for_output,
                &swapped,
                &outputs_to_correct[1..],
                z,
                z2,
            );

            if found.is_some() {
                return found;
            }
        }
    }

    find_bad_gates(
        could_affect_output_upto,
        gates,
        wires,
        wires2,
        gates_for_output,
        swapped_gates,
        &outputs_to_correct[1..],
        z,
        z2,
    )
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

    let mut input_count = (wires.len() / 2) as i32;
    let mut output_count = 0;

    for i in 0.. {
        let gate_name = format!("z{i:02}");

        if !gates.contains_key(gate_name.as_str()) {
            output_count = i;
            break;
        };
    }

    let x = get_value("x", &wires);
    let y = get_value("y", &wires);
    let y2 = y ^ ((1 << output_count) - 1);

    let mut wires2 = wires
        .iter()
        .map(|(k, v)| (*k, *v))
        .filter(|(k, _)| k.starts_with('x'))
        .collect::<AdventHashMap<_, _>>();

    for i in 0..input_count {
        let name = format!("y{i:02}");
        let value = (y2 >> i) & 1;
        wires2.insert(wires.get_key_value(name.as_str()).unwrap().0, value as u8);
    }

    let z = x + y;
    let z2 = x + y2;

    let gate_wires = gates
        .iter()
        .map(|(name, gate)| {
            let mut gate_wires = AdventHashSet::default();
            fetch_wires(&gates, &wires, gate, &mut gate_wires);
            (*name, gate_wires)
        })
        .collect::<AdventHashMap<_, _>>();

    let could_affect_output_upto = gate_wires
        .iter()
        .map(|(k, v)| {
            let range = v
                .iter()
                .copied()
                .filter(|v| v.starts_with('x') || v.starts_with('y'))
                .map(|i| {
                    i.trim_start_matches('x')
                        .trim_start_matches('y')
                        .parse::<i32>()
                        .unwrap()
                })
                .minmax()
                .into_option()
                .unwrap();

            (*k, range.0..=range.1)
        })
        .collect::<AdventHashMap<_, _>>();

    println!("{could_affect_output_upto:#?}");

    let mut outputs_is_missing = AdventHashMap::<i32, AdventHashSet<&str>>::default();
    let mut outputs_has_out_of_range = AdventHashMap::<i32, AdventHashSet<&str>>::default();

    for i in 0.. {
        let gate_name = format!("z{i:02}");

        let Some(gate_wires) = gate_wires.get(gate_name.as_str()) else {
            break;
        };

        for in_i in 0..=i.min(input_count - 1) {
            let in1_name = format!("x{in_i:02}");
            let in2_name = format!("y{in_i:02}");

            if !gate_wires.contains(in1_name.as_str()) {
                outputs_is_missing
                    .entry(i)
                    .or_default()
                    .insert(wires.get_key_value(in1_name.as_str()).unwrap().0);
            }

            if !gate_wires.contains(in2_name.as_str()) {
                outputs_is_missing
                    .entry(i)
                    .or_default()
                    .insert(wires.get_key_value(in2_name.as_str()).unwrap().0);
            }
        }

        for in_i in (i + 1)..input_count {
            let in1_name = format!("x{in_i:02}");
            let in2_name = format!("y{in_i:02}");

            if gate_wires.contains(in1_name.as_str()) {
                outputs_has_out_of_range
                    .entry(i)
                    .or_default()
                    .insert(wires.get_key_value(in1_name.as_str()).unwrap().0);
            }

            if gate_wires.contains(in2_name.as_str()) {
                outputs_has_out_of_range
                    .entry(i)
                    .or_default()
                    .insert(wires.get_key_value(in2_name.as_str()).unwrap().0);
            }
        }
    }

    for (oim, ins) in &outputs_is_missing {
        let ins = ins.iter().copied().sorted().collect::<Vec<_>>();
        println!("{:?}", ins);
    }

    println!("OIM {outputs_is_missing:#?}");
    println!("OOR {outputs_has_out_of_range:#?}");

    panic!();

    let mut gates_for_output = AdventHashMap::default();
    let mut outputs_to_correct = Vec::new();

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        let bit = resolve_gate(&gates, &wires, gate).unwrap();
        let bit2 = resolve_gate(&gates, &wires2, gate).unwrap();

        let should_be = (z >> i) & 1;
        let should_be_2 = (z2 >> i) & 1;

        let mut possible_gates = AdventHashSet::default();
        fetch_gates(&gates, gate, &mut possible_gates);

        gates_for_output.insert(i, possible_gates);

        if bit != should_be as u8 || bit2 != should_be_2 as u8
        /*|| outputs_is_missing.contains_key(&i)
        || outputs_has_out_of_range.contains_key(&i)*/
        {
            outputs_to_correct.push((name, i, should_be, should_be_2))
        }
    }

    outputs_to_correct.reverse();

    if let Some(res) = find_bad_gates(
        &could_affect_output_upto,
        &gates,
        &wires,
        &wires2,
        &gates_for_output,
        &[],
        &outputs_to_correct,
        z,
        z2,
    ) {
        res.iter()
            .copied()
            .flat_map(|p| vec![p.0, p.1])
            .sorted()
            .join(",")
    } else {
        panic!("Did not find answer");
    }
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT), "");
}
