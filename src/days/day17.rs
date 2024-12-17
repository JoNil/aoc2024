use std::fmt::Debug;

pub static INPUT: &str = include_str!("../input/17.txt");
pub static TEST_INPUT: &str = include_str!("../input/17_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/17_test_2.txt");

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
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

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::Adv as u8 => Ok(Instruction::Adv),
            x if x == Instruction::Bxl as u8 => Ok(Instruction::Bxl),
            x if x == Instruction::Bst as u8 => Ok(Instruction::Bst),
            x if x == Instruction::Jnz as u8 => Ok(Instruction::Jnz),
            x if x == Instruction::Bxc as u8 => Ok(Instruction::Bxc),
            x if x == Instruction::Out as u8 => Ok(Instruction::Out),
            x if x == Instruction::Bdv as u8 => Ok(Instruction::Bdv),
            x if x == Instruction::Cdv as u8 => Ok(Instruction::Cdv),
            _ => Err(()),
        }
    }
}

struct Machine {
    a: u32,
    b: u32,
    c: u32,
    ip: usize,
    program: Vec<u8>,
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("ip", &self.ip)
            .finish()
    }
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
                .map(|i| i.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn combo(&self, op: u8) -> u32 {
        match op {
            0..=3 => op as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid instruction"),
        }
    }

    fn run(&mut self, out: &mut Vec<u8>) {
        while self.ip + 1 < self.program.len() {
            let ins = Instruction::try_from(self.program[self.ip]).unwrap();
            let op = self.program[self.ip + 1];

            match ins {
                Instruction::Adv => {
                    self.a >>= self.combo(op);
                    self.ip += 2;
                }
                Instruction::Bxl => {
                    self.b ^= op as u32;
                    self.ip += 2;
                }
                Instruction::Bst => {
                    self.b = self.combo(op) & 0b111;
                    self.ip += 2;
                }
                Instruction::Jnz => {
                    if self.a != 0 {
                        self.ip = op as _;
                    } else {
                        self.ip += 2;
                    }
                }
                Instruction::Bxc => {
                    self.b ^= self.c;
                    self.ip += 2;
                }
                Instruction::Out => {
                    out.push((self.combo(op) & 0b111) as u8);
                    self.ip += 2;
                }
                Instruction::Bdv => {
                    self.b = self.a >> self.combo(op);
                    self.ip += 2;
                }
                Instruction::Cdv => {
                    self.c = self.a >> self.combo(op);
                    self.ip += 2;
                }
            }
        }
    }
}

#[test]
fn test_machine() {
    {
        let mut out: Vec<_> = Vec::new();
        let mut machine = Machine {
            a: 0,
            b: 0,
            c: 9,
            ip: 0,
            program: vec![2, 6],
        };

        machine.run(&mut out);

        assert_eq!(machine.b, 1);
    }

    {
        let mut out: Vec<_> = Vec::new();
        let mut machine = Machine {
            a: 10,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };

        machine.run(&mut out);

        assert_eq!(&out, &[0, 1, 2]);
    }

    {
        let mut out: Vec<_> = Vec::new();
        let mut machine = Machine {
            a: 2024,
            b: 0,
            c: 0,
            ip: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };

        machine.run(&mut out);

        assert_eq!(&out, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.a, 0);
    }

    {
        let mut out: Vec<_> = Vec::new();
        let mut machine = Machine {
            a: 0,
            b: 29,
            c: 0,
            ip: 0,
            program: vec![1, 7],
        };

        machine.run(&mut out);

        assert_eq!(machine.b, 26);
    }

    {
        let mut out: Vec<_> = Vec::new();
        let mut machine = Machine {
            a: 0,
            b: 2024,
            c: 43690,
            ip: 0,
            program: vec![4, 0],
        };

        machine.run(&mut out);

        assert_eq!(machine.b, 44354);
    }
}

pub fn a(input: &str) -> String {
    let mut machine = Machine::new(input);
    let mut out = Vec::new();

    machine.run(&mut out);

    let out = out.iter().map(|n| format!("{n}")).collect::<Vec<_>>();
    out.join(",")
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(a(TEST_INPUT_2), "2,7,6,5,6,0,2,3,1");
    assert_eq!(a(INPUT), "7,5,4,3,4,5,3,4,6");
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
