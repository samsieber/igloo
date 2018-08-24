use syn::ItemStruct;

pub enum Children {
    None,
    One,
    List,
}

pub struct AstType {
    pub ast: ItemStruct,
    pub differ: ItemStruct,
    pub children: Children,
    pub name: String
}