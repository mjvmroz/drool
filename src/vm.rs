use crate::{data::FromU24Bytes, op::OpCode};
use rawpointer::PointerExt;

use crate::{chunk::Chunk, value::Value};

pub struct VM<'a> {
    chunk: &'a Chunk,
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
    pub fn new(chunk: &Chunk) -> VM {
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
    unsafe fn force_last_mut(&mut self) -> &mut Value {
        self.stack.last_mut().expect("Peeka beep boop no bueno 🙅‍♂️")
    }

    #[inline(always)]
    unsafe fn op_binary(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.force_pop();
        let a = self.force_pop();
        self.stack.push(op(a, b));
    }

    #[inline(always)]
    unsafe fn op_unary_mut(&mut self, op: fn(&mut Value) -> ()) {
        op(self.force_last_mut());
    }

    #[inline(always)]
    unsafe fn op_binary_mut(&mut self, op: fn(&mut Value, Value) -> ()) {
        let b = self.force_pop();
        let a = self.force_last_mut();
        op(a, b);
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
                    }
                    OpCode::CONST_THICC => {
                        self.chunk.get_constant(usize::from_u24_ptr(ip));
                        ip = ip.offset(3);
                    }
                    OpCode::NEGATE => {
                        self.op_unary_mut(Value::negate_mut);
                    }
                    OpCode::ADD => self.op_binary_mut(Value::add_mut),
                    OpCode::SUBTRACT => self.op_binary_mut(Value::subtract_mut),
                    OpCode::MULTIPLY => self.op_binary_mut(Value::multiply_mut),
                    OpCode::DIVIDE => self.op_binary_mut(Value::divide_mut),
                    _ => return InterpretResult::InterpretRuntimeError,
                }
            }
        }
    }
}
