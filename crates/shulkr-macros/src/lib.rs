use proc_macro::TokenStream;
use syn::{DeriveInput, ItemImpl, parse_macro_input};

mod data_type;
mod enumeration;
mod property_enum;
mod static_registry;

#[proc_macro_derive(Enumeration)]
pub fn derive_enumeration(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    enumeration::expand(input).into()
}

#[proc_macro_derive(DataType)]
pub fn derive_data_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    data_type::expand(input).into()
}

#[proc_macro_derive(PropertyEnum)]
pub fn derive_property_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    property_enum::expand(input).into()
}

#[proc_macro_attribute]
pub fn static_registry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    static_registry::expand(input).into()
}
