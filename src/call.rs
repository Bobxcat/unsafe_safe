use crate::transmute::transmute;

pub trait UnsafeFn<Args>: Copy {
    // type Args;
    type Ret;
    fn call(self, args: Args) -> Self::Ret;
}

impl<A0, R> UnsafeFn<A0> for unsafe fn(A0) -> R {
    type Ret = R;

    fn call(self, args: A0) -> Self::Ret {
        let f: fn(A0) -> R = transmute(self);
        f(args)
    }
}

impl<A0, R> UnsafeFn<(A0,)> for unsafe fn(A0) -> R {
    type Ret = R;

    fn call(self, args: (A0,)) -> Self::Ret {
        let f: fn(A0) -> R = transmute(self);
        f(args.0)
    }
}

impl<A0, A1, R> UnsafeFn<(A0, A1)> for unsafe fn(A0, A1) -> R {
    type Ret = R;

    fn call(self, args: (A0, A1)) -> Self::Ret {
        let f: fn(A0, A1) -> R = transmute(self);
        f(args.0, args.1)
    }
}

impl<A0, A1, A2, R> UnsafeFn<(A0, A1, A2)> for unsafe fn(A0, A1, A2) -> R {
    type Ret = R;

    fn call(self, args: (A0, A1, A2)) -> Self::Ret {
        let f: fn(A0, A1, A2) -> R = transmute(self);
        f(args.0, args.1, args.2)
    }
}

/// Invoke this method as in the following examples:
/// ```
/// unsafe fn foo(n: usize) -> () { ... }
/// unsafe fn bar(n: usize, m: u16, w: u32) -> f32 { ... }
///
/// call_unsafe::<unsafe fn(_) -> _, _>(foo, 10);
/// call_unsafe::<unsafe fn(_) -> _, _>(foo, (10));
/// let num = call_unsafe::<unsafe fn(_, _, _) -> _, _>(bar, (10, 20, 30));
/// ```
///
/// It's a little clunky, sorry!
pub fn call_unsafe<F: UnsafeFn<A>, A>(f: F, args: A) -> F::Ret {
    f.call(args)
}

pub fn call_ffi<Args, Ret>(f: unsafe extern "C" fn(Args) -> Ret, args: Args) -> Ret {
    let f: extern "C" fn(Args) -> Ret = transmute(f);
    f(args)
}

pub mod alloc {
    use std::alloc::Layout;

    use super::call_unsafe;

    pub fn alloc(layout: Layout) -> *mut u8 {
        call_unsafe::<unsafe fn(_) -> _, _>(std::alloc::alloc, layout)
    }

    pub fn dealloc(ptr: *mut u8, layout: Layout) {
        call_unsafe::<unsafe fn(_, _) -> _, (_, _)>(std::alloc::dealloc, (ptr, layout))
    }
}
