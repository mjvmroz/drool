use crate::{
    operation::{get_op, put_op},
    value::Value,
};

use super::{data_chunk::DataChunk, line_data::LineData, Chunk};

#[derive(Clone, Copy)]
struct Positioned<A> {
    val: A,
    pos: usize,
}

pub struct BytecodeChunk {
    pub code: Vec<u8>,
    values: Vec<Value>,
    lines: Vec<LineData>,
}

impl BytecodeChunk {
    pub fn code_ptr(&self) -> *const u8 {
        self.code.as_ptr()
    }
}

impl From<DataChunk> for BytecodeChunk {
    fn from(data_chunk: DataChunk) -> Self {
        let mut buffer = vec![];
        data_chunk
            .code()
            .iter()
            .for_each(|op| put_op(&mut buffer, op));
        BytecodeChunk {
            code: buffer,
            values: data_chunk.constant_pool().clone(),
            lines: data_chunk.lines().clone(),
        }
    }
}

impl<'a> Chunk<'a, Vec<u8>> for BytecodeChunk {
    fn code(&self) -> &Vec<u8> {
        &self.code
    }

    fn constant_pool(&self) -> &Vec<Value> {
        &self.values
    }

    fn lines(&'a self) -> &'a Vec<LineData> {
        &self.lines
    }

    fn disassemble_at(&'a self, op_index: usize, pos: usize) {
        get_op(&self.code, pos).print(self, op_index, pos)
    }
}
