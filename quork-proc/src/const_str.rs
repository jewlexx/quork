use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn into_const_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let vars = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => panic!("Can only be derived on an enum"),
    };

    let mut arms = Vec::<TokenStream>::new();

    for var in vars.iter() {
        let var_name = &var.ident;

        let params = match var.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        let arm = quote! { #name::#var_name #params => stringify!(#var_name) };

        arms.push(arm);
    }

    let v = quote! {
        impl #name {
            pub const fn const_to_string(&self) -> &'static str {
                match *self {
                    #(#arms),*
                }
            }
        }
    };

    v.into()
}
