use instruction::Instruction;

pub struct Field {
    data: Vec<Instruction>,
}

pub const FIELD_WIDTH: usize = 80;
pub const FIELD_HEIGHT: usize = 25;
pub type Pos = (usize, usize);

impl Field {
    pub fn from_str(prog: &str) -> Field {
        let mut data = vec![Instruction::Noop; FIELD_WIDTH * FIELD_HEIGHT];
        for (y, line) in prog.split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                data[x + y * FIELD_WIDTH] = Instruction::from_char(c);
            }
        }
        Field { data }
    }
    pub fn get(&self, (x, y): Pos) -> &Instruction {
        &self.data[x + y * FIELD_WIDTH]
    }
    pub fn set(&mut self, (x, y): Pos, val: Instruction) {
        self.data[x + y * FIELD_WIDTH] = val;
    }


}

