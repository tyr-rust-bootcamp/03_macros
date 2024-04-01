mod enum_from;
mod enum_from_darling;

use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;
use proc_macro::TokenStream;

/// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    process_enum_from_darling(input).into()
}
