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

pub enum Op {
    //               // CODE, COST
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
    #[inline]
    pub unsafe fn read_and_advance(ptr: &mut *const u8) -> Op {
        let op = match **ptr {
            OpCode::RETURN => Op::Return,
            OpCode::CONST_SMOL => Op::ConstSmol(*ptr.add(1)),
            OpCode::CONST_THICC => Op::ConstThicc(u24::from_u8_ptr(ptr.add(1))),
            OpCode::NEGATE => Op::Negate,
            OpCode::ADD => Op::Add,
            OpCode::SUBTRACT => Op::Subtract,
            OpCode::MULTIPLY => Op::Multiply,
            OpCode::DIVIDE => Op::Divide,
            _ => panic!("Corrupt bytecode"),
        };
        *ptr = ptr.add(op.cost());
        return op;
    }

    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        match *self {
            Op::Return => buffer.push(OpCode::RETURN),
            Op::ConstSmol(i) => {
                buffer.push(OpCode::CONST_SMOL);
                buffer.push(i);
            }
            Op::ConstThicc(i) => {
                buffer.push(OpCode::CONST_THICC);
                i.to_bytes().iter().for_each(|b| buffer.push(*b));
            }
            Op::Negate => buffer.push(OpCode::NEGATE),
            Op::Add => buffer.push(OpCode::ADD),
            Op::Subtract => buffer.push(OpCode::SUBTRACT),
            Op::Multiply => buffer.push(OpCode::MULTIPLY),
            Op::Divide => buffer.push(OpCode::DIVIDE),
        }
    }

    pub fn cost(&self) -> usize {
        match self {
            Op::Return => 1,
            Op::ConstSmol(_) => 2,
            Op::ConstThicc(_) => 4,
            Op::Negate => 1,
            Op::Add => 1,
            Op::Subtract => 1,
            Op::Multiply => 1,
            Op::Divide => 1,
        }
    }

    fn name(&self) -> &str {
        match self {
            Op::Return => "OP_RETURN",
            Op::ConstSmol(_) => "OP_CONST_SMOL",
            Op::ConstThicc(_) => "OP_CONST_THICC",
            Op::Negate => "OP_NEGATE",
            Op::Add => "OP_ADD",
            Op::Subtract => "OP_SUBTRACT",
            Op::Multiply => "OP_MULTIPLY",
            Op::Divide => "OP_DIVIDE",
        }
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

    fn simple_instruction(&self) {
        println!("{}", self.name());
    }

    fn constant_instruction(&self, index: usize, value: &Value) {
        print!("{:<16} {:>4} {}", self.name(), index, value);
        println!();
    }

    #[allow(non_snake_case)]
    pub fn Const(val_index: usize) -> Op {
        if val_index <= 0xFF {
            Self::ConstSmol(val_index.try_into().unwrap())
        } else {
            Self::ConstThicc(u24::from(val_index))
        }
    }

    pub fn read_at_pos(buffer: &Vec<u8>, pos: usize) -> Op {
        unsafe { Self::read_and_advance(&mut buffer.as_ptr().add(pos)) }
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
}
