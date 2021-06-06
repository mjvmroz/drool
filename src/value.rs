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
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[allow(dead_code)]
impl Value {
    #[inline]
    pub fn negate(val: Value) -> TypeResult<Value> {
        match val {
            Self::Double(double) => Ok(Self::Double(-double)),
            _ => Err(TypeError::NotANumber(val)),
        }
    }

    #[inline]
    pub fn not(val: Value) -> TypeResult<Value> {
        match val {
            Self::Bool(bool) => Ok(Value::Bool(!bool)),
            Self::Nil => Ok(Value::Bool(true)),
            _ => Err(TypeError::NotBoolLike(val)),
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
    pub fn subtract(a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a - b)),
            vw => Err(TypeError::NotANumber(vw.0)),
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
