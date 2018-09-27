use instruction::Instruction;
use itertools::Itertools;

pub struct Field {
    data: Vec<Instruction>,
    width: usize,
    height: usize,
}

const DEFAULT_FIELD_WIDTH: usize = 80;
const DEFAULT_FIELD_HEIGHT: usize = 25;
pub type Pos = (usize, usize);

impl Field {
    pub fn from_bin(prog: &Vec<u8>) -> Field {
        // cleanup CRLF
        let prog = prog.iter().tuple_windows().filter_map(|(a, b)| {
            if *a == b'\r' && *b == b'\n' {
                None
            } else {
                Some(a)
            }
        }).cloned().collect::<Vec<u8>>();

        let mut data = vec![Instruction::Noop; DEFAULT_FIELD_WIDTH * DEFAULT_FIELD_HEIGHT];
        for (y, line) in prog.split(|&b| b == b'\n').enumerate() {
            for (x, &b) in line.iter().enumerate() {
                data[x + y * DEFAULT_FIELD_WIDTH] = Instruction::from_u8(b);
            }
        }
        Field {
            data,
            width: DEFAULT_FIELD_WIDTH,
            height: DEFAULT_FIELD_HEIGHT,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, (x, y): Pos) -> &Instruction {
        &self.data[x + y * self.width]
    }
    pub fn set(&mut self, (x, y): Pos, val: Instruction) {
        self.data[x + y * self.width] = val;
    }
}
