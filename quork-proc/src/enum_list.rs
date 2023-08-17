use proc_macro2::{Ident, Span, TokenStream};
use syn::{Data, DeriveInput};

use proc_macro_crate::{crate_name, FoundCrate};

pub fn enum_list(ast: DeriveInput) -> TokenStream {
    let found_crate = crate_name("quork").expect("quork is present in `Cargo.toml`");

    let trait_ident = match found_crate {
        FoundCrate::Itself => quote!(crate::traits::list::ListVariants),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::traits::list::ListVariants )
        }
    };

    let variants: Vec<()> = match &ast.data {
        Data::Enum(enum_data) => {
            vec![]
        }
        _ => proc_macro_error::abort_call_site!("Can only be derived on an enum"),
    };

    let ident = &ast.ident;

    let variants = variants.len();

    quote::quote! {
        impl #trait_ident<#variants> for #ident {
            const VARIANTS: [Self;#variants] = [];
        }
    }
}
