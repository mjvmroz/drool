use crate::{chunk::Chunk, data::FromU24Bytes};
use crate::{data::u24, value::Value};
use std::{convert::TryInto, u8, usize};

// The actual constant map, for use in the real, scary world.
// Rust doesn't yet me let associate these with the actual enum entries.
// There's no sugar for it, and mem::discriminant can't give me a usize
// at const time. :(
// So I'm doing it manually. Yay.
#[non_exhaustive]
pub struct OpCode {}
#[cfg_attr(rustfmt, rustfmt_skip)]
impl OpCode {
    pub const RETURN: u8       = 0x00;
    pub const CONST_SMOL: u8   = 0x01;
    pub const CONST_THICC: u8  = 0x02;
    pub const NEGATE: u8       = 0x03;
    pub const ADD: u8          = 0x04;
    pub const SUBTRACT: u8     = 0x05;
    pub const MULTIPLY: u8     = 0x06;
    pub const DIVIDE: u8       = 0x07;
}

// A friendly data representation for nicer assembly and disassembly (which comes at some cost).
pub enum Op {
    Return,          // 0x00
    ConstSmol(u8),   // 0x01, 2
    ConstThicc(u24), // 0x02, 4
    Negate,          // 0x03
    Add,             // 0x04
    Subtract,        // 0x05
    Multiply,        // 0x06
    Divide,          // 0x07
}

impl Op {
    unsafe fn read_at_ptr(ptr: &mut *const u8) -> Op {
        let op = match **ptr {
            OpCode::RETURN => Op::Return,
            OpCode::CONST_SMOL => Op::ConstSmol(*ptr.add(1)),
            OpCode::CONST_THICC => Op::ConstThicc(u24::from_u24_ptr(*ptr)),
            OpCode::NEGATE => Op::Negate,
            OpCode::ADD => Op::Add,
            OpCode::SUBTRACT => Op::Subtract,
            OpCode::MULTIPLY => Op::Multiply,
            OpCode::DIVIDE => Op::Divide,
            _ => panic!("Corrupt bytecode"),
        };
        // DANGER
        *ptr = ptr.add(op.cost());
        return op;
    }

    pub fn cost(&self) -> usize {
        match self {
            Self::Return => 1,
            Self::ConstSmol(_) => 2,
            Self::ConstThicc(_) => 4,
            Self::Negate => 1,
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 1,
            Self::Divide => 1,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Return => "OP_RETURN",
            Self::ConstSmol(_) => "OP_CONST_SMOL",
            Self::ConstThicc(_) => "OP_CONST_THICC",
            Self::Negate => "OP_NEGATE",
            Self::Add => "OP_ADD",
            Self::Subtract => "OP_SUBTRACT",
            Self::Multiply => "OP_MULTIPLY",
            Self::Divide => "OP_DIVIDE",
        }
    }

    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        match *self {
            Self::Return => buffer.push(OpCode::RETURN),
            Self::ConstSmol(i) => {
                buffer.push(OpCode::CONST_SMOL);
                buffer.push(i);
            }
            Self::ConstThicc(i) => {
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

    #[allow(non_snake_case, dead_code)]
    pub fn Const(val_index: usize) -> Op {
        if val_index <= 0xFF {
            Self::ConstSmol(val_index.try_into().unwrap())
        } else {
            Self::ConstThicc(u24::from(val_index))
        }
    }

    pub fn read_at_pos(buffer: &Vec<u8>, pos: usize) -> Op {
        unsafe { Self::read_at_ptr(&mut buffer.as_ptr().add(pos)) }
    }

    pub fn read_all(buffer: &Vec<u8>) -> Vec<Op> {
        let mut pos: usize = 0;
        let mut ops: Vec<Op> = Vec::new();
        while pos < buffer.len() {
            let op = Op::read_at_pos(buffer, pos);
            pos += op.cost();
            ops.push(op);
        }
        return ops;
    }

    fn simple_instruction(&self) {
        println!("{}", self.name());
    }

    fn constant_instruction(&self, index: usize, value: &Value) {
        print!("{:<16} {:>4} ", self.name(), index);
        value.print();
        println!();
    }

    pub fn print(&self, chunk: &Chunk, op_index: usize, pos: usize) {
        print!("{:0>4} ", pos);
        if op_index > 0 && chunk.get_line(op_index) == chunk.get_line(op_index - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", chunk.get_line(op_index));
        }
        match self {
            Self::Return => self.simple_instruction(),
            Self::ConstSmol(i) => {
                let index: usize = (*i).into();
                self.constant_instruction(index, &chunk.values[index])
            }
            Self::ConstThicc(i) => {
                let index: usize = i.to_usize();
                self.constant_instruction(index, &chunk.values[index])
            }
            Self::Negate => self.simple_instruction(),
            Self::Add => self.simple_instruction(),
            Self::Subtract => self.simple_instruction(),
            Self::Multiply => self.simple_instruction(),
            Self::Divide => self.simple_instruction(),
        }
    }
}
