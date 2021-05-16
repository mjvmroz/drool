#[cfg(debug_assertions)]
use chunk::{Chunk, Operation};
use value::Value;
use vm::VM;

mod chunk;
mod data;
mod value;
mod vm;

fn main() {
    let mut test = Chunk::default();
    test.add_constant(Value::Double(3.0), 1);
    test.add_constant(Value::Double(6.0), 2);
    test.write(Operation::Multiply, 3);
    test.write(Operation::Negate, 3);
    test.add_constant(Value::Double(4.0), 4);
    test.write(Operation::Subtract, 4);
    test.add_constant(Value::Double(2.0), 5);
    test.write(Operation::Divide, 5);
    test.write(Operation::Return, 5);

    let mut vm = VM::new(test);
    vm.run();
}
