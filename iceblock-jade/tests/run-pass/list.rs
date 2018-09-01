// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(trace_macros)]

#[macro_use]
extern crate iceblock_jade;

struct Builder {
}
struct div {
}
#[derive(PartialEq, Eq, Debug)]
struct Div {
}
struct DivBuilder{
  reqs: div,
  _children: Vec<Node>,
}

#[derive(PartialEq, Eq, Debug)]
enum Node {
  Div(Div, Vec<Node>),
}
impl Builder {
  fn div(reqs: div) -> DivBuilder {
    DivBuilder {
      reqs,
      _children: vec!(),
    }
  }
}
impl DivBuilder {
  fn add_child(&mut self, node: Node) {
    self._children.push(node);
  }
  fn to_node(self) -> Node {
    Node::Div(Div{}, self._children)
  }
}


fn main() {
  trace_macros!(true);
  let t = jade!(
        div[]
        {
          div[]
          div[]
        }
    );
  trace_macros!(false);
  assert_eq!(t, Node::Div(Div{}, vec!(Node::Div(Div{}, vec!()), Node::Div(Div{}, vec!()))))
}