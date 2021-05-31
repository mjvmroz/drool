use std::io::{self, stdout, Write};

use crate::vm::VM;

pub struct Repl {
    vm: VM,
}
impl Repl {
    pub fn new(vm: VM) -> Repl {
        Repl { vm }
    }

    pub fn start(&mut self) -> io::Result<()> {
        println!();
        println!();
        println!("ドルール。");
        println!();
        loop {
            print!("> ");
            stdout().flush()?;
            let line: String = text_io::read!("{}\n");
            if line.len() == 0 {
                println!();
                return Ok(());
            }
            match self.vm.interpret(line.as_str()) {
                Ok(()) => {}
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
