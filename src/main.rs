use chunk::{Chunk, Operation};
use value::Value;

mod chunk;
mod u24;
mod value;

fn main() {
    let mut test = Chunk::default();
    for i in 1..=260 {
        test.add_constant(Value::Double(i as f64), 122);
    }
    test.write(Operation::Return, 123);
    test.disassemble("Test".to_string());
}
