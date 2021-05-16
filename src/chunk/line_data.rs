#[derive(Clone, Copy)]
pub struct LineData {
    pub ops: usize,
    pub line: u32,
}

impl LineData {
    pub fn new(line: u32) -> LineData {
        LineData { ops: 1, line }
    }

    pub fn tick(&mut self) {
        self.ops += 1;
    }
}
