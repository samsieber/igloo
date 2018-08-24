use syn::Type;
use syn::Path;
use proc_macro2::Span;
use syn::Ident;
use syn::GenericArgument;
use syn::AngleBracketedGenericArguments;
use syn::PathSegment;
use syn::TypePath;
use syn::PathArguments;
use syn::token::{Lt, Gt, Colon2};

pub fn wrap_in_option(ty: &Type) -> Type{
    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: Some(Colon2::new(Span::call_site())),
            segments: vec!(
                PathSegment {
                    ident: Ident::new("std", Span::call_site()),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: Ident::new("option", Span::call_site()),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: Ident::new("Option", Span::call_site()),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments{
                        colon2_token:None,
                        lt_token: Lt::new(Span::call_site()),
                        args: ::util::to_punctuated_by_commas(vec!(GenericArgument::Type(ty.clone()))),
                        gt_token:  Gt::new(Span::call_site()),
                    }),
                }
            ).into_iter().collect()
        },
    })
}