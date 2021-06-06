use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeError {
    NotANumber(Value),
    NotBoolLike(Value),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotANumber(v) => write!(f, "{} is not a number", v),
            Self::NotBoolLike(v) => write!(f, "{} cannot be coerced to a boolean", v),
        }
    }
}

pub type TypeResult<A> = Result<A, TypeError>;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Value {
    Double(f64),
    Nil,
    Bool(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => value.fmt(f),
            Self::Bool(value) => value.fmt(f),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

#[allow(dead_code)]
impl Value {
    #[inline]
    pub fn negate(&self) -> TypeResult<Value> {
        match self {
            Self::Double(double) => Ok(Self::Double(-double)),
            v => Err(TypeError::NotANumber(*v)),
        }
    }

    #[inline]
    pub fn negate_mut(&mut self) -> TypeResult<()> {
        match self {
            Self::Double(double) => Ok(*double = -(*double)),
            v => Err(TypeError::NotANumber(*v)),
        }
    }

    #[inline]
    pub fn not_mut(&mut self) -> TypeResult<()> {
        match self {
            Self::Bool(bool) => Ok(*bool = !(*bool)),
            v @ Self::Nil => Ok(*v = Value::Bool(true)),
            v => Err(TypeError::NotBoolLike(*v)),
        }
    }

    #[inline]
    pub fn add(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a + b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn add_mut(a: &mut Value, b: Value) -> TypeResult<()> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(*a += b),
            vw => Err(TypeError::NotANumber(*vw.0)),
        }
    }

    #[inline]
    pub fn subtract(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a - b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn subtract_mut(a: &mut Value, b: Value) -> TypeResult<()> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(*a -= b),
            vw => Err(TypeError::NotANumber(*vw.0)),
        }
    }

    #[inline]
    pub fn multiply(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a * b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn multiply_mut(a: &mut Value, b: Value) -> TypeResult<()> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(*a *= b),
            vw => Err(TypeError::NotANumber(*vw.0)),
        }
    }

    #[inline]
    pub fn divide(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a / b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn divide_mut(a: &mut Value, b: Value) -> TypeResult<()> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(*a /= b),
            vw => Err(TypeError::NotANumber(*vw.0)),
        }
    }

    #[inline]
    pub fn equal(a: Value, b: Value) -> TypeResult<Value> {
        Ok(Value::Bool(a == b))
    }

    #[inline]
    pub fn greater(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Value::Bool(a > b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn less(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Value::Bool(a < b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }
}
