use crate::{data::u24, value::Value};
use std::{convert::TryInto, u8, usize};

// The actual constant map, for use in the real, scary world.
#[non_exhaustive]
pub struct OpCode {}
impl OpCode {
    pub const RETURN: u8 = 0x00;
    pub const CONST_SMOL: u8 = 0x01;
    pub const CONST_THICC: u8 = 0x02;
    pub const NEGATE: u8 = 0x03;
    pub const ADD: u8 = 0x04;
    pub const SUBTRACT: u8 = 0x05;
    pub const MULTIPLY: u8 = 0x06;
    pub const DIVIDE: u8 = 0x07;
}

#[derive(Clone, Copy)]
// A friendly data representation for nicer assembly and disassembly (which comes at some cost).
pub enum Operation {
    Return,
    ConstantSmol(u8),
    ConstantThicc(u24),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Copy)]
struct Positioned<A> {
    val: A,
    pos: usize,
}

impl Operation {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        match *self {
            Self::Return => buffer.push(OpCode::RETURN),
            Self::ConstantSmol(i) => {
                buffer.push(OpCode::CONST_SMOL);
                buffer.push(i);
            }
            Self::ConstantThicc(i) => {
                buffer.push(OpCode::CONST_THICC);
                i.to_bytes().iter().for_each(|b| buffer.push(*b));
            }
            Self::Negate => buffer.push(OpCode::NEGATE),
            Self::Add => buffer.push(OpCode::ADD),
            Self::Subtract => buffer.push(OpCode::SUBTRACT),
            Self::Multiply => buffer.push(OpCode::MULTIPLY),
            Self::Divide => buffer.push(OpCode::DIVIDE),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Self::Return => 1,
            Self::ConstantSmol(_) => 2,
            Self::ConstantThicc(_) => 4,
            Self::Negate => 1,
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 1,
            Self::Divide => 1,
        }
    }

    fn read_at(buffer: &Vec<u8>, pos: usize) -> Operation {
        match buffer[pos] {
            OpCode::RETURN => Operation::Return,
            OpCode::CONST_SMOL => Operation::ConstantSmol(buffer[pos + 1]),
            OpCode::CONST_THICC => Operation::ConstantThicc(u24::from_buffer(buffer, pos + 1)),
            OpCode::NEGATE => Operation::Negate,
            OpCode::ADD => Operation::Add,
            OpCode::SUBTRACT => Operation::Subtract,
            OpCode::MULTIPLY => Operation::Multiply,
            OpCode::DIVIDE => Operation::Divide,
            _ => panic!("Corrupt bytecode"),
        }
    }

    fn read_all(buffer: &Vec<u8>) -> Vec<Positioned<Operation>> {
        let mut pos: usize = 0;
        let mut ops: Vec<Positioned<Operation>> = Vec::new();
        while pos < buffer.len() {
            let op = Operation::read_at(buffer, pos);
            ops.push(Positioned { val: op, pos });
            pos += op.cost();
        }
        return ops;
    }

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
            Self::Return => Operation::simple_instruction("OP_RETURN".to_string()),
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

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    // 1-1 mapping with line numbers.
    // TODO: Come up with a more efficient encoding.
    lines: Vec<LineData>,
}

impl Chunk {
    pub fn of(f: fn(&mut Chunk) -> ()) -> Chunk {
        let mut new = Chunk::default();
        f(&mut new);
        new
    }

    #[inline(always)]
    pub fn code_ptr(&self) -> *const u8 {
        return self.code.as_ptr();
    }

    #[inline(always)]
    pub fn get_constant(&self, index: usize) -> &Value {
        return &self.values[index];
    }

    pub fn operation(&mut self, op: Operation, line: u32) {
        op.write_to(&mut self.code);
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
        let op = Operation::read_at(&self.code, pos);
        // I really need to rethink this index stuff.
        // Alternatively, I could just disassemble the whole chunk once in the VM and follow along. 🤷‍♂️
        op.print(self, op_index, pos);
    }

    #[allow(dead_code)]
    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        let ops = Operation::read_all(&self.code);
        for (op_index, Positioned { val: op, pos }) in ops.iter().enumerate() {
            op.print(&self, op_index, *pos)
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
