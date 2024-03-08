use std::mem::size_of;

use crate::extend_lifetime_mut;

enum Container<T, G> {
    Foo(Option<Box<T>>),
    Bar(Option<Box<G>>),
}

pub fn transmute_unchecked<T, G>(a: T) -> G {
    let some_a = Some(Box::new(a));
    let mut container: Container<T, G> = Container::Bar(None);

    let p = &mut container;
    let a = match p {
        Container::Bar(x) => x,
        _ => unreachable!(),
    };
    let a = extend_lifetime_mut(a); // Feels a little magical, don't it?

    *p = Container::Foo(some_a);

    *a.take().unwrap()
}

pub fn transmute<T, G>(a: T) -> G {
    assert_eq!(size_of::<T>(), size_of::<G>());
    transmute_unchecked(a)
}
