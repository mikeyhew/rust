// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(arbitrary_self_types, unsize, coerce_unsized, box_syntax)]

use std::{
    ops::{Deref, CoerceUnsized},
    marker::Unsize,
    fmt::Debug,
};

struct Ptr<T: ?Sized>(Box<T>);

impl<T: ?Sized> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T> {}

struct Wrapper<T: ?Sized>(T);

impl<T: ?Sized> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: CoerceUnsized<U>, U> CoerceUnsized<Wrapper<U>> for Wrapper<T> {}

trait Trait {
    // This method can't be called on trait objects, since the receiver would be unsized,
    // but should not cause an object safety error
    fn wrapper(self: Wrapper<Self>) -> i32;
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32;
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32;
}

impl Trait for i32 {
    fn wrapper(self: Wrapper<Self>) -> i32 {
        *self
    }
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32 {
        **self
    }
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32 {
        ***self
    }
}

fn main() {
    let pw = Ptr(box Wrapper(5)) as Ptr<Wrapper<Debug>>;
    assert_eq!(pw.ptr_wrapper(), 5);

    let wpw = Wrapper(Ptr(box Wrapper(6))) as Wrapper<Ptr<Wrapper<Debug>>>;
    assert_eq!(wpw.wrapper_ptr_wrapper(), 6);
}
