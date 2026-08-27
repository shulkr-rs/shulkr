use convert_case::{Case, Casing as _};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{Ident, LitInt};

use crate::{object::StaticObjectBuilder, write_wide_file};

pub fn generate() {
    let object = StaticObjectBuilder::new("Block")
        .with_json(crate::read_asset("block.json"))
        .with_init(generate_init)
        .build();

    let tokens = object.generate();
    write_wide_file(&tokens, "blocks.rs");
}

fn generate_init(_ident: Ident, value: Value) -> TokenStream {
    let ident = format_ident!("Block");
    let props = value
        .get("properties")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|v| format_ident!("{}", v.as_str().unwrap()));

    let default_state_id = value
        .get("defaultStateId")
        .and_then(Value::as_u64)
        .expect("defaultStateId must be a number");
    let default_state_id = LitInt::new(&default_state_id.to_string(), Span::call_site());

    let min_state_id = value
        .get("minStateId")
        .and_then(Value::as_u64)
        .expect("minStateId must be a number");
    let min_state_id = LitInt::new(&min_state_id.to_string(), Span::call_site());

    let block_entity = value
        .get("blockEntity")
        .and_then(Value::as_str)
        .map_or_else(
            || quote! { None },
            |v| {
                let block = format_ident!(
                    "{}",
                    v.split_once(":").map_or(v, |v| v.1).to_case(Case::Constant)
                );
                quote! { Some(BlockEntityType::#block) }
            },
        );

    quote! {
        #ident::new(
            #default_state_id,
            #min_state_id,
            &[ #( &Properties::#props ),* ],
            #block_entity
        )
    }
}
