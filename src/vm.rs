use rawpointer::PointerExt;

use crate::chunk::{Chunk, OpCode};

#[derive(Default)]
pub struct VM {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn interpret(&self, chunk: &Chunk) -> InterpretResult {
        let mut ip = chunk.code_ptr();

        unsafe {
            loop {
                let instruction = *ip.post_inc();
                match instruction {
                    OpCode::RETURN => return InterpretResult::InterpretOk,
                    OpCode::CONST_SMOL => {
                        let constant = chunk.get_constant(*ip.post_inc());
                        constant.print();
                        println!();
                    }
                    _ => panic!("Corrupt bytecode"),
                }
            }
        }
    }
}
