use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort_call_site, emit_error};
use syn::{spanned::Spanned, Data, DeriveInput};

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

    let ident = &ast.ident;

    let mut variants_have_fields = false;

    let variants = match &ast.data {
        Data::Enum(enum_data) => {
            let variants = enum_data.variants.iter();

            variants
                .map(|var| {
                    let var_ident = &var.ident;
                    if !var.fields.is_empty() {
                        variants_have_fields = true;
                        emit_error!(var.span(), "This variant has one or more fields")
                    }

                    quote! ( #ident::#var_ident )
                })
                .collect::<Vec<_>>()
        }
        _ => proc_macro_error::abort_call_site!("Can only be derived on an enum"),
    };

    if variants_have_fields {
        abort_call_site!("No variants can have fields");
    }

    let variants_size = variants.len();

    quote::quote! {
        impl #trait_ident<#variants_size> for #ident {
            const VARIANTS: [Self;#variants_size] = [#(#variants,)*];
        }
    }
}
