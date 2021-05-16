use rawpointer::PointerExt;

use crate::{
    chunk::{Chunk, OpCode},
    data::u24,
    value::Value,
};

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    #[inline(always)]
    unsafe fn force_pop(&mut self) -> Value {
        self.stack
            .pop()
            .expect("We poppa de stack but de stacka empty 🧑‍🍳🤷‍♂️")
    }

    #[inline(always)]
    unsafe fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.force_pop();
        let a = self.force_pop();
        self.stack.push(op(a, b));
    }

    pub fn run(&mut self) -> InterpretResult {
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
                        println!();
                    }
                    OpCode::CONST_THICC => {
                        let index = u24::from_ptr(ip);
                        self.chunk.get_constant(index.to_usize());
                        ip = ip.offset(3);
                    }
                    OpCode::NEGATE => {
                        let new_value = self.force_pop().negate();
                        self.stack.push(new_value);
                    }
                    OpCode::ADD => self.binary_op(Value::add),
                    OpCode::SUBTRACT => self.binary_op(Value::subtract),
                    OpCode::MULTIPLY => self.binary_op(Value::multiply),
                    OpCode::DIVIDE => self.binary_op(Value::divide),
                    _ => panic!("Corrupt bytecode"),
                }
            }
        }
    }
}
