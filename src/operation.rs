use crate::{chunk::Chunk, data::u24, value::Value};

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

#[non_exhaustive]
// Used in the bytecode VM
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

pub fn get_op(buffer: &Vec<u8>, pos: usize) -> Operation {
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

pub fn put_op(buffer: &mut Vec<u8>, op: &Operation) {
    match op {
        Operation::Return => buffer.push(OpCode::RETURN),
        Operation::ConstantSmol(i) => {
            buffer.push(OpCode::CONST_SMOL);
            buffer.push(*i);
        }
        Operation::ConstantThicc(i) => {
            buffer.push(OpCode::CONST_THICC);
            i.to_bytes().iter().for_each(|b| buffer.push(*b));
        }
        Operation::Negate => buffer.push(OpCode::NEGATE),
        Operation::Add => buffer.push(OpCode::ADD),
        Operation::Subtract => buffer.push(OpCode::SUBTRACT),
        Operation::Multiply => buffer.push(OpCode::MULTIPLY),
        Operation::Divide => buffer.push(OpCode::DIVIDE),
    }
}

impl Operation {
    pub fn constant(index: usize) -> Operation {
        if index <= 0xFF {
            Self::ConstantSmol(index as u8)
        } else {
            Self::ConstantThicc(u24::from_usize(index))
        }
    }

    fn simple_instruction(name: String) {
        println!("{}", name);
    }

    fn constant_instruction(name: String, index: usize, value: &Value) {
        print!("{:<16} {:>4} ", name, index);
        value.print();
        println!();
    }

    /// Print an instruction, with extra information from its chunk
    /// index within it, and its position in memory (in bytes)
    pub fn print<'a, C, T>(&self, chunk: &'a C, op_index: usize, mem_pos: usize)
    where
        C: Chunk<'a, T>,
    {
        print!("{:0>4} ", mem_pos);
        if op_index > 0 && chunk.get_line(op_index) == chunk.get_line(op_index - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", chunk.get_line(op_index));
        }
        match self {
            Self::Return => Self::simple_instruction("OP_RETURN".to_string()),
            Self::ConstantSmol(i) => {
                let index: usize = (*i).into();
                Self::constant_instruction(
                    "OP_CONST_SMOL".to_string(),
                    index,
                    chunk.get_constant(index),
                )
            }
            Self::ConstantThicc(i) => {
                let index: usize = i.to_usize();
                Self::constant_instruction(
                    "OP_CONST_THICC".to_string(),
                    index,
                    chunk.get_constant(index),
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