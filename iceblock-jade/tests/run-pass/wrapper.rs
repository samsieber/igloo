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

struct Builder {}
struct div {}

#[derive(PartialEq, Eq, Debug)]
struct Div {}
struct plain {}

#[derive(PartialEq, Eq, Debug)]
struct Plain {}
struct DivBuilder {
  reqs: div,
  _children: Vec<Node>,
}
struct PlainBuilder {}
#[derive(PartialEq, Eq, Debug)]
enum Node {
  Div(Div, Box<Node>),
  Plain(Plain),
}
impl Builder {
  fn div(reqs: div) -> DivBuilder {
    DivBuilder {
      reqs,
      _children: vec!(),
    }
  }
  fn plain(reqs: plain) -> PlainBuilder {
    PlainBuilder {}
  }
}
impl DivBuilder {
  fn set_child(&mut self, node: Node) {
    self._children.push(node);
  }
  fn to_node(mut self) -> Node {
    Node::Div(Div{}, Box::new(self._children.remove(0)))
  }
}
impl PlainBuilder {
  fn to_node(mut self) -> Node {
    Node::Plain(Plain{})
  }
}


fn main() {
  let t = jade!(
      div[](plain[])
//        div[]
//        (
//          div[]
//        )
    );
  assert_eq!(t, Node::Div(Div{}, Box::new(Node::Plain(Plain{}))));
}