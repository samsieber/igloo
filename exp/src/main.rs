#![feature(concat_idents)]
#![feature(log_syntax)]
#![feature(trace_macros)]
#![recursion_limit="64"]

extern crate gen_test;
extern crate iceblock;

use gen_test::*;
use iceblock::*;

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

macro_rules! jade_like {
//    // Entry Point
//    ($node:ident [$($attrs:tt)*] {$($children:tt)*}) => {
//        jade_like!($node @__processing [$($attrs)*] @__required [] @__optional [] @__children {$($children)*})
//    };

    /*
        Arity checks for the one one or no children found simple cases
    */

    // Arity check fails (one child required, and no child found)
    (@status [$($status:tt)*] @find one) => {
        jade_like! (@error "One child required, but none found");
    };
    // Arity check passes (no child required, and no child found)
    (@status [$($status:tt)*] @find none) => {
        ;
    };
    // Arity check fails (no child required, and at least one child found)
    (@status [$($status:tt)*] @find none $(tail:tt)*) => {
        jade_like! (@error "No child required, but at least found");
    };

    /*
        Extract the first child
    */

    // 2) Extract the tag -> custom check
    (@status [$($status:tt)*] @find $find:ident $tag:ident $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @check_custom $($tail)*);
    };

    // 3a) Custom check (true) -> Attrs check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @check_custom : $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom true @check_for_attrs $($tail)*);
    };
    // 3b) Custom check (false) -> Attrs Check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @check_custom $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom false @check_for_attrs $($tail)*);
    };

    // 4a) Attrs check (present) -> Child check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @custom $is_custom:ident @check_for_attrs [$($attrs:tt)*] $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom $is_custom @attrs[$($attrs)*] @check_child $($tail)*);
    };
    // 4b) Attrs check (none) -> Child check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @custom $is_custom:ident @check_for_attrs $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom $is_custom @attrs [] @check_child $($tail)*);
    };

    // 5a) Child check (one) -> Child check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child ($($child:tt)*) $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom $is_custom @attrs[$($attrs)*] @child one [@find one$($child)*] $($tail)*);
    };
    // 5b) Child check (list) -> Child check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child {$($child:tt)*} $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom $is_custom @attrs[$($attrs)*] @child list [@find list $($child)*] $($tail)*);
    };
    // 5c) Child check (none) -> Child check
    (@status [$($status:tt)*] @find $find:ident @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find $find @tag $tag @custom $is_custom @attrs[$($attrs)*] @child none [@find none] $($tail)*);
    };

    /*
        Check arity again
    */

    // Arity check passes (one child required, and one child found)
    (@status [$($status:tt)*] @find one @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @child $child_type:ident [$($children:tt)*]) => {
        jade_like! (@render one @status[$($status)*] [@tag $tag @custom $is_custom @attrs[$($attrs)*] @child $child_type [$($children)*]]);
    };
    //  Arity check fails (one child required, and more than one child found)
    (@status [$($status:tt)*] @find one @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @child $($tail:tt)*) => {
        jade_like! (@error "A wrapper tag has more than one child");
    };


    /*
        Check for extra children
    */

    // We are processing a list and more are found
    (@status [$($status:tt)*] @find list @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @child $child_type:ident [$($children:tt)*] $($tail:tt)*) => {
        jade_like! (@status [$($status)*] @find list [[@tag $tag @custom $is_custom @attrs[$($attrs)*] @child $child_type [$($children)*]]] $($tail)*);
    };

    /*
        Process extra children (loop)
    */

    // 0) No more tags to extract for list - skip strait to rendering
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
    ) => {
        jade_like! (@render list @status [$($status)*] [$($already_found)*]);
    };

    // 2) Extract the tag -> custom check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        $tag:ident $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @check_custom $($tail)*);
    };

    // 3a) Custom check (true) -> Attrs check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @check_custom : $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom true @check_for_attrs $($tail)*);
    };
    // 3b) Custom check (false) -> Attrs Check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @check_custom $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom false @check_for_attrs $($tail)*);
    };

    // 4a) Attrs check (present) -> Child check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @check_for_attrs [$($attrs:tt)*] $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom $is_custom @attrs[$($attrs)*] @check_child $($tail)*);
    };
    // 4b) Attrs check (none) -> Child check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @check_for_attrs $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom $is_custom @attrs [] @check_child $($tail)*);
    };

    // 5a) Child check (one) -> Child check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child ($($child:tt)*) $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom $is_custom @attrs[$($attrs)*] @child one [@find one$($child)*] $($tail)*);
    };
    // 5b) Child check (list) -> Child check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child {$($child:tt)*} $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom $is_custom @attrs[$($attrs)*] @child list [@find list $($child)*] $($tail)*);
    };
    // 5c) Child check (none) -> Child check
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @check_child $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [$($already_found)*] @tag $tag @custom $is_custom @attrs[$($attrs)*] @child none [@find none] $($tail)*);
    };

    // 6) Child found. Add child to list and continue back to 0
    (
        @status [$($status:tt)*] @find list [$($already_found:tt)*]
        @tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @child $child_type:ident [$($children:tt)*] $($tail:tt)*
    ) => {
        jade_like! (@status [$($status)*] @find list [[@tag $tag @custom $is_custom @attrs[$($attrs)*] @child $child_type [$($children)*]] $($already_found)*] $($tail)*);
    };


    /*
        Rendering Time
    */

    // Flatten lists
    (@render list @status[$parent:ident] [$($already_found:tt)*]) => {{
        $(jade_like!(@render one @status [from_list $parent] $already_found);)*
    }};
    // Add a child to the parent
    (@render one @status[from_list $parent:ident] [$($already_found:tt)*]) => {
        $parent.add_child(jade_like!(@render one @status[plain] [$($already_found)*]));
    };
    // Add a child to the parent
    (@render one @status[single $parent:ident] [$($already_found:tt)*]) => {
        $parent.set_child(jade_like!(@render one @status[plain] [$($already_found)*]));
    };
    // Render plainly
    (@render one @status[plain] [@tag $tag:ident @custom $is_custom:ident @attrs[$($attrs:tt)*] @child $child_type:ident [$($children:tt)*]]) => {{
        let parent = jade_like_attrs!($tag @custom $is_custom [$($attrs)*]);
        jade_like!(@status [parent] $($children)*);
        parent.to_node()
    }};



    // Template for processing
//    ($node:ident @__processing [$attr:ident = $value:expr, $($attrs:tt)*] @__required[$($r:tt)*] @__optional [$($o:tt)*] @__children{$($c:tt)*}) => {
//        jade_like!($node @__processing [$($attrs)*] @__required[$($r)*] @__optional [$($o)*] @__children{$($c)*})
//    };
}

//    rep!(div a, b, c);
//    rep!(div a="11" , c=Some("test"), b="another");
//    render!(div[a="test", b="see", c=Some("some")][/])
//    close!(/);
//    built_in_node!(<div a="testing a", b="another test", c*="CCCCC");
//    built_in_node!(@__working div  ;  ; c*="ccc", d*=11, b="42" ,a="string", @__attrs_done);
//    built_in_node!(@__working div  ;  ; a="ccc", d*=11, b="42" ,c*="string", @__attrs_done);

macro_rules! jade {
    ($($all:tt)*) => {
        jade_like!(@status[plain] @find one $($all)*)
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
struct Div {
    a: String,
    b: String,
    c: Option<String>,
    d: Option<u32>,
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

fn main() {
    trace_macros!(true);
//    jade_like_attrs!(div [a="testing a", b="another!", c=>"Actually a Some value", d=>11,]);

    let t = jade!(
        div [a="testing a".to_string(), b="another!".to_string(), c=>"Actually a Some value".to_string(), d=>11,]
        {
            cust:[a="w".to_string(), b="c".to_string(),]
            div[a="2".to_string(), b="2".to_string(),]
        }
    );
    trace_macros!(false);
}
