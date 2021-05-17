use std::{convert::TryInto, usize};
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct u24([u8; 3]);

#[allow(clippy::many_single_char_names)]
impl u24 {
    pub fn to_usize(self) -> usize {
        let u24([a, b, c]) = self;
        usize::from_le_bytes([a, b, c, 0, 0, 0, 0, 0])
    }

    pub fn from_usize(n: usize) -> Self {
        let [a, b, c, d, e, f, g, h] = n.to_le_bytes();
        [d, e, f, g, h].iter().for_each(|v| debug_assert!(*v == 0));
        u24([a, b, c])
    }

    pub fn to_bytes(self) -> [u8; 3] {
        let u24(bytes) = self;
        return bytes;
    }

    pub fn from_bytes(bytes: [u8; 3]) -> u24 {
        return u24(bytes);
    }

    pub fn from_buffer(buffer: &Vec<u8>, pos: usize) -> u24 {
        u24::from_bytes(
            buffer[pos..=pos + 2]
                .try_into()
                .expect("Are you bad at math or something? This slice should have THREE things 🙄"),
        )
    }
}

pub trait FromU24Bytes {
    unsafe fn from_u24_ptr(ptr: *const u8) -> Self;
}

impl FromU24Bytes for usize {
    unsafe fn from_u24_ptr(ptr: *const u8) -> usize {
        usize::from_le_bytes([*ptr, *ptr.add(1), *ptr.add(2), 0, 0, 0, 0, 0])
    }
}
