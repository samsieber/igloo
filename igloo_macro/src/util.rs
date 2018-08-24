use syn::punctuated::Punctuated;
use syn::token::Comma;

pub fn to_punctuated_by_commas<T>(items: Vec<T>) -> Punctuated<T, Comma> {
    let mut p = Punctuated::new();
    for item in items {
        p.push(item);
    }
    p
}