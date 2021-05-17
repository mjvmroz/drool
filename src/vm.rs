use crate::op::Op;

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

    pub fn print_stack(&self) {
        print!("          ");
        self.stack.iter().for_each(|v| {
            print!("[ ");
            v.print();
            print!(" ]");
        });
        println!();
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
                let op: Op;
                if cfg!(debug_assertions) {
                    if !self.stack.is_empty() {
                        self.print_stack();
                    }

                    let pos = (ip as usize) - (self.chunk.code_ptr() as usize);
                    op = Op::read_and_advance(&mut ip);
                    op.print(self.chunk, op_index, pos);
                    op_index += 1;
                } else {
                    op = Op::read_and_advance(&mut ip);
                }

                match op {
                    Op::Return => {
                        self.force_pop().print();
                        println!();
                        return InterpretResult::InterpretOk;
                    }
                    Op::ConstSmol(val_index) => {
                        let value = self.chunk.get_constant(val_index.into());
                        self.stack.push(*value);
                    }
                    Op::ConstThicc(val_index) => {
                        let value = self.chunk.get_constant(val_index.into());
                        self.stack.push(*value);
                    }
                    Op::Negate => {
                        self.op_unary_mut(Value::negate_mut);
                    }
                    Op::Add => self.op_binary_mut(Value::add_mut),
                    Op::Subtract => self.op_binary_mut(Value::subtract_mut),
                    Op::Multiply => self.op_binary_mut(Value::multiply_mut),
                    Op::Divide => self.op_binary_mut(Value::divide_mut),
                }
            }
        }
    }
}
