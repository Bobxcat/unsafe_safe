fn nothing_strange_here<'a, 'b, T: ?Sized>(
    data: &'a T,
    _inconspicuous_little_helper_guy: &'b &'a (),
) -> &'b T {
    data
}

pub fn extend_lifetime<'a, 'b, T: ?Sized>(data: &'a T) -> &'b T {
    let w: fn(&'a T, &'static &'static ()) -> &'b T = nothing_strange_here;
    w(data, &&())
}

pub fn as_static<T: ?Sized>(data: &T) -> &'static T {
    extend_lifetime(data)
}

fn nothing_strange_here_mut<'a, 'b, T: ?Sized>(
    data: &'a mut T,
    _inconspicuous_little_helper_guy: &'b &'a (),
) -> &'b mut T {
    data
}

pub fn extend_lifetime_mut<'a, 'b, T: ?Sized>(data: &'a mut T) -> &'b mut T {
    let w: fn(&'a mut T, &'static &'static ()) -> &'b mut T = nothing_strange_here_mut;
    w(data, &&())
}

pub fn as_static_mut<T: ?Sized>(data: &mut T) -> &'static mut T {
    extend_lifetime_mut(data)
}
