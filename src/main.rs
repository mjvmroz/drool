use chunk::{Chunk, Operation};
use value::Value;
use vm::VM;

mod chunk;
mod data;
mod value;
mod vm;

fn main() {
    let vm = VM::new();

    let mut test = Chunk::default();
    test.add_constant(Value::Double(30.0), 1);
    test.write(Operation::Return, 2);
    test.disassemble("Test".to_string());

    vm.interpret(&test);
}
