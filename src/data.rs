use std::convert::{TryFrom, TryInto};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct u24([u8; 3]);

#[allow(clippy::many_single_char_names)]
impl u24 {
    pub fn to_usize(self) -> usize {
        let u24([a, b, c]) = self;
        usize::from_le_bytes([a, b, c, 0, 0, 0, 0, 0])
    }

    pub fn to_bytes(self) -> [u8; 3] {
        let u24(bytes) = self;
        bytes
    }
}

pub trait FromU24Bytes {
    unsafe fn from_u8_ptr(ptr: *const u8) -> Self;
}

impl FromU24Bytes for usize {
    unsafe fn from_u8_ptr(ptr: *const u8) -> usize {
        usize::from_le_bytes([*ptr, *ptr.add(1), *ptr.add(2), 0, 0, 0, 0, 0])
    }
}

impl FromU24Bytes for u24 {
    unsafe fn from_u8_ptr(ptr: *const u8) -> u24 {
        u24([*ptr, *ptr.add(1), *ptr.add(2)])
    }
}

impl From<usize> for u24 {
    fn from(v: usize) -> u24 {
        let [a, b, c, d, e, f, g, h] = v.to_le_bytes();
        [d, e, f, g, h].iter().for_each(|v| debug_assert!(*v == 0));
        u24([a, b, c])
    }
}

impl From<u24> for usize {
    fn from(val: u24) -> Self {
        val.to_usize()
    }
}

#[derive(Debug)]
pub struct OutOfRangeError {}
impl TryFrom<u32> for u24 {
    type Error = OutOfRangeError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 0x1_00_00_00 {
            let vv: usize = value.try_into().unwrap();
            Ok(vv.into())
        } else {
            Err(OutOfRangeError {})
        }
    }
}
