use std::{
    env, io,
    process::{self},
};

use chunk::Chunk;
use op::Op;
use repl::Repl;
use vm::VM;

mod chunk;
mod compiler;
mod data;
mod op;
mod repl;
mod scanner;
mod value;
mod vm;

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

fn main() -> io::Result<()> {
    switch()
}
