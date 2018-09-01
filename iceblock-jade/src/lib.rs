#![feature(concat_idents)]
#![feature(log_syntax)]
#![feature(trace_macros)]
#![recursion_limit="64"]

extern crate gen_test;
extern crate iceblock;

use gen_test::*;
use iceblock::*;

#[macro_export]
macro_rules! jade_like_attrs {
    // Entry point (public format)
    ($node:ident [$($attrs:tt)*]) => {
        jade_like_attrs!($node @custom false [$($attrs)*])
    };
    // Entry point (custom tag type)
    ($node:ident @custom true [$($attrs:tt)*]) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[] @__optional[] @fun custom)
    };
    // Entry point (normal tag type)
    ($node:ident @custom false [$($attrs:tt)*]) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[] @__optional[] @fun $node)
    };

    // Required Attribute (first)
    ($node:ident @__processing [$attr:ident = $value:expr, $($attrs:tt)*] @__required[] @__optional [$($o:tt)*] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$attr = $value] @__optional [$($o)*] $($tail)*)
    };
    // Required Attribute (extra)
    ($node:ident @__processing [$attr:ident = $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [$($o:tt)*] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$($r)*, $attr = $value] @__optional [$($o)*] $($tail)*)
    };


    // Optional Attribute (first) - needs wrapping
    ($node:ident @__processing [$attr:ident => $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$($r)*] @__optional [$attr = Some($value)] $($tail)*)
    };
    // Optional Attribute (extra) - needs wrapping
    ($node:ident @__processing [$attr:ident => $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [$($o:tt)*] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$($r)*] @__optional [$($o)*, $attr = Some($value)] $($tail)*)
    };


    // Optional Attribute (first) - no wrapping
    ($node:ident @__processing [$attr:ident -> $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$($r)*] @__optional [$attr = $value] $($tail)*)
    };
    // Optional Attribute (extra) - no wrapping
    ($node:ident @__processing [$attr:ident -> $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [$($o:tt)*] $($tail:tt)*) => {
        jade_like_attrs!($node @__processing [$($attrs)*] @__required[$($r)*] @__optional [$($o)*, $attr = $value] $($tail)*)
    };



    // No more processing
    ($node:ident @__processing [] @__required[$($r:tt)*] @__optional [$($o:tt)*] $($tail:tt)*) => {
        jade_like_attrs!($node @__required[$($r)*] @__optional [$($o)*] $($tail)*)
    };



    // Render
    ($node:ident @__required[$($req:ident = $req_value:expr),*] @__optional [$($opt:ident = $opt_value:expr),*] @fun $fun:ident ) => {
        {
            let mut parent = Builder::$fun($node {
                $($req: $req_value,)*
            });
            $(parent.$opt = $opt_value;)*
            parent
        }
    };
}

#[macro_export]
macro_rules! jade_like {
    //  1) If there's no more tail, go to rendering. Else continue to step 2
    (
        @searching [$($meta:tt)*] [$($children:tt)*] []
    ) => {
        jade_like! (@render [$($meta)*] [$($children)*]);
    };

    // 2) Extract the tag -> custom check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] []
        $tag:ident $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*]  [$tag] @custom $($tail)*);
    };

    // 3a) Custom check (true) -> Attrs check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @custom : $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* true] @attrs $($tail)*);
    };
    // 3b) Custom check (false) -> Attrs Check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @custom $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* false ] @attrs $($tail)*);
    };

    // 4a) Attrs check (present) -> Child check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @attrs [$($attrs:tt)*] $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* [$($attrs)*]] @child $($tail)*);
    };
    // 4b) Attrs check (none) -> Child check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @attrs [$($attrs:tt)*] $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* []] @child $($tail)*);
    };

    // 5a) Child check (one) -> Child check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @child ($($child:tt)*) $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* one [$($child)*]] @found $($tail)*);
    };
    // 5b) Child check (list) -> Child check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @child {$($child:tt)*} $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* list [$($child)*]] @found $($tail)*);
    };
    // 5c) Child check (none) -> Child check
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($building:tt)*]
        @child $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)*] [$($building)* none []] @found $($tail)*);
    };

    // 6) Child found. Add child to list and continue back to 1
    (
        @searching [$($meta:tt)*] [$($children:tt)*] [$($built:tt)*]
        @found $($tail:tt)*
    ) => {
        jade_like! (@searching [$($meta)*] [$($children)* [$($built)*]] [] $($tail)*);
    };


    /*
        Rendering Time
    */
    // Render none
    (@render [none @parent $parent:ident] []) => {
        {}
    };
    (@render [none @parent $parent:ident] [$($extra:tt)*]) => {
        compile_error!("Unreachable state - at least one child when none expected");
    };


    // Render a plain item
    (@render [one @plain] [[$tag:ident $is_custom:ident [$($attrs:tt)*] $child_type:ident [$($children:tt)*] ]]) => {{
        let mut parent = jade_like_attrs!($tag @custom $is_custom [$($attrs)*]);
        jade_like!(@searching [$child_type @parent parent] [] [] $($children)*);
        parent.to_node()
    }};
    // Add a child to the parent
    (@render [one @parent $parent:ident] [[$($part_of_child:tt)*]]) => {
        $parent.set_child(jade_like!(@render [one @plain] [[$($part_of_child)*]]))
    };
    // Add children to the parent - flatten list
    (@render [list @parent $parent:ident] [$($renderable_children:tt)*]) => {{
        $($parent.add_child(jade_like!(@render [one @plain] [$renderable_children]));)*
    }};
}

//    rep!(div a, b, c);
//    rep!(div a="11" , c=Some("test"), b="another");
//    render!(div[a="test", b="see", c=Some("some")][/])
//    close!(/);
//    built_in_node!(<div a="testing a", b="another test", c*="CCCCC");
//    built_in_node!(@__working div  ;  ; c*="ccc", d*=11, b="42" ,a="string", @__attrs_done);
//    built_in_node!(@__working div  ;  ; a="ccc", d*=11, b="42" ,c*="string", @__attrs_done);

#[macro_export]
macro_rules! jade {
    ($($all:tt)*) => {
        jade_like!(@searching [one @plain] [] [] $($all)*)
    }
}

// Called by macro
struct Builder {
}
// Called by macro
struct div {
  a: String,
  b: String,
}
struct mycust {
  a: String,
  b: String,
}
struct Div {
  a: String,
  b: String,
  c: Option<String>,
  d: Option<u32>,
}
struct Cust {

}
// This could be named anything really...
struct DivBuilder{
  reqs: div,
  c: Option<String>,
  d: Option<u32>,
  _children: Vec<Node>,
}
enum Node {
  Div(Div),
  Custom(Cust),
}
impl Builder {
  // Called by macro
  fn div(reqs: div) -> DivBuilder {
    DivBuilder {
      reqs,
      c: None,
      d: None,
      _children: vec!(),
    }
  }
  fn custom(reqs: div) -> DivBuilder {
    DivBuilder {
      reqs,
      c: None,
      d: None,
      _children: vec!(),
    }
  }
}
impl DivBuilder {
  // Called by macro -
  fn add_child(&mut self, node: Node) {
    self._children.push(node);
  }
  fn to_node(self) -> Node {
    Node::Div(Div{
      a:self.reqs.a,
      b:self.reqs.b,
      c:self.c,
      d:self.d,
    })
  }
}

pub struct TestStruct {

}
