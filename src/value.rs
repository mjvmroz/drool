pub enum Value {
    Double(f64),
}

impl Value {
    pub fn print(&self) {
        match self {
            Self::Double(value) => print!("{}", value),
        }
    }
}
