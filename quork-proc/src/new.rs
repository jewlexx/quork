use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Ident};

pub fn derive_new(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(v) => &v.fields,
        _ => return macro_error!("Can only be derived on a struct"),
    };

    let (con, args): (Vec<_>, Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field
                .ident
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or_else(|| i.to_string());

            let ty = &field.ty;

            (quote!(#ident), quote!(#ident: #ty))
        })
        .unzip();

    quote! {
        impl #name {
            pub fn new(#(#args,)*) -> Self {
                Self {
                    #(#con,)*
                }
            }
        }
    }
}
