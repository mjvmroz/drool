use rawpointer::PointerExt;

use crate::{
    chunk::{bytecode_chunk::BytecodeChunk, Chunk},
    data::FromU24Bytes,
    operation::OpCode,
    value::Value,
    vm::InterpretResult,
};

use super::VM;

pub struct BytecodeVM<'a> {
    chunk: &'a BytecodeChunk,
    stack: Vec<Value>,
}

impl<'a> VM<'a, BytecodeChunk, Vec<u8>> for BytecodeVM<'a> {
    fn new(chunk: &'a BytecodeChunk) -> Self {
        Self {
            chunk,
            stack: Vec::new(),
        }
    }

    fn stack(&mut self) -> &mut Vec<Value> {
        &mut self.stack
    }

    fn run(&mut self) -> super::InterpretResult {
        let mut ip = self.chunk.code_ptr();
        let mut op_index: usize = 0;

        unsafe {
            loop {
                if cfg!(debug_assertions) {
                    print!("          ");

                    self.stack.iter().for_each(|v| {
                        print!("[ ");
                        v.print();
                        print!(" ]");
                    });
                    println!();
                    let pos = (ip as usize) - (self.chunk.code_ptr() as usize);
                    self.chunk.disassemble_at(op_index, pos);

                    op_index += 1;
                }

                let instruction = *ip.post_inc();
                match instruction {
                    OpCode::RETURN => {
                        self.force_pop().print();
                        println!();
                        return InterpretResult::InterpretOk;
                    }
                    OpCode::CONST_SMOL => {
                        let value = self.chunk.get_constant(*ip.post_inc() as usize);
                        self.stack.push(*value);
                    }
                    OpCode::CONST_THICC => {
                        self.chunk.get_constant(usize::from_u24_ptr(ip));
                        ip = ip.offset(3);
                    }
                    OpCode::NEGATE => {
                        self.force_peek_mut(Value::negate_mut);
                    }
                    OpCode::ADD => self.binary_op_mut(Value::add_mut),
                    OpCode::SUBTRACT => self.binary_op_mut(Value::subtract_mut),
                    OpCode::MULTIPLY => self.binary_op_mut(Value::multiply_mut),
                    OpCode::DIVIDE => self.binary_op_mut(Value::divide_mut),
                    _ => return InterpretResult::InterpretRuntimeError,
                }
            }
        }
    }
}
