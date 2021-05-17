use crate::op::Op;
use crate::value::Value;
use std::{u8, usize};

pub struct LineData {
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

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub values: Vec<Value>,
    pub lines: Vec<LineData>,
}

impl Chunk {
    pub fn of(f: fn(&mut Chunk) -> ()) -> Chunk {
        let mut new = Chunk::default();
        f(&mut new);
        new
    }

    #[inline]
    pub fn code_ptr(&self) -> *const u8 {
        return self.code.as_ptr();
    }

    #[inline]
    pub fn get_constant(&self, val_index: usize) -> &Value {
        return &self.values[val_index];
    }

    pub fn operation(&mut self, op: Op, line: u32) {
        op.write_to(&mut self.code);
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

    /// Store and add a retrieve instruction for a constant.
    /// Useful for early tests but I should nuke it some time.
    pub fn push_const(&mut self, value: Value, line: u32) {
        let val_index = self.values.len();
        self.values.push(value);
        self.operation(Op::Const(val_index), line);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {:^27} ==", name);

        let ops = Op::read_all(&self.code);
        let mut pos: usize = 0;

        for (op_index, op) in ops.iter().enumerate() {
            op.print(&self, op_index, pos);
            pos += op.cost();
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
