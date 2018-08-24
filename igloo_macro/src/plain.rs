use super::{attributes, types, model::{Children, AstType}};
use syn::{Ident, Item, ItemStruct};
use proc_macro2::Span;
use proc_macro::TokenStream;

// TODO pub enum Render
// TODO impl ToPlain<Node> for Render
// TODO pub enum Request
// TODO impl ToRender<Request>


pub fn process_struct(item: ItemStruct) -> AstType {
    let attrs = item.attrs;
    let (attrs, has_children) = attributes::remove(attrs, "has_children");
    let (attrs, has_child) =  attributes::remove(attrs, "has_child");

    let vis = item.vis;
    let struct_token = item.struct_token;
    let ident = item.ident;
    let generics = item.generics;
    let fields = item.fields;
    let semi_token = item.semi_token;

    let tree_struct = ItemStruct {
        attrs: attrs.clone(),
        vis: vis.clone(),
        struct_token: struct_token.clone(),
        ident: ident.clone(),
        generics: generics.clone(),
        fields: fields.clone(),
        semi_token: semi_token.clone(),
    };

    let mut new_fields = fields.clone();
    new_fields.iter_mut().for_each(|ref mut f| {
        f.ty = types::wrap_in_option(&f.ty);
    });

    let diff_struct = ItemStruct {
        attrs,
        vis,
        struct_token,
        ident: Ident::new(&format!("{}Diff", ident.clone()), Span::call_site()),
        generics,
        fields: new_fields,
        semi_token,
    };

    let offspring = if has_children {
        Children::List
    } else if has_child {
        Children::One
    } else {
        Children::None
    };

    AstType{
        ast: tree_struct,
        differ: diff_struct,
        children: offspring,
        name: ident.to_string()
    }
}

pub fn gen_enum(types: &Vec<AstType>) -> Vec<Item> {
    let node_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(#name(#name))
            },
            Children::One => {
                quote!(#name(#name, ::std::boxed::Box<Node>))
            },
            Children::List => {
                quote!(#name(#name, ::std::vec::Vec<Node>))
            },
        }
    });

    let node_diff_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();
        let diff_name = a.differ.ident.clone();

        match a.children {
            Children::None => {
                quote!(#name(#diff_name))
            },
            Children::One => {
                quote!(#name(#diff_name, Box<::iceblock::NodeChange<Node, NodeDiff>>))
            },
            Children::List => {
                quote!(#name(#diff_name, Vec<::iceblock::ChildChange<Node, NodeDiff>>))
            },
        }
    });

    let quotes = vec!(
        quote! {
            #[derive(Debug)]
            pub enum NodeDiff {
                #(#node_diff_entries),*
            }
        },

        quote!  {
            #[derive(Clone, Debug)]
            pub enum Node {
                #(#node_entries),*
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

pub fn gen_code(types: &Vec<AstType>) -> Vec<Item> {
    let impl_arms = types.iter().map(|ast|{
        let name = ast.ast.ident.clone();
        match ast.children {
            Children::None => {
                quote!(
                    Node::#name(props) => {
                        match other {
                            Node::#name(old_props) => {
                                if *props == old_props {
                                    ::iceblock::NodeChange::Identical
                                } else {
                                    ::iceblock::NodeChange::Changed(NodeDiff::#name(props.diff(old_props)))
                                }
                            }
                            _ => ::iceblock::NodeChange::Swapped(Node::#name(props.clone()), other)
                        }
                    }
                )
            },
            Children::One => {
                quote!(
                    Node::#name(props, child) => {
                        match other {
                            Node::#name(old_props, old_child) => {
                                if *props == old_props {
                                    ::iceblock::NodeChange::Identical
                                } else {
                                    ::iceblock::NodeChange::Changed(NodeDiff::#name(
                                        props.diff(old_props),
                                        ::std::boxed::Box::new(child.diff(*old_child)),
                                    ))
                                }
                            },
                            _ => ::iceblock::NodeChange::Swapped(Node::#name(props.clone(), child.clone()), other)
                        }
                    }
                )
            },
            Children::List => {
                quote!(
                    Node::#name(props, children) => {
                        match other {
                            Node::#name(old_props, old_children) => {
                                let compared = compare_children(old_children, children);
                                if compared.len() == 0 && *props == old_props {
                                    NodeChange::Identical
                                } else {
                                    NodeChange::Changed(NodeDiff::#name(
                                        props.diff(old_props),
                                        compared,
                                    ))
                                }
                            },
                            _ => ::iceblock::NodeChange::Swapped(Node::#name(props.clone(), children.clone()), other)
                        }
                    }
                )
            }
        }
    });

    let mut supports : Vec<_> = vec!(
        quote!(impl ::iceblock::Diffable for Node {
            type Diff = NodeDiff;

            fn diff(&self, other: Node) -> ::iceblock::NodeChange<Node, NodeDiff> {
                match self {
                    #(#impl_arms),*
                }
            }
        }),
    );
    let mut differs : Vec<_> = types.iter().map(|ast| {
        let name = ast.ast.ident.clone();
        let diff_name = ast.differ.ident.clone();

        let field : Vec<_> = ast.ast.fields.iter().map(|f| {
            f.ident.as_ref().expect("Doesn't work with tuple structs yet")
        }).collect();
        let from_self = field.clone();
        let from_other = field.clone();

        quote!(
            impl #name {
                pub fn diff(&self, other: #name) -> #diff_name {
                    #diff_name {
                        #(#field: ::iceblock::diff(&self.#from_self, &other.#from_other)),*
                    }
                }
            }
        )
    }).collect();


    let mut quotes = vec!();
    quotes.append(&mut supports);
    quotes.append(&mut differs);

    quotes.into_iter().map(|quoted| {
        let tokens : TokenStream = quoted.into();
        println!("{:?}", tokens.to_string());
        let item : Item = ::syn::parse(tokens).unwrap();
        item
    }).collect()
}