use std::fmt::{self, Display};

use broom::{
    prelude::{Trace, Tracer},
    Handle, Heap,
};

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

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Object {
    Str(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Trace<Self> for Object {
    fn trace(&self, _tracer: &mut Tracer<Self>) {
        match self {
            Object::Str(_) => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Value {
    Double(f64),
    Nil,
    Bool(bool),
    Obj(Handle<Object>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => value.fmt(f),
            Self::Bool(value) => value.fmt(f),
            Self::Nil => write!(f, "nil"),
            Self::Obj(handle) => unsafe { write!(f, "{}", handle.get_unchecked()) },
        }
    }
}

#[allow(dead_code)]
impl Value {
    #[inline]
    pub fn negate(_heap: &mut Heap<Object>, val: Value) -> TypeResult<Value> {
        match val {
            Self::Double(double) => Ok(Self::Double(-double)),
            _ => Err(TypeError::NotANumber(val)),
        }
    }

    #[inline]
    pub fn not(_heap: &mut Heap<Object>, val: Value) -> TypeResult<Value> {
        match val {
            Self::Bool(bool) => Ok(Value::Bool(!bool)),
            Self::Nil => Ok(Value::Bool(true)),
            _ => Err(TypeError::NotBoolLike(val)),
        }
    }

    #[inline]
    pub fn add(heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a + b)),
            (Self::Obj(a), Self::Obj(b)) => unsafe {
                match (a.get_unchecked(), b.get_unchecked()) {
                    (Object::Str(a), Object::Str(b)) => {
                        let str = Object::Str(format!("{}{}", a, b));
                        let obj = heap.insert_temp(str);
                        Ok(Self::Obj(obj))
                    }
                }
            },
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn subtract(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a - b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn multiply(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a * b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn divide(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Self::Double(a / b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn divide_mut(_heap: &mut Heap<Object>, a: &mut Value, b: Value) -> TypeResult<()> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => {
                *a /= b;
                Ok(())
            },
            vw => Err(TypeError::NotANumber(*vw.0)),
        }
    }

    #[inline]
    pub fn equal(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Obj(a), Self::Obj(b)) => unsafe {
                Ok(Value::Bool(a.get_unchecked().eq(b.get_unchecked())))
            },
            _ => Ok(Value::Bool(a == b)),
        }
    }

    #[inline]
    pub fn greater(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Value::Bool(a > b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }

    #[inline]
    pub fn less(_heap: &mut Heap<Object>, a: Value, b: Value) -> TypeResult<Value> {
        match (a, b) {
            (Self::Double(a), Self::Double(b)) => Ok(Value::Bool(a < b)),
            vw => Err(TypeError::NotANumber(vw.0)),
        }
    }
}
