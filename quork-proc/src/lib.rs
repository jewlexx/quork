use proc_macro_error::proc_macro_error;
use strip::strip_inner;
use syn::{parse_macro_input, DeriveInput, LitStr};

mod const_str;
#[macro_use]
mod error;
mod from_tuple;
mod new;
mod strip;
mod time_fn;

#[macro_use]
extern crate quote;

/// Implement `const_to_string` for enum variants.
///
/// Converts an enum variant to a string literal, within a constant context.
#[proc_macro_derive(ConstStr)]
pub fn derive_const_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    const_str::into_const_str(ast).into()
}

/// Implement `new` fn for structs
///
/// Will follow the form of `new(field: Type, ...) -> Self`, where all fields are required.
#[proc_macro_derive(New)]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    new::derive_new(ast).into()
}

#[proc_macro_derive(FromTuple)]
#[proc_macro_error]
pub fn derive_from_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    from_tuple::derive_from_tuple(ast).into()
}

#[proc_macro_attribute]
pub fn time(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    time_fn::time_inner(args.into(), input.into()).into()
}

#[proc_macro]
pub fn strip_lines(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip_inner(literal, strip::Alignment::None).into()
}

#[proc_macro]
pub fn rstrip_lines(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip_inner(literal, strip::Alignment::Right).into()
}

#[proc_macro]
pub fn lstrip_lines(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    strip_inner(literal, strip::Alignment::Left).into()
}
