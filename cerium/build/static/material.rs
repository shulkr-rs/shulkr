use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::Ident;

use crate::object::StaticObjectBuilder;
use crate::write_file;

pub fn generate() {
    println!("cargo:rerun-if-changed=build_assets/item.json");

    let object = StaticObjectBuilder::new("Material")
        .with_json(include_str!("../../build_assets/item.json").to_owned())
        .with_init(generate_init)
        .build();

    let tokens = object.generate();

    write_file(&tokens, "materials.rs");
}

fn generate_init(ident: Ident, value: Value) -> TokenStream {
    let corresponding_block = value
        .get("correspondingBlock")
        .and_then(Value::as_str)
        .map_or_else(
            || quote! { None },
            |v| {
                let block = format_ident!(
                    "{}",
                    v.split_once(":")
                        .map_or(v, |v| v.1)
                        .to_case(Case::UpperCamel)
                );
                quote! { Some(Block::#block) }
            },
        );

    quote! {
        #ident {
            block: #corresponding_block
        }
    }
}
