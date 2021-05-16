use crate::value::Value;

use self::line_data::LineData;

pub mod bytecode_chunk;
pub mod data_chunk;
pub mod line_data;

pub trait Chunk<'a, R> {
    fn code(&self) -> &R;

    fn constant_pool(&self) -> &Vec<Value>;

    fn lines(&'a self) -> &'a Vec<LineData>;

    fn get_line(&'a self, op_index: usize) -> u32 {
        let mut op_count = 0_usize;
        for LineData { ops, line } in Self::lines(self) {
            op_count += *ops;
            if op_index < op_count {
                return *line;
            }
        }
        panic!("Corrupt line data");
    }

    fn disassemble_at(&'a self, op_index: usize, pos: usize);

    fn get_constant(&self, index: usize) -> &Value {
        &self.constant_pool()[index]
    }
}
