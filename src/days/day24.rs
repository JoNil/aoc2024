use crate::AdventHashMap;

pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/24_test_2.txt");

enum GateOp {
    And,
    Or,
    Xor,
}

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

    let mut out_bits = Vec::new();

    for i in 0.. {
        let name = format!("z{i:02}");

        let Some(gate) = gates.get(name.as_str()) else {
            break;
        };

        out_bits.push(resolve_gate(&gates, &wires, gate).unwrap());
    }

    let mut num = 0;

    for bit in out_bits.iter().rev() {
        num <<= 1;
        num |= *bit as u64;
    }

    num
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 4);
    assert_eq!(a(TEST_INPUT_2), 2024);
    assert_eq!(a(INPUT), 36902370467952);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT_2), 0);
    assert_eq!(b(INPUT), 0);
}
