use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, token, DeriveInput, ExprLit, Lit, Variant};

fn ignore_variant(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| match attr.meta {
        syn::Meta::Path(ref p) => p.is_ident("ignore"),
        _ => abort!(
            attr.span(),
            "Expected path-style (i.e #[ignore]), found other style attribute macro"
        ),
    })
}

struct StrippedData {
    ident: Ident,
    variants: Vec<TokenStream>,
}

pub fn strip_enum(ast: &DeriveInput) -> TokenStream {
    let struct_name = {
        let original_ident = &ast.ident;
        let og_ident_span = original_ident.span();
        Ident::new(&format!("{}Hooks", original_ident), og_ident_span)
    };

    let data = &ast.data;
    let attrs = &ast.attrs;

    let info: StrippedData = match data {
        syn::Data::Enum(ref e) => {
            let variants = e
                .variants
                .iter()
                .filter_map(|variant| {
                    if ignore_variant(variant) {
                        None
                    } else {
                        Some(variant.ident.to_token_stream())
                    }
                })
                .collect::<Vec<_>>();

            let new_ident = if let Some(info_attr) = attrs
                .iter()
                .find(|attr| attr.path().is_ident("stripped_ident"))
            {
                match info_attr.meta {
                    syn::Meta::NameValue(name_value) => {
                        let ident = name_value.value;

                        if let syn::Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }) = ident
                        {
                            Ident::new(
                                &string.value().replace("{}", &ast.ident.to_string()),
                                ast.ident.span(),
                            )
                        } else {
                            abort!(ident.span(), "Expected string literal.")
                        }
                    }
                    _ => abort!(
                        info_attr.span(),
                        "Expected #[stripped_ident = \"\"]. Found other style attribute."
                    ),
                }
            } else {
                ast.ident
            };

            StrippedData {
                ident: new_ident,
                variants,
            }
        }
        _ => abort_call_site!("`Strip` can only be derived for enums"),
    };

    let command_names = info
        .variants
        .iter()
        .map(|variant| heck::AsKebabCase(variant.to_string()).to_string())
        .collect::<Vec<_>>();

    let strum = match crate_name("strum").expect("strum is present in `Cargo.toml`") {
        FoundCrate::Itself => Ident::new("strum", Span::call_site()),
        FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    };

    quote! {
        // TODO: Better way of doing this? or add support for meta in proc macro
        #[derive(Debug, Clone, #strum::Display, #strum::IntoStaticStr, #strum::EnumIter, PartialEq, Eq)]
        #[strum(serialize_all = "kebab-case")]
        pub enum #struct_name {
            #(#variants),*
        }

        impl From<String> for #struct_name {
            fn from(string: String) -> Self {
                match string.as_str() {
                    #(#command_names => #struct_name::#variants,)*
                    _ => panic!("Invalid command name: {}", string),
                }
            }
        }
    }
}