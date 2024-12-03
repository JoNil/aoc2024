use core::str;

pub static INPUT: &str = include_str!("../input/3.txt");
pub static TEST_INPUT: &str = include_str!("../input/3_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/3_test_2.txt");

struct Parse<'a> {
    input: &'a [u8],
    current: usize,
}

impl<'a> Parse<'a> {
    fn new(input: &'a str) -> Parse {
        Parse {
            input: input.as_bytes(),
            current: 0,
        }
    }

    fn skip(&mut self) {
        self.current += 1;
    }

    fn mul_op(&mut self) -> Option<()> {
        if self.current + 2 >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b'm'
            && self.input[self.current + 1] == b'u'
            && self.input[self.current + 2] == b'l'
        {
            self.current += 3;
            return Some(());
        }

        None
    }

    fn left_paran(&mut self) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b'(' {
            self.current += 1;
            return Some(());
        }

        None
    }

    fn right_paran(&mut self) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b')' {
            self.current += 1;
            return Some(());
        }

        None
    }

    fn comma(&mut self) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b',' {
            self.current += 1;
            return Some(());
        }

        None
    }

    fn peek_digit(&self, offset: usize) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        u8::is_ascii_digit(&self.input[self.current + offset]).then_some(())
    }

    fn number(&mut self) -> Option<i32> {
        let mut digits = 0;

        while self.peek_digit(digits).is_some() {
            digits += 1;
        }

        let s = str::from_utf8(&self.input[self.current..(self.current + digits)]).ok()?;
        let num = s.parse::<i32>().ok()?;

        self.current += digits;

        Some(num)
    }

    fn try_mul(&mut self) -> Option<(i32, i32)> {
        self.mul_op()?;
        self.left_paran()?;
        let n1 = self.number()?;
        self.comma();
        let n2 = self.number()?;
        self.right_paran()?;
        Some((n1, n2))
    }

    fn mul(&mut self) -> Option<(i32, i32)> {
        let original = self.current;

        if let Some(mul) = self.try_mul() {
            Some(mul)
        } else {
            self.current = original;
            None
        }
    }

    fn do_op(&mut self) -> Option<()> {
        if self.current + 3 >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b'd'
            && self.input[self.current + 1] == b'o'
            && self.input[self.current + 2] == b'('
            && self.input[self.current + 3] == b')'
        {
            self.current += 4;
            return Some(());
        }

        None
    }

    fn do_not_op(&mut self) -> Option<()> {
        if self.current + 6 >= self.input.len() {
            return None;
        }

        if self.input[self.current] == b'd'
            && self.input[self.current + 1] == b'o'
            && self.input[self.current + 2] == b'n'
            && self.input[self.current + 3] == b'\''
            && self.input[self.current + 4] == b't'
            && self.input[self.current + 5] == b'('
            && self.input[self.current + 6] == b')'
        {
            self.current += 7;
            return Some(());
        }

        None
    }
}

pub fn a(input: &str) -> i32 {
    let mut parse = Parse::new(input);

    let mut muls = Vec::new();

    while parse.current < parse.input.len() {
        if let Some(mul) = parse.mul() {
            muls.push(mul);
        } else {
            parse.skip();
        }
    }

    muls.iter().map(|(n1, n2)| n1 * n2).sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 161);
    assert_eq!(a(INPUT), 169021493);
}

pub fn b(input: &str) -> i32 {
    let mut parse = Parse::new(input);

    let mut muls = Vec::new();
    let mut enabled = true;

    while parse.current < parse.input.len() {
        if let Some(mul) = parse.mul() {
            if enabled {
                muls.push(mul);
            }
        } else if let Some(()) = parse.do_op() {
            enabled = true;
        } else if let Some(()) = parse.do_not_op() {
            enabled = false;
        } else {
            parse.skip();
        }
    }

    muls.iter().map(|(n1, n2)| n1 * n2).sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT_2), 48);
    assert_eq!(b(INPUT), 111762583);
}
