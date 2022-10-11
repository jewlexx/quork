use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

enum TimeFormat {
    Seconds,
    Milliseconds,
    Nanoseconds,
}

pub fn time_inner(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let time_format: u8 = match attrs.to_string().as_str() {
        "s" | "seconds" | "" => TimeFormat::Seconds,
        "ms" | "milliseconds" => TimeFormat::Milliseconds,
        "ns" | "nanoseconds" => TimeFormat::Nanoseconds,
        _ => {
            return quote! { compile_error!("attributes can only be s/ms/ns for seconds, milliseconds and nanoseconds respectively") }
        }
    } as u8;

    let input: syn::ItemFn = match syn::parse2(item.clone()) {
        Ok(input) => input,
        Err(error) => return crate::error::token_stream_with_error(item, error),
    };

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let body = &*input.block;
    let ret = &input.sig.output;
    let args = &input.sig.inputs;
    let is_const = &input.sig.constness;
    let is_unsafe = &input.sig.unsafety;
    let is_async = &input.sig.asyncness;

    if !inputs.is_empty() {
        return quote_spanned! {
            inputs.span() => compile_error!("the main function cannot have any arguments");
        };
    }

    let output = quote! {
        #is_async #is_const #is_unsafe fn #name(#args) #ret {
            use std::time::Instant;
            let start = Instant::now();
            let ret = {
                #body
            };

            let elapsed = start.elapsed();

            match #time_format {
                1 => {
                    println!("{}ms", elapsed.as_millis());
                },
                2 => {
                    println!("{}ns", elapsed.as_nanos());
                },
                _ => {
                    println!("{}.{}s", elapsed.as_secs(), elapsed.subsec_millis());
                },
            };

            ret
        }
    };

    output
}
