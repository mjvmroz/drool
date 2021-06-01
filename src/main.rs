use std::{
    env, io,
    process::{self},
};

use chunk::Chunk;
use op::Op;
use repl::Repl;
use value::Value as Val;
use vm::{InterpretResult, VM};

use crate::vm::InterpretError;

mod chunk;
mod compiler;
mod data;
mod fun;
mod op;
mod repl;
mod scanner;
mod value;
mod vm;

fn test(name: &str, description: &str, f: fn(&mut Chunk) -> ()) -> InterpretResult<'static, ()> {
    println!();
    println!("========= {:^13} =========", name);
    println!("{:^34}", description);
    println!();
    let chunk = Chunk::of(f);
    if cfg!(debug_assertions) {
        chunk.disassemble("Pre-exec disassembly:");
        println!();
        println!("== {:^27} ==", "Execution:");
    }
    VM::new(chunk).run().map_err(InterpretError::Runtime)
}

fn main_example() -> InterpretResult<'static, ()> {
    test("MAIN", "Main contents for canonical project", |c| {
        c.push_const(Val::Double(1.2), 123);
        c.push_const(Val::Double(3.4), 123);
        c.operation(Op::Add, 123);
        c.push_const(Val::Double(5.6), 123);
        c.operation(Op::Divide, 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_1a() -> InterpretResult<'static, ()> {
    test("Ch. 15.1a", "(1 * 2 + 3)", |c| {
        c.push_const(Val::Double(1.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.operation(Op::Multiply, 123);
        c.push_const(Val::Double(3.0), 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_1b() -> InterpretResult<'static, ()> {
    test("Ch. 15.1b", "(1 + 2 * 3)", |c| {
        c.push_const(Val::Double(1.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.push_const(Val::Double(3.0), 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_1c() -> InterpretResult<'static, ()> {
    test("Ch. 15.1c", "(3 - 2 - 1)", |c| {
        c.push_const(Val::Double(3.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.operation(Op::Subtract, 123);
        c.push_const(Val::Double(1.0), 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_1d() -> InterpretResult<'static, ()> {
    test("Ch. 15.1d", "(1 + 2 * 3 - 4 / -5)", |c| {
        c.push_const(Val::Double(1.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.push_const(Val::Double(3.0), 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Add, 123);
        c.push_const(Val::Double(4.0), 123);
        c.push_const(Val::Double(5.0), 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Divide, 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_2a() -> InterpretResult<'static, ()> {
    test("Ch. 15.2a", "(4 - 3 * -2) without NEGATE", |c| {
        c.push_const(Val::Double(4.0), 123);
        c.push_const(Val::Double(3.0), 123);
        c.push_const(Val::Double(0.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Subtract, 123);
        c.operation(Op::Return, 123);
    })
}

fn challenge_15_2b() -> InterpretResult<'static, ()> {
    test("Ch. 15.2b", "(4 - 3 * -2) without SUBTRACT", |c| {
        c.push_const(Val::Double(4.0), 123);
        c.push_const(Val::Double(3.0), 123);
        c.push_const(Val::Double(2.0), 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Multiply, 123);
        c.operation(Op::Negate, 123);
        c.operation(Op::Add, 123);
        c.operation(Op::Return, 123);
    })
}

fn repl() {
    let chunk = Chunk::of(|c| c.operation(Op::Return, 1));
    Repl::new(VM::new(chunk)).start().expect("Oh noes");
}

fn run_file(filename: &str) -> io::Result<()> {
    let src = std::fs::read_to_string(filename)?;

    let chunk = Default::default();
    let mut vm = VM::new(chunk);
    let result = vm.interpret(&src);

    match result {
        Err(vm::InterpretError::Compile(ce)) => {
            println!("Compilation failed: {:?}", ce);
            process::exit(exitcode::DATAERR);
        }
        Err(vm::InterpretError::Runtime(rte)) => {
            println!("Runtime error: {}", rte);
            process::exit(exitcode::SOFTWARE);
        }
        Ok(_) => (),
    };
    Ok(())
}

fn switch() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1])?,
        _ => {
            eprintln!("Usage: drool [file]");
            process::exit(exitcode::USAGE);
        }
    }
    Ok(())
}
fn run_ch15() -> InterpretResult<'static, ()> {
    main_example()?;
    challenge_15_1a()?;
    challenge_15_1b()?;
    challenge_15_1c()?;
    challenge_15_1d()?;
    challenge_15_2a()?;
    challenge_15_2b()
}

fn main() -> io::Result<()> {
    run_ch15().expect("Uh oh.");
    Ok(switch()?)
}
