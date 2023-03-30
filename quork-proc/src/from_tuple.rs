use proc_macro_error::abort_call_site;
use syn::{DeriveInput, Ident, Type};

pub fn derive_from_tuple(input: DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let data = &input.data;

    let mut fields = Vec::new();

    match data {
        syn::Data::Struct(data) => {
            for field in &data.fields {
                fields.push((field.ident.clone(), field.ty.clone()));
            }
        }
        _ => abort_call_site!("Should only be derived from a struct"),
    }

    let (idents, types): (Vec<Option<Ident>>, Vec<Type>) = fields.into_iter().unzip();

    quote! {
        impl #ident {
            fn from_tuple() {
                unimplemented!()
            }
        }
    }
}
