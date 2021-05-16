use crate::{data::u24, operation::Operation, value::Value};
use std::{convert::TryInto, mem, usize};

#[derive(Clone, Copy)]
struct Positioned<A> {
    val: A,
    pos: usize,
}

struct LineData {
    ops: usize,
    line: u32,
}

impl LineData {
    fn new(line: u32) -> LineData {
        LineData { ops: 1, line }
    }

    fn tick(&mut self) {
        self.ops += 1;
    }
}

pub struct Chunk {
    pub code: Vec<Operation>,
    values: Vec<Value>,
    lines: Vec<LineData>,
}

impl Chunk {
    pub fn of(f: fn(&mut Chunk) -> ()) -> Chunk {
        let mut new = Chunk {
            code: vec![],
            values: vec![],
            lines: vec![],
        };
        f(&mut new);
        new
    }

    pub fn get_constant(&self, index: usize) -> &Value {
        return &self.values[index];
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

    pub fn disassemble_at(&self, op_index: usize, pos: usize) {
        let op = self.code.get(op_index).expect("Ruh roh.");
        op.print(self, op_index, pos);
    }

    #[allow(dead_code)]
    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        let mut byte_pos: usize = 0;
        for (op_index, op) in self.code.iter().enumerate() {
            op.print(&self, op_index, byte_pos);
            byte_pos += mem::size_of_val(&op);
        }
    }

    pub fn get_line(&self, op_index: usize) -> u32 {
        let mut op_count = 0_usize;
        for LineData { ops, line } in &self.lines {
            op_count += *ops;
            if op_index < op_count {
                return *line;
            }
        }
        panic!("Corrupt line data");
    }
}
