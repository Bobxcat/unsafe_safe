use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
};

use crate::transmute::transmute;

/// A mutable raw pointer, written without ever using the word `unsafe`!
pub struct Ptr<T>(usize, PhantomData<T>);

impl<T> Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = ['E'; size_of::<usize>()];
        let mut s = format!("0x");
        for i in 0..chars.len() {
            s.push(self.0.to_ne_bytes()[i] as char);
        }
        write!(f, "{s}")
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Ptr<T> {}

impl<T> Ptr<T> {
    pub fn null() -> Self {
        Self::from_addr(0)
    }
    pub fn from_addr(addr: usize) -> Self {
        Self(addr, PhantomData)
    }
    pub fn from_ptr(p: *const T) -> Self {
        Self::from_addr(transmute(p))
    }
    pub fn from_ref(r: &T) -> Self {
        Self::from_ptr(r)
    }
    /// Puts `v` onto the heap and returns a reference to it.
    /// If you want to be safe, you should free it eventually
    ///
    /// Note: Unfortunately, merely leaking memory is not a violation of Rust's memory safety principles
    pub fn allocate(v: T) -> Self {
        let v = Box::leak(Box::new(v));
        Self::from_ref(v)
    }
    /// Calls the destructor for the pointed to value and deallocates the memory
    pub fn free(self) {
        // https://doc.rust-lang.org/std/mem/fn.size_of.html
        // usize, &T and Box<T> have the same size for `Sized` T
        let v: Box<T> = transmute(self.0);
        std::mem::drop(v);
    }
    pub fn as_ref<'b>(self) -> &'b T {
        transmute(self.0)
    }
    pub fn as_mut<'b>(self) -> &'b mut T {
        transmute(self.0)
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

/// Mwahahahahahahahahaahahahahahahahahahahahahahahahahahahhahahaha
pub fn make_mut<'a, 'b, T>(r: &'a T) -> &'b mut T {
    transmute(r)
}
