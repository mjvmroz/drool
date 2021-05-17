use crate::chunk::Chunk;
use crate::value::Value;

pub mod bytecode_vm;
pub mod data_vm;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub trait VM<'a, C, R>
where
    C: Chunk<'a, R>,
{
    fn new(chunk: &'a C) -> Self;

    fn stack(&mut self) -> &mut Vec<Value>;

    fn run(&mut self) -> InterpretResult;

    unsafe fn force_pop(&mut self) -> Value {
        self.stack()
            .pop()
            .expect("We poppa de stack but de stacka empty 🧑‍🍳🤷‍♂️")
    }

    unsafe fn force_peek_mut<F>(&mut self, op: F)
    where
        F: Fn(&mut Value) -> (),
    {
        op(self
            .stack()
            .last_mut()
            .expect("Peeka beep boop no bueno 🙅‍♂️"));
    }

    unsafe fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.force_pop();
        let a = self.force_pop();
        self.stack().push(op(a, b));
    }

    unsafe fn binary_op_mut(&mut self, op: fn(&mut Value, Value) -> ()) {
        let b = self.force_pop();
        let a = self.force_peek_mut(|a| op(a, b));
    }
}
