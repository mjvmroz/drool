use std::io::{self, stdout, Read, Write};

use crate::vm::VM;

pub struct Repl<'a> {
    vm: VM<'a>,
}
impl<'a> Repl<'a> {
    pub fn new(vm: VM<'a>) -> Repl {
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
                println!("ありがとうございます");
                return Ok(());
            }
            println!("え?");
        }
    }
}
