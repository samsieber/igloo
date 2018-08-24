use model::AstType;
use syn::Item;
use model::Children;
use proc_macro::TokenStream;

pub fn gen_request_enum(types: &Vec<AstType>) -> Vec<Item> {
    let request_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(#name(#name))
            },
            Children::One => {
                quote!(#name(#name, ::std::boxed::Box<Request>))
            },
            Children::List => {
                quote!(#name(#name, ::std::vec::Vec<Request>))
            },
        }
    });

    let quotes = vec!(
        quote! {
            #[derive(Debug)]
            pub enum Request {
                #(#request_entries),*,
                Custom(Box<RequestedComponent<Render, Node, NodeDiff>>)
            }
        },
    );

    quotes.into_iter().map(|quoted| {
        let tokens : TokenStream = quoted.into();
        println!("{:?}", tokens.to_string());
        let item : Item = ::syn::parse(tokens).unwrap();
        item
    }).collect()
}

pub fn gen_request_impl(types: &Vec<AstType>) -> Vec<Item> {
    let render_new_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(Request::#name(props) => Render::#name(props))
            },
            Children::One => {
                quote!(Request::#name(props, child) => Render::#name(props, Box::new(child.render_new())))
            },
            Children::List => {
                quote!(Request::#name(props, children) => Render::#name(props, children.into_iter().map(|v| v.render_new()).collect()))
            },
        }
    });

    let render_update_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(
                     Request::#name(props) => {
                        let diff = match old {
                            Render::#name(other_props) => {
                                if props == other_props {
                                    NodeChange::Identical
                                } else {
                                    NodeChange::Changed(NodeDiff::#name(
                                        props.diff(other_props),
                                    ))
                                }
                            },
                            other => {
                                NodeChange::Swapped(Node::#name(props.clone()), other.to_plain())
                            },
                        };

                        RenderResult {
                            render: Render::#name(props),
                            diff,
                        }
                    }
                )
            },
            Children::One => {
                quote!(
                    Request::#name(props, child) => {
                        match old {
                            Render::#name(other_props, other_child) => {
                                let RenderResult{
                                    render: child_render,
                                    diff: child_diff,
                                } = child.render_update(*other_child, false);

                                if props == other_props && child_diff.is_identical(){
                                    RenderResult {
                                        render: Render::#name(props, Box::new(child_render)),
                                        diff: NodeChange::Identical,
                                    }
                                } else {
                                    let diff = NodeChange::Changed(NodeDiff::#name(
                                        props.diff(other_props),
                                        ::std::boxed::Box::new(child_diff),
                                    ));
                                    RenderResult {
                                        render: Render::#name(props, Box::new(child_render)),
                                        diff,
                                    }
                                }
                            }
                            other => {
                                let render = Render::#name(props, Box::new(child.render_new()));
                                let diff = other.to_plain().diff(render.to_plain());
                                RenderResult { render, diff }
                            },
                        }
                    }
                )
            },
            Children::List => {
                quote!(
                    Request::#name(props, children) => {
                        match old {
                            Render::#name(other_props, old_children) => {
                                let (children_render, children_changes) = compare_render_children(children, old_children, compatible);
                                if props == other_props && children_changes.is_empty() {
                                    RenderResult {
                                        render: Render::#name(props, children_render),
                                        diff: NodeChange::Identical,
                                    }
                                } else {
                                    let diff = NodeChange::Changed(NodeDiff::#name(props.diff(other_props), children_changes));
                                    RenderResult {
                                        render: Render::#name( props, children_render),
                                        diff,
                                    }
                                }
                            },
                            other => {
                                let render = Render::#name(props, children.into_iter().map(|c| c.render_new()).collect());
                                let diff = other.to_plain().diff(render.to_plain());
                                RenderResult { render, diff }
                            },
                        }
                    }
                )
            },
        }
    });

    let quotes = vec!(
        quote! {
            impl ToRender<Render, Node, NodeDiff> for Request {
                fn render_new(self) -> Render{
                    match self {
                        #(#render_new_entries),*,
                        Request::Custom(component) => Render::Custom(Box::new(component.render_new_box())),
                    }
                }

                fn render_update(self, old: Render, compatible: bool) -> RenderResult<Render, Node, NodeDiff> {
                    match self {
                        #(#render_update_entries),*,
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
        },
    );

    quotes.into_iter().map(|quoted| {
        let tokens : TokenStream = quoted.into();
        println!("{:?}", tokens.to_string());
        let item : Item = ::syn::parse(tokens).unwrap();
        item
    }).collect()
}

pub fn gen_request_builder(types: &Vec<AstType>) -> Vec<Item> {
    let quotes = vec!(
        quote! {
            use std::fmt::Debug;
        },
        quote! {
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
        },
    );

    quotes.into_iter().map(|quoted| {
        let tokens : TokenStream = quoted.into();
        println!("{:?}", tokens.to_string());
        let item : Item = ::syn::parse(tokens).unwrap();
        item
    }).collect()
}