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
#[rustfmt::skip]
impl OpCode {
    pub const RETURN: u8       = 0x00;
    pub const CONST_SMOL: u8   = 0x01;
    pub const CONST_THICC: u8  = 0x02;
    pub const NEGATE: u8       = 0x03;
    pub const ADD: u8          = 0x04;
    pub const SUBTRACT: u8     = 0x05;
    pub const MULTIPLY: u8     = 0x06;
    pub const DIVIDE: u8       = 0x07;
    pub const NIL: u8          = 0x08;
    pub const TRUE: u8         = 0x09;
    pub const FALSE: u8        = 0x0A;
    pub const NOT: u8          = 0x0B;
    pub const EQUAL: u8        = 0x0C;
    pub const GREATER: u8      = 0x0D;
    pub const LESS: u8         = 0x0E;
    pub const PRINT: u8        = 0x0F;
    pub const POP: u8          = 0x10;
}

#[derive(Debug, Eq, PartialEq)]
// I need Clone for prop testing, but I don't want to accidentally
// clone `Op`s in production code, since I might introduce
// performance regressions.
#[cfg_attr(test, derive(Clone))]

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
    Nil,             // 0x08
    True,            // 0x09
    False,           // 0x0A
    Not,             // 0x0B
    Equal,           // 0x0C
    Greater,         // 0x0D
    Less,            // 0x0E
    Print,           // 0x0F
    Pop,             // 0x10
}

impl Op {
    #[inline]
    /// Given a `(mut ptr) -> (ptr) -> Bytecode`, read the bytecode
    /// into an `Op`, then advance the pointer.
    /// Inlined, as it's used in the VM's critical path.
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
            OpCode::NIL => Op::Nil,
            OpCode::TRUE => Op::True,
            OpCode::FALSE => Op::False,
            OpCode::NOT => Op::Not,
            OpCode::EQUAL => Op::Equal,
            OpCode::GREATER => Op::Greater,
            OpCode::LESS => Op::Less,
            OpCode::PRINT => Op::Print,
            OpCode::POP => Op::Pop,
            _ => panic!("Corrupt bytecode"),
        };
        *ptr = ptr.add(op.cost());
        op
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
            Op::Nil => buffer.push(OpCode::NIL),
            Op::True => buffer.push(OpCode::TRUE),
            Op::False => buffer.push(OpCode::FALSE),
            Op::Not => buffer.push(OpCode::NOT),
            Op::Equal => buffer.push(OpCode::EQUAL),
            Op::Greater => buffer.push(OpCode::GREATER),
            Op::Less => buffer.push(OpCode::LESS),
            Op::Print => buffer.push(OpCode::PRINT),
            Op::Pop => buffer.push(OpCode::POP),
        }
    }

    #[inline]
    /// Inlined, as it's used in the VM's critical path.
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
            Op::Nil => 1,
            Op::True => 1,
            Op::False => 1,
            Op::Not => 1,
            Op::Equal => 1,
            Op::Greater => 1,
            Op::Less => 1,
            Op::Print => 1,
            Op::Pop => 1,
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
            Op::Nil => "OP_NIL",
            Op::True => "OP_TRUE",
            Op::False => "OP_FALSE",
            Op::Not => "OP_NOT",
            Op::Equal => "OP_EQUAL",
            Op::Greater => "OP_GREATER",
            Op::Less => "OP_LESS",
            Op::Print => "OP_PRINT",
            Op::Pop => "OP_POP",
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
                let val_index: usize = (*i).into();
                self.constant_instruction(val_index, chunk.get_constant(val_index))
            }
            Self::ConstThicc(i) => {
                let val_index: usize = i.to_usize();
                self.constant_instruction(val_index, chunk.get_constant(val_index))
            }
            Self::Negate => self.simple_instruction(),
            Self::Add => self.simple_instruction(),
            Self::Subtract => self.simple_instruction(),
            Self::Multiply => self.simple_instruction(),
            Self::Divide => self.simple_instruction(),
            Self::Nil => self.simple_instruction(),
            Self::True => self.simple_instruction(),
            Self::False => self.simple_instruction(),
            Self::Not => self.simple_instruction(),
            Self::Equal => self.simple_instruction(),
            Self::Greater => self.simple_instruction(),
            Self::Less => self.simple_instruction(),
            Self::Print => self.simple_instruction(),
            Self::Pop => self.simple_instruction(),
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
    /// Get an operation to access a constant. Exists since the actual instruction will
    /// vary based on `val_index`.
    pub fn Const(val_index: usize) -> Op {
        val_index
            .try_into()
            .map_or(Self::ConstThicc(u24::from(val_index)), Self::ConstSmol)
    }

    pub fn read_at_pos(buffer: &Vec<u8>, pos: usize) -> Op {
        unsafe { Self::read_and_advance(&mut buffer.as_ptr().add(pos)) }
    }

    pub fn read_all(buffer: &Vec<u8>) -> Vec<Op> {
        let mut pos: usize = 0;
        let mut ops: Vec<Op> = Vec::new();
        // TODO: figure out stateful iterators
        while pos < buffer.len() {
            let op = Op::read_at_pos(buffer, pos);
            pos += op.cost();
            ops.push(op);
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::Op;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Op {
        fn arbitrary<G>(g: &mut G) -> Self
        where
            G: Gen,
        {
            let n = g.next_u32() & 0x10;
            match n {
                0x00 => Op::Return,
                0x01 => {
                    let v = g.next_u32() & 0xFF;
                    Op::ConstSmol(v.try_into().unwrap())
                }
                0x02 => {
                    let v = g.next_u32() & 0xFF_FF_FF;
                    Op::ConstThicc(v.try_into().unwrap())
                }
                0x03 => Op::Negate,
                0x04 => Op::Add,
                0x05 => Op::Subtract,
                0x06 => Op::Multiply,
                0x07 => Op::Divide,
                0x08 => Op::Nil,
                0x09 => Op::True,
                0x0A => Op::False,
                0x0B => Op::Not,
                0x0C => Op::Equal,
                0x0D => Op::Greater,
                0x0E => Op::Less,
                0x0F => Op::Print,
                0x10 => Op::Pop,
                _ => {
                    panic!("Did you mask correctly? I'm guessing you didn't mask correctly. :bonk:")
                }
            }
        }
    }

    #[quickcheck]
    fn op_codec(ops: Vec<Op>) {
        let mut bytecode: Vec<u8> = vec![];
        ops.iter().for_each(|op| op.write_to(&mut bytecode));
        let decoded_ops = Op::read_all(&bytecode);

        assert_eq!(ops, decoded_ops);
    }
}
