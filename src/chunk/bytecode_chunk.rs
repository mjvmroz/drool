use crate::value::Value;

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

impl From<DataChunk> for BytecodeChunk {
    fn from(data_chunk: DataChunk) -> Self {
        BytecodeChunk {
            code: vec![],
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

    fn op_at(&self, op_index: usize) -> &crate::operation::Operation {
        todo!()
    }
}
