// run-pass
#![feature(arbitrary_self_types, unsize, coerce_unsized, dispatch_from_dyn, unsized_locals)]
#![feature(rustc_attrs)]

use std::{
    ops::{Deref, CoerceUnsized, DispatchFromDyn},
    marker::Unsize,
};

struct Ptr<T: ?Sized> {
    bx: Box<T>,
    extra: i32,
}

impl<T: ?Sized> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.bx
    }
}

impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T> {}
impl<T: Unsize<U> + ?Sized, U: ?Sized> DispatchFromDyn<Ptr<U>> for Ptr<T> {}

struct Wrapper<T: ?Sized>{
    extra: i32,
    value: T,
}

impl<T: ?Sized> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: CoerceUnsized<U>, U> CoerceUnsized<Wrapper<U>> for Wrapper<T> {}
impl<T: DispatchFromDyn<U>, U> DispatchFromDyn<Wrapper<U>> for Wrapper<T> {}


trait Trait {
    fn wrapper(self: Wrapper<Self>) -> i32;
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32;
    fn wrapper_ptr(self: Wrapper<Ptr<Self>>) -> i32;
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32;
}

impl Trait for i32 {
    fn wrapper(self: Wrapper<Self>) -> i32 {
        self.extra + *self
    }
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32 {
        self.extra + (*self).extra + **self
    }
    fn wrapper_ptr(self: Wrapper<Ptr<Self>>) -> i32 {
        self.extra + (*self).extra + **self
    }
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32 {
        self.extra + (*self).extra + (**self).extra + ***self
    }
}

fn main() {
    // FIXME (mikeyhew) using *Box::new(...) to get an unsized rvalue
    // unsize the value directly once that's implemented
    let w = *(Box::new(Wrapper {
        extra: 1,
        value: 2,
    }) as Box<Wrapper<dyn Trait>>);

    assert_eq!(w.wrapper(), 3);

    let pw = Ptr {
        extra: 2,
        bx: Box::new(Wrapper {
            extra: 1,
            value: 2,
        }),
    } as Ptr<Wrapper<dyn Trait>>;

    assert_eq!(pw.ptr_wrapper(), 5);

    let wp = Wrapper {
        extra: 3,
        value: Ptr {
            extra: -2,
            bx: Box::new(5),
        },
    } as Wrapper<Ptr<dyn Trait>>;

    assert_eq!(wp.wrapper_ptr(), 6);

    let wpw = Wrapper {
        extra: 1,
        value: Ptr {
            extra: 2,
            bx: Box::new(Wrapper {
                extra: -4,
                value: 8,
            })
        },
    } as Wrapper<Ptr<Wrapper<dyn Trait>>>;

    assert_eq!(wpw.wrapper_ptr_wrapper(), 7);
}
