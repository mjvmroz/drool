#[derive(Debug)]
pub enum Operation {
    Return,
}

#[derive(Debug, Default)]
pub struct Chunk {
    code: Vec<Operation>,
}

impl Chunk {
    pub fn write(&mut self, op: Operation) {
        self.code.push(op);
    }

    pub fn disassemble(self, name: String) {
        println!("== {} ==", name);
    }
}
