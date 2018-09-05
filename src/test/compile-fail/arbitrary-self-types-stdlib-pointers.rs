// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(arbitrary_self_types)]
#![feature(pin)]
#![feature(rustc_attrs)]

use std::{
    rc::Rc,
    sync::Arc,
    pin::{PinMut, PinBox},
};

trait Trait {
    fn by_rc(self: Rc<Self>) -> i64;
    fn by_arc(self: Arc<Self>) -> i64;
    fn by_pin_mut(self: PinMut<Self>) -> i64;
    fn by_pin_box(self: PinBox<Self>) -> i64;
}

impl Trait for i64 {
    fn by_rc(self: Rc<Self>) -> i64 {
        *self
    }
    fn by_arc(self: Arc<Self>) -> i64 {
        *self
    }
    fn by_pin_mut(self: PinMut<Self>) -> i64 {
        *self
    }
    fn by_pin_box(self: PinBox<Self>) -> i64 {
        *self
    }
}


#[rustc_error]
fn main() { //~ ERROR compilation successful
    let rc = Rc::new(1i64) as Rc<dyn Trait>;
    assert_eq!(1, rc.by_rc());

    let arc = Arc::new(2i64) as Arc<dyn Trait>;
    assert_eq!(2, arc.by_arc());

    let mut value = 3i64;
    let pin_mut = PinMut::new(&mut value) as PinMut<dyn Trait>;
    assert_eq!(3, pin_mut.by_pin_mut());

    let pin_box = PinBox::new(4i64) as PinBox<dyn Trait>;
    assert_eq!(4, pin_box.by_pin_box());
}
