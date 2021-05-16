use crate::{u24::u24, value::Value};
use std::{convert::TryInto, u8, usize};

pub enum Operation {
    Return,
    ConstantSmol(u8),
    ConstantThicc(u24),
}

struct Positioned<A> {
    val: A,
    pos: usize,
}

impl Operation {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        match *self {
            Operation::Return => buffer.push(0x00),
            Operation::ConstantSmol(i) => {
                buffer.push(0x01);
                buffer.push(i);
            }
            Operation::ConstantThicc(i) => {
                buffer.push(0x02);
                i.to_bytes().iter().for_each(|b| buffer.push(*b));
            }
        }
    }

    fn read_all(buffer: &Vec<u8>) -> Vec<Positioned<Operation>> {
        let mut cur: usize = 0;
        let mut ops: Vec<Positioned<Operation>> = Vec::new();
        while cur < buffer.len() {
            let mut advance: usize = 1;
            let op = match buffer[cur] {
                0x00 => Operation::Return,
                0x01 => {
                    advance = 2;
                    Operation::ConstantSmol(buffer[cur + 1])
                }
                0x02 => {
                    advance = 4;
                    Operation::ConstantThicc(u24::from_bytes(
                        buffer[cur + 1..=cur + 3].try_into().expect("Are you bad at math or something? This slice should have THREE things 🙄"),
                    ))
                }
                _ => panic!("Corrupt bytecode"),
            };

            ops.push(Positioned { val: op, pos: cur });
            cur += advance;
        }
        return ops;
    }

    fn simple_instruction(name: String) {
        println!("{}", name);
    }

    fn constant_instruction(name: String, index: usize, value: &Value) {
        print!("{:<16} {:>4} ", name, index);
        value.print();
        println!();
    }

    fn print(&self, chunk: &Chunk, op_index: usize, pos: usize) {
        print!("{:0>4} ", pos);
        if op_index > 0 && chunk.get_line(op_index) == chunk.get_line(op_index - 1) {
            print!("   | ");
        } else {
            print!("{:>4} ", chunk.get_line(op_index));
        }
        match self {
            Operation::Return => Operation::simple_instruction("OP_RETURN".to_string()),
            Operation::ConstantSmol(i) => {
                let index: usize = (*i).into();
                Operation::constant_instruction(
                    "OP_CONST_SMOL".to_string(),
                    index,
                    &chunk.values[index],
                )
            }
            Operation::ConstantThicc(i) => {
                let index: usize = i.to_usize();
                Operation::constant_instruction(
                    "OP_CONST_THICC".to_string(),
                    index,
                    &chunk.values[index],
                )
            }
        }
    }
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

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    // 1-1 mapping with line numbers.
    // TODO: Come up with a more efficient encoding.
    lines: Vec<LineData>,
}

impl Chunk {
    pub fn write(&mut self, op: Operation, line: u32) {
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

    pub fn add_constant(&mut self, value: Value, line: u32) {
        let val_index = self.values.len();
        self.values.push(value);
        if val_index <= 255 {
            self.write(Operation::ConstantSmol(val_index.try_into().unwrap()), line);
        } else {
            self.write(Operation::ConstantThicc(u24::from_usize(val_index)), line);
        }
    }

    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        let ops = Operation::read_all(&self.code);
        for (op_index, Positioned { val: op, pos }) in ops.iter().enumerate() {
            op.print(&self, op_index, *pos)
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
