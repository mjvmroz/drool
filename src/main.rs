use chunk::{Chunk, Operation};

mod chunk;
mod value;

fn main() {
    let mut test = Chunk::default();
    test.write(Operation::Return);
    test.disassemble("Test".to_string());
}
