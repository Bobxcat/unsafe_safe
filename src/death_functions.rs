use crate::{bytes::copy, extend_lifetime, pointer::Ptr};
use rand::{prelude::SliceRandom, thread_rng};

struct Cool {
    foo: u32,
    bar: String,
}

impl std::fmt::Debug for Cool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cool")
            .field("foo", &self.foo)
            .field("bar", &self.bar)
            .finish()
    }
}

impl Cool {
    pub fn new() -> Self {
        Self {
            foo: 10012,
            bar: "10012".into(),
        }
    }
}

pub fn death_by_double_free() {
    println!("Death by double free");

    let a = Cool::new();
    let b = copy(&a); // Can be wrapped in `ManuallyDrop<Cool>` to avoid the double free (still UB tho)

    println!("{a:#?}\n{b:#?}");
}

pub fn death_by_double_free_with_ptr() {
    println!("Death by double free with `Ptr` (Allocated on the heap)");

    let a = Cool::new();
    let p0 = Ptr::allocate(a);

    p0.free();
    p0.free();
}

pub fn death_by_null_deref() {
    println!("Death by null deref");

    let null: Ptr<Cool> = Ptr::null();
    println!("{:#?}", *null)
}

pub fn death_by_use_after_free() {
    println!("Death by use after free");

    let a: &Cool;
    {
        let c = Cool::new();
        a = extend_lifetime(&c);
    }
    println!("{:#?}", a);
}

fn deaths() -> Vec<fn()> {
    vec![
        death_by_double_free,
        death_by_double_free_with_ptr,
        death_by_null_deref,
        death_by_use_after_free,
    ]
}

pub fn death_by_random() {
    let d = deaths();
    let f = d.choose(&mut thread_rng()).unwrap();
    f();
}

// pub fn mutate_static() {
//     static mut FOO_INT: usize = 1000;
//     extern "C" {
//         static mut FOO_EXT: usize;
//     }
//     let int = &FOO_INT;
//     let ext = ;
// }
