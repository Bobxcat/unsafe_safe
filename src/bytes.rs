use std::mem::{size_of, MaybeUninit};

use crate::transmute::transmute;

/// Creates a value of type `T` directly with the bytes provided
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> T {
    assert_eq!(bytes.len(), size_of::<T>());

    let y = MaybeUninit::<T>::uninit();
    let y_ptr: usize = transmute(&y);
    for offset in 0..size_of::<T>() {
        let write_byte_ptr: &mut u8 = transmute(y_ptr + offset);
        *write_byte_ptr = bytes[offset];
    }

    transmute(y)
}

/// Gets the bytes of `*x`
#[inline]
pub fn get_bytes<T>(x: &T) -> Vec<u8> {
    let x_ptr: usize = transmute(x);
    let mut v = vec![];
    for offset in 0..size_of::<T>() {
        let read_byte_ptr: &u8 = transmute(x_ptr + offset);
        v.push(*read_byte_ptr);
    }

    v
}

/// Creates a bitwise copy of `x` and returns it
#[inline]
pub fn copy<T>(x: &T) -> T {
    from_bytes(&get_bytes(x))
}
