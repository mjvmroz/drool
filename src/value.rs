#[derive(Clone, Copy)]
pub enum Value {
    Double(f64),
}

impl Value {
    pub fn print(&self) {
        match self {
            Self::Double(value) => print!("{}", value),
        }
    }

    #[inline(always)]
    // Not sure how expensive this unwrapping and rewrapping is.
    // Might want to make shit mutable, or do some casting. 🧙‍♂️
    // EDIT: Turned out that was a challenge question. Added a mutating implementation.
    pub fn negate(&self) -> Value {
        match self {
            Self::Double(value) => Self::Double(-value),
        }
    }

    #[inline(always)]
    pub fn negate_mut(&mut self) {
        match self {
            Self::Double(value) => *value = -(*value),
        }
    }

    #[inline(always)]
    pub fn add(a: Value, b: Value) -> Value {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Self::Double(a + b),
        }
    }

    #[inline(always)]
    pub fn add_mut(a: &mut Value, b: Value) {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => *a += b,
        }
    }

    #[inline(always)]
    pub fn subtract(a: Value, b: Value) -> Value {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Self::Double(a - b),
        }
    }

    #[inline(always)]
    pub fn subtract_mut(a: &mut Value, b: Value) {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => *a -= b,
        }
    }

    #[inline(always)]
    pub fn multiply(a: Value, b: Value) -> Value {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Self::Double(a * b),
        }
    }

    #[inline(always)]
    pub fn multiply_mut(a: &mut Value, b: Value) {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => *a *= b,
        }
    }

    #[inline(always)]
    pub fn divide(a: Value, b: Value) -> Value {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Self::Double(a / b),
        }
    }

    #[inline(always)]
    pub fn divide_mut(a: &mut Value, b: Value) {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => *a /= b,
        }
    }
}
