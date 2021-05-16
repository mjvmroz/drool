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

impl Operation {
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
    pub fn print(&self, chunk: &Chunk, op_index: usize, mem_pos: usize) {
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
