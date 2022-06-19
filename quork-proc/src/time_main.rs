use proc_macro::TokenStream;
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
        _ => return quote! { compile_error!("attributes can only be s/ms/ns for seconds, milliseconds and nanoseconds respectively") }.into(),
    } as u8;

    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(input) => input,
        Err(error) => return crate::error::token_stream_with_error(item, error),
    };

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let body = &*input.block;
    let ret = &input.sig.output;

    if name != "main" {
        return quote_spanned! {
            name.span() => compile_error!("#[time] can only be applied to the main function");
        }
        .into();
    }

    if !inputs.is_empty() {
        return quote_spanned! {
            inputs.span() => compile_error!("the main function cannot have any arguments");
        }
        .into();
    }

    let output = quote! {
        fn main() #ret {
            use std::time::{Duration, Instant};
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

    output.into()
}
