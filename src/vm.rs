use std::usize;

use crate::{chunk::Chunk, operation::Operation as Op};

use crate::{chunk::data_chunk::DataChunk, value::Value};

pub struct VM<'a> {
    chunk: &'a DataChunk,
    stack: Vec<Value>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

#[allow(dead_code)]
impl<'a> VM<'a> {
    pub fn new(chunk: &DataChunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    unsafe fn force_pop(&mut self) -> Value {
        self.stack
            .pop()
            .expect("We poppa de stack but de stacka empty 🧑‍🍳🤷‍♂️")
    }

    unsafe fn force_last_mut(&mut self) -> &mut Value {
        self.stack.last_mut().expect("Peeka beep boop no bueno 🙅‍♂️")
    }

    unsafe fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.force_pop();
        let a = self.force_pop();
        self.stack.push(op(a, b));
    }

    unsafe fn binary_op_mut(&mut self, op: fn(&mut Value, Value) -> ()) {
        let b = self.force_pop();
        let a = self.force_last_mut();
        op(a, b);
    }

    pub fn run(&mut self) -> InterpretResult {
        unsafe {
            for (op_index, op) in self.chunk.code.iter().enumerate() {
                if cfg!(debug_assertions) {
                    print!("          ");

                    self.stack.iter().for_each(|v| {
                        print!("[ ");
                        v.print();
                        print!(" ]");
                    });
                    println!();
                    let pos = std::mem::size_of_val(&self.chunk.code[0..op_index]);
                    self.chunk.disassemble_at(op_index, pos);
                }

                match op {
                    Op::Return => {
                        self.force_pop().print();
                        println!();
                        return InterpretResult::InterpretOk;
                    }
                    Op::ConstantSmol(i) => {
                        let value = self.chunk.get_constant(*i as usize);
                        self.stack.push(*value);
                    }
                    Op::ConstantThicc(i) => {
                        self.chunk.get_constant(i.to_usize());
                    }
                    Op::Negate => {
                        self.force_last_mut().negate_mut();
                    }
                    Op::Add => self.binary_op_mut(Value::add_mut),
                    Op::Subtract => self.binary_op_mut(Value::subtract_mut),
                    Op::Multiply => self.binary_op_mut(Value::multiply_mut),
                    Op::Divide => self.binary_op_mut(Value::divide_mut),
                }
            }
        }
        return InterpretResult::InterpretRuntimeError;
    }
}
