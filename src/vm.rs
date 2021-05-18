use std::fmt::{self, Display, Formatter};

use crate::op::Op;

use crate::{chunk::Chunk, value::Value};

pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Stack,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuntimeError {
    StackUnderflow,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RuntimeError::StackUnderflow => f.write_str("stack underflow"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum InterpretError {
    Compile,
    Runtime(RuntimeError),
}

type InterpretResult<A> = Result<A, InterpretError>;

#[allow(dead_code)]
impl<'a> VM<'a> {
    pub fn new(chunk: &Chunk) -> VM {
        VM {
            chunk,
            stack: Stack::default(),
        }
    }

    pub fn interpret(source: &str) -> InterpretResult<()> {
        Ok(())
    }

    #[inline]
    unsafe fn force_last_mut(&mut self) -> InterpretResult<&mut Value> {
        self.stack
            .0
            .last_mut()
            .ok_or(InterpretError::Runtime(RuntimeError::StackUnderflow))
    }

    #[inline]
    unsafe fn op_binary(&mut self, op: fn(Value, Value) -> Value) -> InterpretResult<()> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        Ok(self.stack.0.push(op(a, b)))
    }

    #[inline]
    unsafe fn op_unary_mut(&mut self, op: fn(&mut Value) -> ()) -> InterpretResult<()> {
        Ok(op(self.force_last_mut()?))
    }

    #[inline]
    unsafe fn op_binary_mut(&mut self, op: fn(&mut Value, Value) -> ()) -> InterpretResult<()> {
        let b = self.stack.pop()?;
        let a = self.force_last_mut()?;
        Ok(op(a, b))
    }

    pub fn run(&mut self) -> InterpretResult<()> {
        let mut ip = self.chunk.code_ptr();
        let mut op_index: usize = 0;

        unsafe {
            loop {
                let op: Op;
                if cfg!(debug_assertions) {
                    if !self.stack.is_empty() {
                        println!("{}", self.stack);
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
                        println!("{}", self.stack.pop()?);
                        return Ok(());
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
                    Op::Add => self.op_binary_mut(Value::add_mut)?,
                    Op::Subtract => self.op_binary_mut(Value::subtract_mut)?,
                    Op::Multiply => self.op_binary_mut(Value::multiply_mut)?,
                    Op::Divide => self.op_binary_mut(Value::divide_mut)?,
                }
            }
        }
    }
}

#[derive(Default)]
struct Stack(Vec<Value>);
impl Stack {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> InterpretResult<Value> {
        self.0
            .pop()
            .ok_or(InterpretError::Runtime(RuntimeError::StackUnderflow))
    }
}
impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "          ")?;
        self.0.iter().map(|v| write!(f, "[ {} ]", v)).collect()
    }
}
