use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, terminal,
};
use glam::{ivec2, IVec2};
use std::{fmt::Display, io::stdout, str};

pub static INPUT: &str = include_str!("../input/14.txt");

#[derive(Clone)]
struct Map {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Map {
    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![b'.'; (width * height) as usize],
            width,
            height,
        }
    }

    fn set(&mut self, pos: IVec2, new: u8) -> bool {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return false;
        }

        if pos.y < 0 || pos.y >= self.height {
            return false;
        }

        self.data[index as usize] = new;

        true
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            writeln!(f, "{}", str::from_utf8(line).unwrap())?;
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
struct Robots {
    pos_x: Vec<i8>,
    pos_y: Vec<i8>,
    speed_x: Vec<i8>,
    speed_y: Vec<i8>,
}

pub fn b(input: &str, size: IVec2) {
    let mut robots = Robots::default();

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        let (x, y) = left.split_once(',').unwrap();
        let x = x.trim_start_matches("p=").parse().unwrap();
        let y = y.parse().unwrap();

        let (dx, dy) = right.split_once(',').unwrap();
        let dx = dx.trim_start_matches("v=").parse().unwrap();
        let dy = dy.parse().unwrap();

        robots.pos_x.push(x);
        robots.pos_y.push(y);
        robots.speed_x.push(dx);
        robots.speed_y.push(dy);
    }

    let mut steps: i32 = 0;
    let mut step: i32 = 7858;
    let mut stdout = stdout();

    'next: loop {
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).ok();

        let mut map = Map::empty(size.x, size.y);

        for _ in 0..step.abs() {
            for i in 0..robots.pos_x.len() {
                let x = (robots.pos_x[i] as i16 + step.signum() as i16 * robots.speed_x[i] as i16)
                    .rem_euclid(size.x as _) as i8;
                let y = (robots.pos_y[i] as i16 + step.signum() as i16 * robots.speed_y[i] as i16)
                    .rem_euclid(size.y as _) as i8;

                robots.pos_x[i] = x;
                robots.pos_y[i] = y;
            }
        }

        steps += step;

        for i in 0..robots.pos_x.len() {
            let x = robots.pos_x[i];
            let y = robots.pos_y[i];

            map.set(ivec2(x as _, y as _), b'x');
        }

        println!("{}", map);
        println!("{steps}");

        loop {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })) => {
                    continue 'next;
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })) => {
                    step += 1;
                    println!("{step}");
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    ..
                })) => {
                    step -= 1;
                    println!("{step}");
                }

                _ => (),
            }
        }
    }
}

fn main() {
    b(INPUT, ivec2(101, 103));
}
