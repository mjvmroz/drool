use crate::{data::u24, operation::Operation, value::Value};
use std::{convert::TryInto, mem, usize};

use super::{line_data::LineData, Chunk};

#[derive(Clone, Default)]
pub struct DataChunk {
    pub code: Vec<Operation>,
    values: Vec<Value>,
    lines: Vec<LineData>,
}

impl DataChunk {
    pub fn of(f: fn(&mut DataChunk) -> ()) -> DataChunk {
        let mut new = DataChunk::default();
        f(&mut new);
        new
    }

    pub fn operation(&mut self, op: Operation, line: u32) {
        self.code.push(op);
        match self.lines.last_mut() {
            None => self.lines.push(LineData::new(line)),
            Some(last_line) => {
                if last_line.line == line {
                    last_line.tick();
                } else {
                    self.lines.push(LineData::new(line));
                }
            }
        };
    }

    pub fn constant(&mut self, value: Value, line: u32) {
        let val_index = self.values.len();
        self.values.push(value);
        if val_index <= 255 {
            self.operation(Operation::ConstantSmol(val_index.try_into().unwrap()), line);
        } else {
            self.operation(Operation::ConstantThicc(u24::from_usize(val_index)), line);
        }
    }

    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        let mut byte_pos: usize = 0;
        for (op_index, op) in self.code.iter().enumerate() {
            op.print(self, op_index, byte_pos);
            byte_pos += mem::size_of_val(&op);
        }
    }
}

impl<'a> Chunk<'a, Vec<Operation>> for DataChunk {
    fn code(&self) -> &Vec<Operation> {
        &self.code
    }

    fn constant_pool(&self) -> &Vec<Value> {
        &self.values
    }

    fn lines(&'a self) -> &'a Vec<LineData> {
        &self.lines
    }

    fn op_at(&self, op_index: usize) -> &Operation {
        &self.code[op_index]
    }

    fn disassemble_at(&self, op_index: usize, pos: usize) {
        let op = self.op_at(op_index);
        op.print(self, op_index, pos);
    }
}
