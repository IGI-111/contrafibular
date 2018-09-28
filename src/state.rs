use colored::*;
use error::Result;
use field::Field;
use instruction::Instruction;
use rand::prelude::*;
use std::char;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use termion::{clear, cursor};

pub struct State {
    stack: Vec<u32>,
    field: Field,
    position: (usize, usize),
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
                Instruction::StringMode => {
                    self.string_mode = false;
                }

                ins => {
                    self.stack.push(ins.to_u32());
                }
            }
        } else {
            match self.field.get(self.position) {
                &Instruction::Push(n) => {
                    self.stack.push(n);
                }
                Instruction::Noop => {}
                Instruction::Add => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a + b);
                }
                Instruction::Subtract => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(match b.checked_sub(a) {
                        Some(val) => val,
                        None => 0,
                    });
                }
                Instruction::Multiply => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a * b);
                }
                Instruction::Divide => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(b.checked_div(a).unwrap_or(read_u32()?));
                }
                Instruction::Modulo => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(b.checked_rem(a).unwrap_or(read_u32()?));
                }
                Instruction::Not => {
                    let a = self.safe_pop();
                    self.stack.push(if a == 0 { 1 } else { 0 });
                }
                Instruction::Greater => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(if b > a { 1 } else { 0 });
                }
                Instruction::Right => {
                    self.direction = Direction::Right;
                }
                Instruction::Left => {
                    self.direction = Direction::Left;
                }
                Instruction::Up => {
                    self.direction = Direction::Up;
                }
                Instruction::Down => {
                    self.direction = Direction::Down;
                }
                Instruction::Random => {
                    let mut rng = thread_rng();
                    self.direction = match rng.gen_range(0, 4) {
                        0 => Direction::Up,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        3 => Direction::Right,
                        _ => panic!("Number out of Range"),
                    }
                }
                Instruction::HorizontalIf => {
                    let a = self.safe_pop();
                    self.direction = if a == 0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                }
                Instruction::VerticalIf => {
                    let a = self.safe_pop();
                    self.direction = if a == 0 {
                        Direction::Down
                    } else {
                        Direction::Up
                    }
                }
                Instruction::StringMode => {
                    self.string_mode = true;
                }
                Instruction::Dup => {
                    let a = self.safe_pop();
                    self.stack.push(a);
                    self.stack.push(a);
                }
                Instruction::Swap => {
                    let a = self.safe_pop();
                    let b = self.safe_pop();
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Instruction::Pop => {
                    self.safe_pop();
                }
                Instruction::PopInt => {
                    let a = self.safe_pop();
                    print!("{} ", a);
                }
                Instruction::PopChar => {
                    let a = self.safe_pop();
                    print!("{}", char::from_u32(a).unwrap_or('�'));
                }
                Instruction::Bridge => {
                    self.step();
                }
                Instruction::Get => {
                    let y = self.safe_pop() as usize;
                    let x = self.safe_pop() as usize;
                    let val = self.field.get((x, y));

                    self.stack.push(val.to_u32());
                }
                Instruction::Put => {
                    let y = self.safe_pop() as usize;
                    let x = self.safe_pop() as usize;
                    let v = self.safe_pop();

                    let ins = Instruction::from_u32(v);
                    self.field.set((x, y), ins);
                }
                Instruction::PushInt => {
                    let a = read_u32()?;
                    self.stack.push(a);
                }
                Instruction::PushChar => {
                    let c = read_char()?;
                    self.stack.push(c as u32);
                }
                Instruction::End => {
                    return Ok(false);
                }
                &Instruction::Unknown(_) => {
                    self.direction = self.direction.reflect();
                }
            }
        }

        self.step();
        Ok(true)
    }

    fn safe_pop(&mut self) -> u32 {
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
                let c = char::from_u32(self.field.get((x, y)).to_u32()).unwrap_or('�');
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

fn read_u32() -> Result<u32> {
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
