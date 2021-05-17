use crate::{
    chunk::{data_chunk::DataChunk, Chunk},
    operation::Operation as Op,
    value::Value,
};

use super::{InterpretResult, VM};

pub struct DataVM<'a> {
    chunk: &'a DataChunk,
    stack: Vec<Value>,
}

impl<'a> VM<'a, DataChunk, Vec<Op>> for DataVM<'a> {
    fn new(chunk: &'a DataChunk) -> Self {
        DataVM {
            chunk,
            stack: vec![],
        }
    }

    fn stack(&mut self) -> &mut Vec<Value> {
        &mut self.stack
    }

    fn run(&mut self) -> InterpretResult {
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
                        self.force_peek_mut(Value::negate_mut);
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
