use std::fmt::{self, Display, Formatter};

use broom::Heap;

use crate::compiler::Compiler;
use crate::value::Object;
use crate::value::TypeError;
use crate::value::TypeResult;
use crate::{compiler::CompileError, op::Op};

use crate::{chunk::Chunk, value::Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuntimeError {
    StackUnderflow,
    Type(TypeError),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RuntimeError::StackUnderflow => write!(f, "StackUnderflow"),
            RuntimeError::Type(e) => write!(f, "TypeError: {}", e),
        }
    }
}

impl From<TypeError> for RuntimeError {
    fn from(e: TypeError) -> Self {
        Self::Type(e)
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
    heap: Heap<Object>,
    stack: Stack,
}

pub type InterpretResult<'s, A> = Result<A, InterpretError>;

pub type RunResult<A> = Result<A, RuntimeError>;

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Stack::default(),
            heap: Heap::new(),
        }
    }

    pub fn interpret<'s>(&mut self, src: &'s str) -> InterpretResult<'s, ()> {
        let (chunk, heap) = Compiler::compile(src).map_err(InterpretError::Compile)?;

        self.chunk = chunk;
        self.heap = heap;
        self.run().map_err(InterpretError::Runtime)
    }

    #[inline]
    fn eff(&mut self, op: fn(&mut Heap<Object>, Value) -> TypeResult<()>) -> RunResult<()> {
        let val = self.stack.pop()?;
        op(&mut self.heap, val)?;
        Ok(())
    }

    #[inline]
    fn op_unary(&mut self, op: fn(&mut Heap<Object>, Value) -> TypeResult<Value>) -> RunResult<()> {
        let top = self.stack.peek()?;
        let res = op(&mut self.heap, *top)?;
        self.stack.pop()?;
        self.stack.push(res);
        Ok(())
    }

    #[inline]
    fn op_binary(
        &mut self,
        op: fn(&mut Heap<Object>, Value, Value) -> TypeResult<Value>,
    ) -> RunResult<()> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        let res = op(&mut self.heap, a, b).map_err(TypeError::into);

        match res {
            Ok(v) => {
                self.stack.push(v);
                Ok(())
            },
            Err(e) => {
                self.stack.push(a);
                self.stack.push(b);
                Err(e)
            }
        }
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
                    Op::Negate => self.op_unary(Value::negate)?,
                    Op::Add => self.op_binary(Value::add)?,
                    Op::Subtract => self.op_binary(Value::subtract)?,
                    Op::Multiply => self.op_binary(Value::multiply)?,
                    Op::Divide => self.op_binary(Value::divide)?,
                    Op::Nil => self.stack.push(Value::Nil),
                    Op::True => self.stack.push(Value::Bool(true)),
                    Op::False => self.stack.push(Value::Bool(false)),
                    Op::Not => self.op_unary(Value::not)?,
                    Op::Equal => self.op_binary(Value::equal)?,
                    Op::Greater => self.op_binary(Value::greater)?,
                    Op::Less => self.op_binary(Value::less)?,
                    Op::Print => self.eff(|_heap, val| Ok(println!("{}", val)))?,
                    Op::Pop => {
                        self.stack.pop()?;
                    }
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
    fn peek(&self) -> RunResult<&Value> {
        self.0.last().ok_or(RuntimeError::StackUnderflow)
    }
}
impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "          ")?;
        self.0.iter().try_for_each(|v| write!(f, "[ {} ]", v))
    }
}
