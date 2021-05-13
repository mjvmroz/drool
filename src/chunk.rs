use std::usize;

#[derive(Debug)]
pub enum Operation {
    Return,
    Constant,
}

impl Operation {
    fn simple_instruction(name: String) {
        println!("{}", name);
    }

    fn print(&self, pos: usize) {
        print!("{:0>4} ", pos);
        match self {
            Operation::Return => Operation::simple_instruction("OP_RETURN".to_string()),
            Operation::Constant => Operation::simple_instruction("OP_CONSTANT".to_string()),
        }
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    code: Vec<Operation>,
    values: Vec<Operation>,
}

impl Chunk {
    pub fn write(&mut self, op: Operation) {
        self.code.push(op);
    }

    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        for (pos, op) in self.code.iter().enumerate() {
            // I cannot for the life of me figure out how to get actual offsets.
            op.print(pos);
        }
    }
}
