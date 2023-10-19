use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, DeriveInput, Expr, ExprLit, Lit, Variant};

fn ignore_variant(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| match attr.meta {
        syn::Meta::Path(ref p) => p.is_ident("stripped_ignore"),
        _ => abort!(
            attr.span(),
            "Expected path-style (i.e #[stripped_ignore]), found other style attribute macro"
        ),
    })
}

struct StrippedData {
    ident: Ident,
    variants: Vec<TokenStream>,
    meta: Vec<Expr>,
}

// struct MetaArgs {
//     meta: Vec<Expr>,
// }

// impl Parse for MetaArgs {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//          input.peek3(token)
//     }
// }

pub fn strip_enum(ast: &DeriveInput) -> TokenStream {
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
                match &info_attr.meta {
                    syn::Meta::NameValue(name_value) => {
                        let ident = &name_value.value;

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
                Ident::new(
                    &format!("{}Stripped", ast.ident.to_string()),
                    ast.ident.span(),
                )
            };

            let meta = attrs
                .iter()
                .filter(|attr| attr.path().is_ident("stripped_meta"))
                .map(|attr| match &attr.meta {
                    // TODO: Add inherit metadata
                    syn::Meta::List(meta) => meta.parse_args().expect("single meta attribute"),
                    _ => abort!(
                        attr.span(),
                        "Expected #[stripped_meta(...)]. Found other style attribute."
                    ),
                })
                .collect();

            StrippedData {
                ident: new_ident,
                variants,
                meta,
            }
        }
        _ => abort_call_site!("`Strip` can only be derived for enums"),
    };

    let StrippedData {
        ident,
        variants,
        meta,
    } = info;

    quote! {
        #(#[#meta])*
        pub enum #ident {
            #(#variants),*
        }
    }
}
