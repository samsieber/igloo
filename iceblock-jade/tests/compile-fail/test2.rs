// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate iceblock_jade;

fn main() {
  let t = jade!( //~ ERROR Wrapper
        div [a="testing a".to_string(), b="another!".to_string(), c=>"Actually a Some value".to_string(), d=>11,]
        (
          div[a="2".to_string(), b="2".to_string(),]
          div[a="2".to_string(), b="2".to_string(),]
        )
    );
}