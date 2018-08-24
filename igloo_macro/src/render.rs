use model::AstType;
use syn::Item;
use model::Children;
use proc_macro::TokenStream;

pub fn gen_render(types: &Vec<AstType>) -> Vec<Item> {
    let render_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(#name(#name))
            },
            Children::One => {
                quote!(#name(#name, ::std::boxed::Box<Render>))
            },
            Children::List => {
                quote!(#name(#name, ::std::vec::Vec<Render>))
            },
        }
    });

    let to_plain_entries = types.iter().map(|a| {
        let name = a.ast.ident.clone();

        match a.children {
            Children::None => {
                quote!(Render::#name(props) => Node::#name(props.clone()))
            },
            Children::One => {
                quote!(Render::#name(props, child) => {
                            Node::#name(props.clone(), Box::new(child.to_plain()))
                        })
            },
            Children::List => {
                quote!(Render::#name(props, children) => {
                            Node::#name(props.clone(), children.iter().map(|c| c.to_plain()).collect())
                        })
            },
        }
    });

    let quotes = vec!(
        quote! {
            #[derive(Debug)]
            pub enum Render {
                #(#render_entries),*,
                Custom(Box<RenderedComponent<Render, Node, NodeDiff>>),
            }
        },

        quote!  {
            impl ToPlain<Node> for Render {
                fn to_plain(&self) -> Node {
                    match &self {
                        #(#to_plain_entries),*,
                        Render::Custom(component) => {
                            component.rendered.to_plain()
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