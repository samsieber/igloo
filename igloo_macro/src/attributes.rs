use syn::Attribute;
use syn::MetaNameValue;
use syn::Meta;
use syn::MetaList;
use syn::AttrStyle;

pub fn remove(attrs: Vec<Attribute>, name: &str) -> (Vec<Attribute>, bool) {
    let count = attrs.len();
    let items: Vec<Attribute> = attrs.into_iter().filter(|i| {!has(i, name)}).collect();
    let different = count != items.len();
    (items, different)
}

fn has(attr: &Attribute, name: &str) -> bool {
    match attr.style {
        AttrStyle::Outer =>  {
            match attr.interpret_meta() {
                Some(meta) => {
                    match meta {
                        Meta::Word(ident) => { ident.to_string() == name }
                        Meta::List(MetaList{ident, ..}) => { ident.to_string() == name }
                        Meta::NameValue(MetaNameValue{ident, ..}) => { ident.to_string() == name }
                    }
                },
                None => false,
            }
        },
        _ => false,
    }
}