mod const_str;
mod error;
mod new;
mod time_fn;

/// Implement `const_to_string` for enum variants.
///
/// Converts an enum variant to a string literal, within a constant context.
#[proc_macro_derive(ConstStr)]
pub fn derive_const_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const_str::into_const_str(input.into()).into()
}

/// Implement `new` fn for structs
///
/// Will follow the form of `new(field: Type, ...) -> Self`, where all fields are required.
#[proc_macro_derive(New)]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    new::derive_new(input.into()).into()
}

#[proc_macro_attribute]
pub fn time(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    time_fn::time_inner(args.into(), input.into()).into()
}
