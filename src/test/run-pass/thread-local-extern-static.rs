// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-windows
// aux-build:thread-local-extern-static.rs

#![feature(thread_local)]
#![feature(cfg_target_thread_local)]

extern crate thread_local_extern_static;

extern {
    #[cfg_attr(target_thread_local, thread_local)]
    static FOO: u32;
}

fn main() {
    unsafe {
        assert_eq!(FOO, 3);
    }
}
