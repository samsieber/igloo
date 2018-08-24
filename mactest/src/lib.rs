#![feature(proc_macro_mod)]
#![feature(proc_macro_gen)]
#![feature(use_extern_macros)]

extern crate igloo_macro;
extern crate iceblock;

use igloo_macro::igloo_ast;
use iceblock::*;

#[igloo_ast]
pub mod ast{
    #[has_children]
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Div {
        pub name: Option<String>,
    }

    #[has_child]
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Cont {
    }

    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct P {
       pub text: String,
    }

    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Button {
        pub text: String,
    }

    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Input {
        pub prompt: String,
        pub value: Option<String>,
    }
}

use ast::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let base = Node::Div(
            Div{ name: None },
            vec!(
                Node::P(P{text:"Hello World".to_string()}),
                Node::P(P{text:"Hello World 2".to_string()}),
                Node::Div(Div{ name: None },
                vec!(
                    Node::P(P{text:"111".to_string()}),
                    Node::P(P{text:"222".to_string()}),
                    Node::P(P{text:"333".to_string()}),
                    Node::Div(Div{ name: None },
                              vec!(
                                  Node::P(P{text:"111".to_string()}),
                                  Node::P(P{text:"222".to_string()}),
                                  Node::P(P{text:"333".to_string()}),
                              ))
                )),
            )
        );
        let new =  Node::Div(
            Div{ name: Some("Pooh".to_string()) },
            vec!(
                Node::P(P{text:"Hello World".to_string()}),
                Node::P(P{text:"Hello World 2".to_string()}),
                Node::Div(Div{ name: None },
                vec!(
                    Node::P(P{text:"111".to_string()}),
                    Node::P(P{text:"222".to_string()}),
                    Node::P(P{text:"333".to_string()}),
                    Node::Div(Div{ name: None },
                              vec!(
                                  Node::P(P{text:"111".to_string()}),
                                  Node::P(P{text:"222".to_string()}),
                                  Node::P(P{text:"333".to_string()}),
                              ))
                )),
            )
        );
        let out = base.clone().diff(&new);
        println!("{:#?}", out);
    }
}
