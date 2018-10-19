use colored::*;
use error::Result;
use field::Field;
use field::Pos;
use rand::prelude::*;
use std::char;
use std::fmt;
use std::io;
use std::io::{BufRead, Read, Write};
use termion::{clear, cursor};

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
            "{}{}{}\
             --------------------------------------------------------------------------------\n\
             {:?}\n\
             --------------------------------------------------------------------------------",
            clear::All,
            cursor::Hide,
            cursor::Goto(1, 1),
            self
        );
        let stdin = io::stdin();
        let stdout = io::stdout();
        let stderr = io::stderr();
        while self.tick()? {
            stdout.lock().flush()?;
            stderr.lock().flush()?;
            eprintln!(
                "{}\
                 --------------------------------------------------------------------------------\n\
                 {:?}\n\
                 --------------------------------------------------------------------------------",
                cursor::Goto(1, 1),
                self
            );
            stdin.lock().lines().next().unwrap()?.to_string();
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
                    self.stack.push(i64::from(ins));
                }
            }
        } else {
            match self.field.get(self.position) {
                n @ b'0'...b'9' => {
                    self.stack.push(i64::from(n - b'0'));
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
                    self.stack.push(b.checked_div(a).unwrap_or(0)); //read_u32()?));
                }
                b'%' => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(b.checked_rem(a).unwrap_or(0)); //read_u32()?));
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

                    self.stack.push(i64::from(val));
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
                    let mut buf = [0];
                    c.encode_utf8(&mut buf);
                    self.stack.push(i64::from(buf[0]));
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
                    y -= 1;
                }
            }
            Direction::Down => {
                if y == self.field.height() - 1 {
                    y = 0;
                } else {
                    y += 1;
                }
            }
            Direction::Right => {
                if x == self.field.width() - 1 {
                    x = 0;
                } else {
                    x += 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    x = self.field.width() - 1;
                } else {
                    x -= 1;
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
            "Direction: {:?}{}\nString Mode: {:?}{}\nStack: {:?}{}\nCurrent Instruction: {:?}{}",
            self.direction,
            clear::UntilNewline,
            self.string_mode,
            clear::UntilNewline,
            self.stack,
            clear::UntilNewline,
            self.field.get(self.position),
            clear::AfterCursor,
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
    loop {
        let st = read_string()?.parse();
        if let Ok(parsed) = st {
            return Ok(parsed);
        }
    }
}

fn read_string() -> Result<String> {
    let stdout = io::stdout();
    let stdin = io::stdin();
    stdout.lock().flush()?;
    let res = stdin.lock().lines().next().unwrap()?.to_string();
    Ok(res)
}

fn read_char() -> Result<char> {
    let stdout = io::stdout();
    let stdin = io::stdin();
    stdout.lock().flush()?;
    let mut buf = [0];
    stdin.lock().read_exact(&mut buf)?;
    Ok(char::from(buf[0]))
}
