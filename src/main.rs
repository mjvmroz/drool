use crate::operation::Operation as Op;
use chunk::Chunk;
use value::Value as Val;
use vm::VM;

mod chunk;
mod data;
mod operation;
mod value;
mod vm;

fn test(name: &str, description: &str, f: fn(&mut Chunk) -> ()) {
    println!();
    println!("========= {:^13} =========", name);
    println!("{:^34}", description);
    VM::new(&mut Chunk::of(f)).run();
}

fn main_example() {
    test("MAIN", "Main contents for canonical project", |c| {
        c.constant(Val::Double(1.2), 123);
        c.constant(Val::Double(3.4), 123);
        c.operation(Op::Add, 123);
        c.constant(Val::Double(5.6), 123);
        c.operation(Op::Divide, 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_1a() {
    test("Ch. 15.1a", "(1 * 2 + 3)", |c| {
        c.constant(Val::Double(1.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.operation(Op::Multiply, 123);
        c.constant(Val::Double(3.0), 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_1b() {
    test("Ch. 15.1b", "(1 + 2 * 3)", |c| {
        c.constant(Val::Double(1.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.constant(Val::Double(3.0), 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_1c() {
    test("Ch. 15.1c", "(3 - 2 - 1)", |c| {
        c.constant(Val::Double(3.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.operation(Op::Subtract, 123);
        c.constant(Val::Double(1.0), 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_1d() {
    test("Ch. 15.1d", "(1 + 2 * 3 - 4 / -5)", |c| {
        c.constant(Val::Double(1.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.constant(Val::Double(3.0), 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Add, 123);
        c.constant(Val::Double(4.0), 123);
        c.constant(Val::Double(5.0), 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Divide, 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_2a() {
    test("Ch. 15.2a", "(4 - 3 * -2) without NEGATE", |c| {
        c.constant(Val::Double(4.0), 123);
        c.constant(Val::Double(3.0), 123);
        c.constant(Val::Double(0.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    });
}

fn challenge_15_2b() {
    test("Ch. 15.2b", "(4 - 3 * -2) without SUBTRACT", |c| {
        c.constant(Val::Double(4.0), 123);
        c.constant(Val::Double(3.0), 123);
        c.constant(Val::Double(2.0), 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    });
}

fn main() {
    main_example();
    challenge_15_1a();
    challenge_15_1b();
    challenge_15_1c();
    challenge_15_1d();
    challenge_15_2a();
    challenge_15_2b();
}
