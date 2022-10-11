use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub fn derive_new(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(v) => &v.fields,
        _ => return macro_error!("Can only be derived on a struct"),
    };

    quote! {
        impl #name {
            pub fn new(#fields) -> Self {
                Self {
                    #fields
                }
            }
        }
    }
}
