use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub fn derive(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(v) => &v.fields,
        _ => return macro_error!("Can only be derived on a struct"),
    };

    let (con, args): (Vec<_>, Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let ident: TokenStream = field
                .ident
                .as_ref()
                .map_or_else(|| format!("con{i}"), std::string::ToString::to_string)
                .parse()
                .unwrap();

            let ty = &field.ty;

            (quote!(#ident), quote!(#ident: #ty))
        })
        .unzip();

    quote! {
        impl #name {
            pub const fn new(#(#args,)*) -> Self {
                Self {
                    #(#con,)*
                }
            }
        }
    }
}
