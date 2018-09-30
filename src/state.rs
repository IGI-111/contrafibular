use colored::*;
use error::Result;
use field::Field;
use rand::prelude::*;
use std::char;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use termion::{clear, cursor};
use field::Pos;

pub struct State {
    stack: Vec<i64>,
    field: Field,
    position: Pos,
    direction: Direction,
    string_mode: bool,
}

impl State {
    pub fn with_field(field: Field) -> State {
        State {
            field,
            stack: Vec::new(),
            position: (0, 0),
            direction: Direction::Right,
            string_mode: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.tick()? {}
        Ok(())
    }

    pub fn run_debug(&mut self) -> Result<()> {
        eprintln!(
            "{}{}\
             --------------------------------------------------------------------------------\n\
             {:?}\n\
             --------------------------------------------------------------------------------",
            cursor::Goto(1, 1),
            clear::All,
            self
        );
        let stdin = io::stdin();
        while self.tick()? {
            io::stdout().flush().unwrap();
            io::stderr().flush().unwrap();
            eprintln!(
                "{}{}\
                 --------------------------------------------------------------------------------\n\
                 {:?}\n\
                 --------------------------------------------------------------------------------",
                cursor::Goto(1, 1),
                clear::All,
                self
            );
            stdin.lock().lines().next().unwrap().unwrap().to_string();
        }
        Ok(())
    }

    pub fn tick(&mut self) -> Result<bool> {
        if self.string_mode {
            match self.field.get(self.position) {
                b'"' => {
                    self.string_mode = false;
                }

                ins => {
                    self.stack.push(ins as i64);
                }
            }
        } else {
            match self.field.get(self.position) {
                n @ b'0'...b'9' => {
                    self.stack.push((n - b'0') as i64);
                }
                b' ' => {}
                b'+' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a + b);
                }
                b'-' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(match b.checked_sub(a) {
                        Some(val) => val,
                        None => 0,
                    });
                }
                b'*' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a * b);
                }
                b'/' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(b.checked_div(a).unwrap_or(0));//read_u32()?));
                }
                b'%' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(b.checked_rem(a).unwrap_or(0));//read_u32()?));
                }
                b'!' => {
                    let a = self.safe_pop();
                    self.stack.push(if a == 0 { 1 } else { 0 });
                }
                b'`' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(if b > a { 1 } else { 0 });
                }
                b'>' => {
                    self.direction = Direction::Right;
                }
                b'<' => {
                    self.direction = Direction::Left;
                }
                b'^' => {
                    self.direction = Direction::Up;
                }
                b'v' => {
                    self.direction = Direction::Down;
                }
                b'?' => {
                    let mut rng = thread_rng();
                    self.direction = match rng.gen_range(0, 4) {
                        0 => Direction::Up,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        3 => Direction::Right,
                        _ => panic!("Number out of Range"),
                    }
                }
                b'_' => {
                    let a = self.safe_pop();
                    self.direction = if a == 0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                }
                b'|' => {
                    let a = self.safe_pop();
                    self.direction = if a == 0 {
                        Direction::Down
                    } else {
                        Direction::Up
                    }
                }
                b'"' => {
                    self.string_mode = true;
                }
                b':' => {
                    let a = self.safe_pop();
                    self.stack.push(a);
                    self.stack.push(a);
                }
                b'\\' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a);
                    self.stack.push(b);
                }
                b'$' => {
                    self.safe_pop();
                }
                b'.' => {
                    let a = self.safe_pop();
                    print!("{} ", a);
                }
                b',' => {
                    let a = self.safe_pop();
                    print!("{}", char::from(a as u8));
                }
                b'#' => {
                    self.step();
                }
                b'g' => {
                    let y = self.safe_pop() as usize;
                    let x = self.safe_pop() as usize;
                    let val = self.field.get((x, y));

                    self.stack.push(val as i64);
                }
                b'p' => {
                    let y = self.safe_pop() as usize;
                    let x = self.safe_pop() as usize;
                    let v = self.safe_pop();

                    self.field.set((x, y), v as u8);
                }
                b'&' => {
                    let a = read_i64()?;
                    self.stack.push(a);
                }
                b'~' => {
                    let c = read_char()?;
                    let mut buf = [ 0 ];
                    c.encode_utf8(&mut buf);
                    self.stack.push(buf[0] as i64);
                }
                b'@' => {
                    return Ok(false);
                }
                _ => {
                    self.direction = self.direction.reflect();
                }
            }
        }

        self.step();
        Ok(true)
    }

    fn safe_pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or(0)
    }

    fn step(&mut self) {
        let (mut x, mut y) = self.position;
        match self.direction {
            Direction::Up => {
                if y == 0 {
                    y = self.field.height() - 1;
                } else {
                    y = y - 1;
                }
            }
            Direction::Down => {
                if y == self.field.height() - 1 {
                    y = 0;
                } else {
                    y = y + 1;
                }
            }
            Direction::Right => {
                if x == self.field.width() - 1 {
                    x = 0;
                } else {
                    x = x + 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    x = self.field.width() - 1;
                } else {
                    x = x - 1;
                }
            }
        }
        self.position = (x, y);
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.field.height() {
            for x in 0..self.field.width() {
                let c = char::from(self.field.get((x, y)));
                let st = c.to_string();

                if (x, y) == self.position {
                    write!(f, "{}", st.reversed())?;
                } else {
                    write!(f, "{}", st)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Direction: {:?}\nString Mode: {:?}\nStack: {:?}\nCurrent Instruction: {:?}",
            self.direction, self.string_mode, self.stack, self.field.get(self.position)
        )?;
        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reflect(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

fn read_i64() -> Result<i64> {
    io::stdout().flush()?;
    let stdin = io::stdin();
    let res = stdin
        .lock()
        .lines()
        .next()
        .unwrap()?
        .to_string()
        .parse()
        .unwrap();
    Ok(res)
}

fn read_char() -> Result<char> {
    io::stdout().flush()?;
    let stdin = io::stdin();
    let mut buf = [0];
    stdin.lock().read(&mut buf)?;
    Ok(char::from(buf[0]))
}
