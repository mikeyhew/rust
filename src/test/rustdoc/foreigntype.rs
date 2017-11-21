// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(extern_types, dynsized)]

use std::marker::DynSized;

extern {
    // @has foreigntype/foreigntype.ExtType.html
    pub type ExtType;
}

impl ExtType {
    // @has - '//a[@class="fnname"]' 'do_something'
    pub fn do_something(&self) {}
}

pub trait Trait: ?DynSized {}

// @has foreigntype/trait.Trait.html '//a[@class="foreigntype"]' 'ExtType'
impl Trait for ExtType {}

// @has foreigntype/index.html '//a[@class="foreigntype"]' 'ExtType'
