// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Original implementation taken from rust-memchr
// Copyright 2015 Andrew Gallant, bluss and Nicolas Koch

pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    use libc;

    let p = unsafe {
        libc::memchr(
            haystack.as_ptr() as *const libc::c_void,
            needle as libc::c_int,
            haystack.len())
    };
    if p.is_null() {
        None
    } else {
        Some(p as usize - (haystack.as_ptr() as usize))
    }
}

pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    // turns out that newlib doesn't have memrchr(), so we
    // use the fallback version instead
    fn memrchr_specific(needle: u8, haystack: &[u8]) -> Option<usize> {
        ::sys_common::memchr::fallback::memrchr(needle, haystack)
    }

    memrchr_specific(needle, haystack)
}
