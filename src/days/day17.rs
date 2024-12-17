pub static INPUT: &str = include_str!("../input/17.txt");
pub static TEST_INPUT: &str = include_str!("../input/17_test.txt");

#[derive(Copy, Clone, Debug)]
#[repr(i8)]
enum Instruction {
    Adv = 0b000,
    Bxl = 0b001,
    Bst = 0b010,
    Jnz = 0b011,
    Bxc = 0b100,
    Out = 0b101,
    Bdv = 0b110,
    Cdv = 0b111,
}

impl TryFrom<i8> for Instruction {
    type Error = ();

    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::Adv as i8 => Ok(Instruction::Adv),
            x if x == Instruction::Bxl as i8 => Ok(Instruction::Bxl),
            x if x == Instruction::Bst as i8 => Ok(Instruction::Bst),
            x if x == Instruction::Jnz as i8 => Ok(Instruction::Jnz),
            x if x == Instruction::Bxc as i8 => Ok(Instruction::Bxc),
            x if x == Instruction::Out as i8 => Ok(Instruction::Out),
            x if x == Instruction::Bdv as i8 => Ok(Instruction::Bdv),
            x if x == Instruction::Cdv as i8 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

struct Machine {
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
    program: Vec<i8>,
}

impl Machine {
    fn new(input: &str) -> Machine {
        let (machine_str, program_str) = input.split_once("\n\n").unwrap();
        let mut lines = machine_str.lines();

        Machine {
            a: lines
                .next()
                .unwrap()
                .trim_start_matches("Register A: ")
                .parse()
                .unwrap(),
            b: lines
                .next()
                .unwrap()
                .trim_start_matches("Register B: ")
                .parse()
                .unwrap(),
            c: lines
                .next()
                .unwrap()
                .trim_start_matches("Register C: ")
                .parse()
                .unwrap(),
            ip: 0,
            program: program_str
                .trim()
                .trim_start_matches("Program: ")
                .split(',')
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn combo(&self, op: i8) -> i32 {
        match op {
            0..4 => op as i32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid instruction"),
        }
    }
}

pub fn a(input: &str) -> String {
    let mut machine = Machine::new(input);
    let mut out = Vec::new();

    while machine.ip + 1 < machine.program.len() {
        let ins = Instruction::try_from(machine.program[machine.ip]).unwrap();
        let op = machine.program[machine.ip + 1];

        match ins {
            Instruction::Adv => {
                machine.a >>= machine.combo(op);
                machine.ip += 2;
            }
            Instruction::Bxl => {
                machine.b ^= op as i32;
                machine.ip += 2;
            }
            Instruction::Bst => {
                machine.b = machine.combo(op) & 0b111;
                machine.ip += 2;
            }
            Instruction::Jnz => {
                if machine.a != 0 {
                    machine.ip = op as _;
                } else {
                    machine.ip += 2;
                }
            }
            Instruction::Bxc => {
                machine.b ^= machine.c;
                machine.ip += 2;
            }
            Instruction::Out => {
                out.push(machine.combo(op) & 0b111);
                machine.ip += 2;
            }
            Instruction::Bdv => {
                machine.b >>= machine.combo(op);
                machine.ip += 2;
            }
            Instruction::Cdv => {
                machine.c >>= machine.combo(op);
                machine.ip += 2;
            }
        }
    }

    let out = out.iter().map(|n| format!("{n}")).collect::<Vec<_>>();
    out.join(",")
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(a(INPUT), "");
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
