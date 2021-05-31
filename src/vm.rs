use std::fmt::{self, Display, Formatter};

use crate::compiler::Compiler;
use crate::{compiler::CompileError, op::Op};

use crate::{chunk::Chunk, value::Value};

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

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum InterpretError {
    Compile(CompileError),
    Runtime(RuntimeError),
}

pub struct VM {
    chunk: Chunk,
    stack: Stack,
}

pub type InterpretResult<'s, A> = Result<A, InterpretError>;

pub type RunResult<A> = Result<A, RuntimeError>;

#[allow(dead_code)]
impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Stack::default(),
        }
    }

    pub fn interpret<'s>(&mut self, src: &'s str) -> InterpretResult<'s, ()> {
        let chunk = Compiler::compile(&src).map_err(InterpretError::Compile)?;

        self.chunk = chunk;
        self.run().map_err(InterpretError::Runtime)
    }

    #[inline]
    fn op_unary_mut(&mut self, op: fn(&mut Value) -> ()) -> RunResult<()> {
        Ok(op(self.stack.peek_mut()?))
    }

    #[inline]
    fn op_binary_mut(&mut self, op: fn(&mut Value, Value) -> ()) -> RunResult<()> {
        let b = self.stack.pop()?;
        let a = self.stack.peek_mut()?;
        Ok(op(a, b))
    }

    pub fn run(&mut self) -> RunResult<()> {
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
                    op.print(&self.chunk, op_index, pos);
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
                    Op::Negate => self.op_unary_mut(Value::negate_mut)?,
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
    pub fn pop(&mut self) -> RunResult<Value> {
        self.0.pop().ok_or(RuntimeError::StackUnderflow)
    }

    #[inline]
    fn peek_mut(&mut self) -> RunResult<&mut Value> {
        self.0.last_mut().ok_or(RuntimeError::StackUnderflow)
    }
}
impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "          ")?;
        self.0.iter().map(|v| write!(f, "[ {} ]", v)).collect()
    }
}
