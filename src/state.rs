use colored::*;
use field::Field;
use field::{FIELD_HEIGHT, FIELD_WIDTH};
use instruction::Instruction;
use rand::prelude::*;
use std::fmt;
use std::io;
use std::io::BufRead;

pub struct State {
    stack: Vec<u8>,
    program: Field,
    position: (usize, usize),
    direction: Direction,
    string_mode: bool,
}

impl State {
    pub fn with_field(program: Field) -> State {
        State {
            program,
            stack: Vec::new(),
            position: (0, 0),
            direction: Direction::Right,
            string_mode: false,
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.tick()? {}
        Ok(())
    }

    pub fn tick(&mut self) -> Result<bool, RuntimeError> {
        if self.string_mode {
            match self.program.get(self.position) {
                Instruction::StringMode => {
                    self.string_mode = false;
                }

                ins => {
                    let c = ins.to_char();

                    let mut buf = [0];
                    c.encode_utf8(&mut buf); // this is ASCII and therefore one byte
                    self.stack.push(buf[0]);
                }
            }
        } else {
            match self.program.get(self.position) {
                &Instruction::Push(n) => {
                    self.stack.push(n);
                }
                Instruction::Noop => {}
                Instruction::Add => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(a + b);
                }
                Instruction::Subtract => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(b - a);
                }
                Instruction::Multiply => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(a * b);
                }
                Instruction::Divide => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(b / a);
                }
                Instruction::Modulo => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(b % a);
                }
                Instruction::Not => {
                    let a = self.try_pop()?;
                    self.stack.push(if a == 0 { 1 } else { 0 });
                }
                Instruction::Greater => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
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
                    let a = self.try_pop()?;
                    self.direction = if a == 0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                }
                Instruction::VerticalIf => {
                    let a = self.try_pop()?;
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
                    let a = self.try_pop()?;
                    self.stack.push(a);
                    self.stack.push(a);
                }
                Instruction::Swap => {
                    let a = self.try_pop()?;
                    let b = self.try_pop()?;
                    self.stack.push(b);
                    self.stack.push(a);
                }
                Instruction::Pop => {
                    self.try_pop()?;
                }
                Instruction::PopInt => {
                    let a = self.try_pop()?;
                    print!("{} ", a);
                }
                Instruction::PopChar => {
                    let a = self.try_pop()?;
                    print!("{}", char::from(a));
                }
                Instruction::Bridge => {
                    self.step();
                }
                Instruction::Get => {
                    let y = self.try_pop()? as usize;
                    let x = self.try_pop()? as usize;
                    let val = self.program.get((x, y));

                    let mut buf = [0];
                    val.to_char().encode_utf8(&mut buf); // this is ASCII and therefore one byte
                    self.stack.push(buf[0]);
                }
                Instruction::Put => {
                    let y = self.try_pop()? as usize;
                    let x = self.try_pop()? as usize;
                    let v = self.try_pop()?;

                    let ins = Instruction::from_char(char::from(v));
                    self.program.set((x, y), ins);
                }
                Instruction::PushInt => {
                    print!("Enter a number: ");
                    let stdin = io::stdin();
                    let a: u8 = stdin
                        .lock()
                        .lines()
                        .next()
                        .unwrap()
                        .unwrap()
                        .to_string()
                        .parse()
                        .unwrap();
                    self.stack.push(a);
                }
                Instruction::PushChar => {
                    print!("Enter a character: ");
                    let stdin = io::stdin();
                    let c: char = stdin
                        .lock()
                        .lines()
                        .next()
                        .unwrap()
                        .unwrap()
                        .to_string()
                        .chars()
                        .next()
                        .unwrap();

                    let mut buf = [0];
                    c.encode_utf8(&mut buf); // this is ASCII and therefore one byte
                    self.stack.push(buf[0]);
                }
                Instruction::End => {
                    return Ok(false);
                }
                &Instruction::Unknown(c) => {
                    return Err(RuntimeError::InvalidInstruction(c));
                }
            }
        }

        self.step();
        Ok(true)
    }

    fn try_pop(&mut self) -> Result<u8, RuntimeError> {
        if let Some(a) = self.stack.pop() {
            Ok(a)
        } else {
            return Err(RuntimeError::EmptyStack);
        }
    }

    fn step(&mut self) {
        let (mut x, mut y) = self.position;
        match self.direction {
            Direction::Up => {
                if y == 0 {
                    y = FIELD_HEIGHT - 1;
                } else {
                    y = y - 1;
                }
            }
            Direction::Down => {
                if y == FIELD_HEIGHT - 1 {
                    y = 0;
                } else {
                    y = y + 1;
                }
            }
            Direction::Right => {
                if x == FIELD_WIDTH - 1 {
                    x = 0;
                } else {
                    x = x + 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    x = FIELD_WIDTH - 1;
                } else {
                    x = x - 1;
                }
            }
        }
        self.position = (x, y);
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum RuntimeError {
        EmptyStack {
            description("Trying to pop on an empty stack.")
        }
        InvalidInstruction(ins: char) {
            description("Trying to execute an unkown instruction.")
                display("Trying to execute an unkown instruction: '{}'.", ins)
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                let s = self.program.get((x, y)).to_char().to_string();
                if (x, y) == self.position {
                    write!(f, "{}", s.reversed())?;
                } else {
                    write!(f, "{}", s)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Direction: {:?}\nString Mode: {:?}",
            self.direction, self.string_mode
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
