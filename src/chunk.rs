use crate::{data::u24, value::Value};
use std::{convert::TryInto, mem, u8, usize};

#[derive(Clone, Copy)]
#[repr(u8, C)]
// A friendly data representation for nicer assembly and disassembly (which comes at some cost).
pub enum Operation {
    Return,             // 0x00
    ConstantSmol(u8),   // 0x01, 2
    ConstantThicc(u24), // 0x02, 4
    Negate,             // 0x03
    Add,                // 0x04
    Subtract,           // 0x05
    Multiply,           // 0x06
    Divide,             // 0x07
}

#[derive(Clone, Copy)]
struct Positioned<A> {
    val: A,
    pos: usize,
}

impl Operation {
    fn simple_instruction(name: String) {
        println!("{}", name);
    }

    fn constant_instruction(name: String, index: usize, value: &Value) {
        print!("{:<16} {:>4} ", name, index);
        value.print();
        println!();
    }

    fn print(&self, chunk: &Chunk, op_index: usize, pos: usize) {
        print!("{:0>4} ", pos);
        if op_index > 0 && chunk.get_line(op_index) == chunk.get_line(op_index - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", chunk.get_line(op_index));
        }
        match self {
            Self::Return => Self::simple_instruction("OP_RETURN".to_string()),
            Self::ConstantSmol(i) => {
                let index: usize = (*i).into();
                Self::constant_instruction("OP_CONST_SMOL".to_string(), index, &chunk.values[index])
            }
            Self::ConstantThicc(i) => {
                let index: usize = i.to_usize();
                Self::constant_instruction(
                    "OP_CONST_THICC".to_string(),
                    index,
                    &chunk.values[index],
                )
            }
            Self::Negate => Self::simple_instruction("OP_NEGATE".to_string()),
            Self::Add => Self::simple_instruction("OP_ADD".to_string()),
            Self::Subtract => Self::simple_instruction("OP_SUBTRACT".to_string()),
            Self::Multiply => Self::simple_instruction("OP_MULTIPLY".to_string()),
            Self::Divide => Self::simple_instruction("OP_DIVIDE".to_string()),
        }
    }
}

struct LineData {
    ops: usize,
    line: u32,
}

impl LineData {
    fn new(line: u32) -> LineData {
        LineData { ops: 1, line }
    }

    fn tick(&mut self) {
        self.ops += 1;
    }
}

pub struct Chunk {
    pub code: Vec<Operation>,
    values: Vec<Value>,
    // 1-1 mapping with line numbers.
    // TODO: Come up with a more efficient encoding.
    lines: Vec<LineData>,
}

impl Chunk {
    pub fn of(f: fn(&mut Chunk) -> ()) -> Chunk {
        let mut new = Chunk {
            code: vec![],
            values: vec![],
            lines: vec![],
        };
        f(&mut new);
        new
    }

    pub fn get_constant(&self, index: usize) -> &Value {
        return &self.values[index];
    }

    pub fn operation(&mut self, op: Operation, line: u32) {
        self.code.push(op);
        match self.lines.last_mut() {
            None => self.lines.push(LineData::new(line)),
            Some(last_line) => {
                if last_line.line == line {
                    last_line.tick();
                } else {
                    self.lines.push(LineData::new(line));
                }
            }
        };
    }

    pub fn constant(&mut self, value: Value, line: u32) {
        let val_index = self.values.len();
        self.values.push(value);
        if val_index <= 255 {
            self.operation(Operation::ConstantSmol(val_index.try_into().unwrap()), line);
        } else {
            self.operation(Operation::ConstantThicc(u24::from_usize(val_index)), line);
        }
    }

    pub fn disassemble_at(&self, op_index: usize, pos: usize) {
        let op = self.code.get(op_index).expect("Ruh roh.");
        // I really need to rethink this index stuff.
        // Alternatively, I could just disassemble the whole chunk once in the VM and follow along. 🤷‍♂️
        op.print(self, op_index, pos);
    }

    #[allow(dead_code)]
    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        let mut byte_pos: usize = 0;
        for (op_index, op) in self.code.iter().enumerate() {
            op.print(&self, op_index, byte_pos);
            byte_pos += mem::size_of_val(&op);
        }
    }

    pub fn get_line(&self, op_index: usize) -> u32 {
        let mut op_count = 0_usize;
        for LineData { ops, line } in &self.lines {
            op_count += *ops;
            if op_index < op_count {
                return *line;
            }
        }
        panic!("Corrupt line data");
    }
}
