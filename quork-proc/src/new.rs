use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub fn derive_new(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vars = match &ast.data {
        Data::Struct(v) => &v.variants,
        _ => return macro_error!("Can only be derived on a struct"),
    };

    quote!()
}
