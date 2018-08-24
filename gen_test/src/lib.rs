extern crate iceblock;
extern crate core;

pub mod ast {
    use ::iceblock::*;
    use std::fmt::Debug;

    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Div {
        pub name: Option<String>,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct DivDiff {
        pub name: ::std::option::Option<Option<String>>,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Cont {}
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct ContDiff {}
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct P {
        pub text: String,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct PDiff {
        pub text: ::std::option::Option<String>,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Button {
        pub text: String,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct ButtonDiff {
        pub text: ::std::option::Option<String>,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct Input {
        pub prompt: String,
        pub value: Option<String>,
    }
    #[derive(PartialEq, PartialOrd, Clone, Debug)]
    pub struct InputDiff {
        pub prompt: ::std::option::Option<String>,
        pub value: ::std::option::Option<Option<String>>,
    }

    #[derive(Clone, Debug)]
    pub enum Node {
        Div(Div, ::std::vec::Vec<Node>),
        Cont(Cont, ::std::boxed::Box<Node>),
        P(P),
        Button(Button),
        Input(Input),
    }

    #[derive(Debug)]
    pub enum NodeDiff {
        Div(DivDiff, Vec<ChildChange<Node, NodeDiff>>),
        Cont(ContDiff, Box<NodeChange<Node, NodeDiff>>),
        P(PDiff),
        Button(ButtonDiff),
        Input(InputDiff),
    }

    #[derive(Debug)]
    pub enum Render {
        Div(Div, ::std::vec::Vec<Render>),
        Cont(Cont, ::std::boxed::Box<Render>),
        P(P),
        Button(Button),
        Input(Input),
        Custom(Box<RenderedComponent<Render, Node, NodeDiff>>)
    }

    #[derive(Debug)]
    pub enum Request {
        Div(Div, ::std::vec::Vec<Request>),
        Cont(Cont, ::std::boxed::Box<Request>),
        P(P),
        Button(Button),
        Input(Input),
        Custom(Box<RequestedComponent<Render, Node, NodeDiff>>)
    }

    impl ToPlain<Node> for Render {
        fn to_plain(&self) -> Node {
            match &self {
                Render::Div(props, children) => {
                    Node::Div(props.clone(), children.iter().map(|c| c.to_plain()).collect())
                },
                Render::Cont(props, child) => {
                    Node::Cont(props.clone(), Box::new(child.to_plain()))
                },
                Render::P(props) => Node::P(props.clone()),
                Render::Button(props) => Node::Button(props.clone()),
                Render::Input(props) => Node::Input(props.clone()),
                Render::Custom(component) => {
                    component.rendered.to_plain()
                }
            }
        }
    }

    impl ToRender<Render, Node, NodeDiff> for Request {
        fn render_new(self) -> Render{
            match self {
                Request::Div(props, children) => Render::Div(props, children.into_iter().map(|v| v.render_new()).collect()),
                Request::Cont(props, child) => Render::Cont(props, Box::new(child.render_new())),
                Request::P(props) => Render::P(props),
                Request::Button(props) => Render::Button(props),
                Request::Input(props) => Render::Input(props),
                Request::Custom(component) => Render::Custom(Box::new(component.render_new_box())),
            }
        }

        fn render_update(self, old: Render, compatible: bool) -> RenderResult<Render, Node, NodeDiff> {
            match self {
                Request::Div(props, children) => {
                    match old {
                        Render::Div(other_props, old_children) => {
                            let (children_render, children_changes) = compare_render_children(children, old_children, compatible);
                            if props == other_props && children_changes.is_empty() {
                                RenderResult {
                                    render: Render::Div(props, children_render),
                                    diff: NodeChange::Identical,
                                }
                            } else {
                                let diff = NodeChange::Changed(NodeDiff::Div(props.diff(other_props), children_changes));
                                RenderResult {
                                    render: Render::Div( props, children_render),
                                    diff,
                                }
                            }
                        },
                        other => {
                            let render = Render::Div(props, children.into_iter().map(|c| c.render_new()).collect());
                            let diff = other.to_plain().diff(render.to_plain());
                            RenderResult { render, diff }
                        },
                    }
                },
                Request::Cont(props, child) => {
                    match old {
                        Render::Cont(other_props, other_child) => {
                            let RenderResult{
                                render: child_render,
                                diff: child_diff,
                            } = child.render_update(*other_child, false);

                            if props == other_props && child_diff.is_identical(){
                                RenderResult {
                                    render: Render::Cont(props, Box::new(child_render)),
                                    diff: NodeChange::Identical,
                                }
                            } else {
                                let diff = NodeChange::Changed(NodeDiff::Cont(
                                    props.diff(other_props),
                                    ::std::boxed::Box::new(child_diff),
                                ));
                                RenderResult {
                                    render: Render::Cont(props, Box::new(child_render)),
                                    diff,
                                }
                            }
                        }
                        other => {
                            let render = Render::Cont(props, Box::new(child.render_new()));
                            let diff = other.to_plain().diff(render.to_plain());
                            RenderResult { render, diff }
                        },
                    }
                },
                Request::P(props) => {
                    let diff = match old {
                        Render::P(other_props) => {
                            if props == other_props {
                                NodeChange::Identical
                            } else {
                                NodeChange::Changed(NodeDiff::P(
                                    props.diff(other_props),
                                ))
                            }
                        },
                        other => {
                            NodeChange::Swapped(Node::P(props.clone()), other.to_plain())
                        },
                    };

                    RenderResult {
                        render: Render::P(props),
                        diff,
                    }
                },
                Request::Button(props) => {
                    let diff = match old {
                        Render::Button(other_props) => {
                            if props == other_props {
                                NodeChange::Identical
                            } else {
                                NodeChange::Changed(NodeDiff::Button(
                                    props.diff(other_props),
                                ))
                            }
                        },
                        other => {
                            NodeChange::Swapped(Node::Button(props.clone()), other.to_plain())
                        },
                    };

                    RenderResult {
                        render: Render::Button(props),
                        diff,
                    }
                },
                Request::Input(props) => {
                    let diff = match old {
                        Render::Input(other_props) => {
                            if props == other_props {
                                NodeChange::Identical
                            } else {
                                NodeChange::Changed(NodeDiff::Input(
                                    props.diff(other_props),
                                ))
                            }
                        },
                        other => {
                            NodeChange::Swapped(Node::Input(props.clone()), other.to_plain())
                        },
                    };

                    RenderResult {
                        render: Render::Input(props),
                        diff,
                    }
                },
                Request::Custom(requested_component) => {
                    if compatible {
                        let render = requested_component.render_new_box();
                        let plain = render.rendered.to_plain();
                        let old_plain = old.to_plain();
                        let diff = plain.diff(old_plain);
                        RenderResult {
                            render: Render::Custom(Box::new(render)),
                            diff,
                        }
                    } else {
                        let prev = match old {
                            Render::Custom(component) => PreviousRender::OfComponent(*component),
                            render => PreviousRender::OfRender(render)
                        };

                        let rendered = requested_component.render_update_box(prev);
                        RenderResult {
                            render: Render::Custom(Box::new(rendered.rendered_component)),
                            diff: rendered.diff,
                        }
                    }
                }
            }
        }
    }


    impl Diffable for Node {
        type Diff = NodeDiff;
        fn diff(&self, old: Node) -> NodeChange<Node, NodeDiff> {
            match self {
                Node::Div(props, children) => match old {
                    Node::Div(old_props, old_children) => {
                        let compared = compare_children(old_children, children);
                        if compared.len() == 0 && *props == old_props {
                            NodeChange::Identical
                        } else {
                            NodeChange::Changed(NodeDiff::Div(
                                props.diff(old_props),
                                compared,
                            ))
                        }
                    }
                    _ => NodeChange::Swapped(Node::Div(props.clone(), children.clone()), old),
                },
                Node::Cont(props, child) => match old {
                    Node::Cont(old_props, old_child) => {
                        NodeChange::Changed(NodeDiff::Cont(
                            props.diff(old_props),
                            ::std::boxed::Box::new(child.diff(*old_child)),
                        ))
                    }
                    _ => NodeChange::Swapped(Node::Cont(props.clone(), child.clone()), old),
                },
                Node::P(props) => match old {
                    Node::P(old_props) => {
                        if *props == old_props {
                            NodeChange::Identical
                        } else {
                            NodeChange::Changed(NodeDiff::P(props.diff(old_props)))
                        }
                    }
                    _ => NodeChange::Swapped(Node::P(props.clone()), old),
                },
                Node::Button(props) => match old {
                    Node::Button(old_props) => {
                        if *props == old_props {
                            NodeChange::Identical
                        } else {
                            NodeChange::Changed(NodeDiff::Button(
                                props.diff(old_props),
                            ))
                        }
                    }
                    _ => NodeChange::Swapped(Node::Button(props.clone()), old),
                },
                Node::Input(props) => match old {
                    Node::Input(old_props) => {
                        if *props == old_props {
                            NodeChange::Identical
                        } else {
                            NodeChange::Changed(NodeDiff::Input(
                                props.diff(old_props),
                            ))
                        }
                    }
                    _ => NodeChange::Swapped(Node::Input(props.clone()), old),
                },
            }
        }
    }
    impl Div {
        pub fn diff(&self, old: Div) -> DivDiff {
            DivDiff {
                name: diff(&self.name, &old.name),
            }
        }
    }
    impl Cont {
        pub fn diff(&self, old: Cont) -> ContDiff {
            ContDiff {}
        }
    }
    impl P {
        pub fn diff(&self, old: P) -> PDiff {
            PDiff {
                text: diff(&self.text, &old.text),
            }
        }
    }
    impl Button {
        pub fn diff(&self, old: Button) -> ButtonDiff {
            ButtonDiff {
                text: diff(&self.text, &old.text),
            }
        }
    }
    impl Input {
        pub fn diff(&self, old: Input) -> InputDiff {
            InputDiff {
                prompt: diff(&self.prompt, &old.prompt),
                value: diff(&self.value, &old.value),
            }
        }
    }

    impl Request {
        pub fn custom<Comp: Debug + 'static + Renderer<Props, State, Request>, Props: PartialEq + Debug + 'static, State: Debug + Default + 'static>(component: Comp, props: Props) -> Request {
            let requested_raw : RequestedComponentImpl<Comp, Props, State, Request, Render, Node, NodeDiff> = RequestedComponentImpl{
                renderer: component,
                props,
                state_type: ::std::marker::PhantomData,
                request_type: ::std::marker::PhantomData,
                render_type: ::std::marker::PhantomData,
                node_type: ::std::marker::PhantomData,
                diff_type: ::std::marker::PhantomData,
            };

            let boxed : Box<RequestedComponent<Render, Node, NodeDiff>> = Box::new(requested_raw);

            Request::Custom(boxed)
        }
    }
}

mod trial_by_fire {
    use super::ast::*;
    use ::iceblock::*;
    use std::marker::PhantomData;
    use std::fmt::Debug;
    use core::fmt;

    #[derive(PartialEq, Debug)]
    pub struct NoProps1 {
        pub name: String,
        pub count: u32,
    }

    #[derive(PartialEq, Debug)]
    pub struct NoProps2 {
        name: String
    }


    #[derive(PartialEq, Default, Debug)]
    pub struct RootState {
    }

    #[derive(PartialEq, Default, Debug)]
    pub struct Differ {
        message: String,
    }

    pub fn render_differ(props: &NoProps2, state: &Differ) -> Request {
        Request::Div(
            Div{ name: None },
            vec!(
                Request::P(P{text: format!("Hey")}),
                Request::P(P{text: format!("{}", &state.message)}),
                Request::P(P{text: format!("{}", &props.name)}),
            ),
        )
    }

    pub fn render_root(props: &NoProps1, state: &RootState) -> Request {
        Request::Div(
            Div{ name: Some(props.name.clone()) },
            vec!(
                Request::P(P{text: format!("Hello {}", &props.name)}),
                Request::P(P{text: format!("Another message: {}", &props.count)}),
                Request::custom(wrap(render_differ), NoProps2{ name: format!("{}", &props.name)}),
            ),
        )
    }

    pub struct FnWrapper<A,B,C> {
        fun: fn(&A,&B) -> C,
    }

    pub fn wrap<A,B,C>(fun: fn(&A,&B) -> C) -> FnWrapper<A,B,C>{
        FnWrapper {
            fun,
        }
    }

    impl <A,B,C> Renderer<A,B,C> for FnWrapper<A,B,C> {
        fn render(&self, props: &A, state: &B) -> C {
            (self.fun)(props, state)
        }
    }

    impl <A,B,C> Debug for FnWrapper<A,B,C> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Custom Function Wrapper")
        }
    }
}

use ast::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::ast::*;
    use super::trial_by_fire::*;
    use iceblock::RenderResult;
    use iceblock::ToRender;
    use iceblock::ToPlain;
    use iceblock::Diffable;

    #[test]
    fn test_component_tree() {
        let request = Request::custom(wrap(render_root), NoProps1{name: "Bob".to_owned(), count: 10});
        let rendered = request.render_new();
        println!("{:#?}", &rendered);
        println!("{:#?}", rendered.to_plain());
        let request = Request::custom(wrap(render_root), NoProps1{name: "Jones".to_owned(), count: 10});
        let RenderResult {
            render: rendered, diff
        } = request.render_update(rendered, true);

        println!("{:#?}", &rendered);
        println!("{:#?}", rendered.to_plain());
        println!("{:#?}", diff);
    }

    #[test]
    fn test_node_diff() {
        let base = Node::Div(
            Div { name: None },
            vec![
                Node::P(P {
                    text: "Hello World".to_string(),
                }),
                Node::P(P {
                    text: "Hello World 2".to_string(),
                }),
                Node::Div(
                    Div { name: None },
                    vec![
                        Node::P(P {
                            text: "111".to_string(),
                        }),
                        Node::P(P {
                            text: "222".to_string(),
                        }),
                        Node::P(P {
                            text: "333".to_string(),
                        }),
                        Node::Div(
                            Div { name: None },
                            vec![
                                Node::P(P {
                                    text: "111".to_string(),
                                }),
                                Node::P(P {
                                    text: "222".to_string(),
                                }),
                                Node::P(P {
                                    text: "333".to_string(),
                                }),
                            ],
                        ),
                    ],
                ),
            ],
        );
        let new = Node::Div(
            Div {
                name: Some("Pooh".to_string()),
            },
            vec![
                Node::P(P {
                    text: "Hello World".to_string(),
                }),
                Node::P(P {
                    text: "Hello World 2".to_string(),
                }),
                Node::Div(
                    Div { name: None },
                    vec![
                        Node::P(P {
                            text: "111".to_string(),
                        }),
                        Node::P(P {
                            text: "222".to_string(),
                        }),
                        Node::P(P {
                            text: "333".to_string(),
                        }),
                        Node::Div(
                            Div { name: None },
                            vec![
                                Node::P(P {
                                    text: "111".to_string(),
                                }),
                                Node::P(P {
                                    text: "222".to_string(),
                                }),
                                Node::P(P {
                                    text: "333".to_string(),
                                }),
                            ],
                        ),
                    ],
                ),
            ],
        );
        let out = new.clone().diff(base);
        println!("{:#?}", out);
    }
}
