#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{Item, ItemMod, token::{Brace}};

use model::AstType;

mod util;
mod attributes;
mod types;
mod model;
mod plain;
mod render;
mod request;

#[proc_macro_attribute]
pub fn igloo_ast(_args: TokenStream, input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    // Parse the string representation
    let ast : Item = syn::parse(input).unwrap();
    println!("Here!");
    // Build the impl
    let out = impl_modification(ast);

    println!("New AST: ------------ \n{}\n----------", out.to_string());

    out
}

fn impl_modification(ast: Item) -> TokenStream {
    match ast {
        Item::Mod(item_mod) => {
            let new_content = filter_content(item_mod.content);
            let new_ast = Item::Mod(ItemMod {
                attrs : vec!(),//TODO: Fix this to use all but the ones I used;
                mod_token: item_mod.mod_token,
                vis : item_mod.vis,
                ident: item_mod.ident,
                content: new_content,
                semi: item_mod.semi,
            });
            quote!(#new_ast).into()
        },
        _ => panic!("Must be declared on a module"),
    }
}

fn filter_content(ast: Option<(Brace, Vec<Item>)>) -> Option<(Brace, Vec<Item>)>{
    match ast {
        None => panic!("Must be declared on an inline module"),
        Some((brace, items)) => {
            let ast_types : Vec<AstType>= items.into_iter().filter_map(|i| {
                match i {
                    Item::Struct(item_struct) => {
                        Some(plain::process_struct(item_struct))
                    }
                    _ => None
                }
            }).collect();

            let mut node_stuff = plain::gen_enum(&ast_types);
            let mut code_stuff = plain::gen_code(&ast_types);
            let mut render_stuff = render::gen_render(&ast_types);
            let mut request_enum = request::gen_request_enum(&ast_types);
            let mut request_impl = request::gen_request_impl(&ast_types);
            let mut request_builder = request::gen_request_builder(&ast_types);

            let mut new_items : Vec<Item> = ast_types.iter()
                .flat_map(|t| {
                    vec!(
                        Item::Struct(t.ast.clone()),
                        Item::Struct(t.differ.clone()),
                    ).into_iter()
                })
                .collect();
            new_items.push(::syn::parse(quote!(use ::iceblock::*;).into()).unwrap());
            new_items.append(&mut node_stuff);
            new_items.append(&mut code_stuff);
            new_items.append(&mut render_stuff);
            new_items.append(&mut request_enum);
            new_items.append(&mut request_impl);
            new_items.append(&mut request_builder);

            Some((brace, new_items))
        }
    }
}



