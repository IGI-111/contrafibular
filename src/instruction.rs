use std::char;

#[derive(Clone, Debug)]
pub enum Instruction {
    Noop,
    Push(u32),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Greater,
    Right,
    Left,
    Up,
    Down,
    Random,
    HorizontalIf,
    VerticalIf,
    StringMode,
    Dup,
    Swap,
    Pop,
    PopInt,
    PopChar,
    Bridge,
    Get,
    Put,
    PushInt,
    PushChar,
    End,
    Unknown(u32),
}

impl Instruction {
    pub fn from_char(c: char) -> Instruction {
        match c {
            n @ '0'...'9' => Instruction::Push(n as u32 - '0' as u32),
            ' ' => Instruction::Noop,
            '+' => Instruction::Add,
            '-' => Instruction::Subtract,
            '*' => Instruction::Multiply,
            '/' => Instruction::Divide,
            '%' => Instruction::Modulo,
            '!' => Instruction::Not,
            '`' => Instruction::Greater,
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '?' => Instruction::Random,
            '_' => Instruction::HorizontalIf,
            '|' => Instruction::VerticalIf,
            '"' => Instruction::StringMode,
            ':' => Instruction::Dup,
            '\\' => Instruction::Swap,
            '$' => Instruction::Pop,
            '.' => Instruction::PopInt,
            ',' => Instruction::PopChar,
            '#' => Instruction::Bridge,
            'g' => Instruction::Get,
            'p' => Instruction::Put,
            '&' => Instruction::PushInt,
            '~' => Instruction::PushChar,
            '@' => Instruction::End,
            c => Instruction::Unknown(c as u32),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match *self {
            Instruction::Push(n) => '0' as u32 + n,
            Instruction::Noop => ' ' as u32,
            Instruction::Add => '+' as u32,
            Instruction::Subtract => '-' as u32,
            Instruction::Multiply => '*' as u32,
            Instruction::Divide => '/' as u32,
            Instruction::Modulo => '%' as u32,
            Instruction::Not => '!' as u32,
            Instruction::Greater => '`' as u32,
            Instruction::Right => '>' as u32,
            Instruction::Left => '<' as u32,
            Instruction::Up => '^' as u32,
            Instruction::Down => 'v' as u32,
            Instruction::Random => '?' as u32,
            Instruction::HorizontalIf => '_' as u32,
            Instruction::VerticalIf => '|' as u32,
            Instruction::StringMode => '"' as u32,
            Instruction::Dup => ':' as u32,
            Instruction::Swap => '\\' as u32,
            Instruction::Pop => '$' as u32,
            Instruction::PopInt => '.' as u32,
            Instruction::PopChar => ',' as u32,
            Instruction::Bridge => '#' as u32,
            Instruction::Get => 'g' as u32,
            Instruction::Put => 'p' as u32,
            Instruction::PushInt => '&' as u32,
            Instruction::PushChar => '~' as u32,
            Instruction::End => '@' as u32,
            Instruction::Unknown(b) => b,
        }
    }

    pub fn from_u32(b: u32) -> Instruction {
        let ascii = if let Some(c) = char::from_u32(b) {
            c.is_ascii()
        } else {
            false
        };
        if ascii {
            match b as u8 {
                n @ b'0'...b'9' => Instruction::Push((n - b'0') as u32),
                b' ' => Instruction::Noop,
                b'+' => Instruction::Add,
                b'-' => Instruction::Subtract,
                b'*' => Instruction::Multiply,
                b'/' => Instruction::Divide,
                b'%' => Instruction::Modulo,
                b'!' => Instruction::Not,
                b'`' => Instruction::Greater,
                b'>' => Instruction::Right,
                b'<' => Instruction::Left,
                b'^' => Instruction::Up,
                b'v' => Instruction::Down,
                b'?' => Instruction::Random,
                b'_' => Instruction::HorizontalIf,
                b'|' => Instruction::VerticalIf,
                b'"' => Instruction::StringMode,
                b':' => Instruction::Dup,
                b'\\' => Instruction::Swap,
                b'$' => Instruction::Pop,
                b'.' => Instruction::PopInt,
                b',' => Instruction::PopChar,
                b'#' => Instruction::Bridge,
                b'g' => Instruction::Get,
                b'p' => Instruction::Put,
                b'&' => Instruction::PushInt,
                b'~' => Instruction::PushChar,
                b'@' => Instruction::End,
                b => Instruction::Unknown(b as u32),
            }
        } else {
            Instruction::Unknown(b)
        }
    }
}
