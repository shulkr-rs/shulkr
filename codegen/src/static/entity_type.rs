use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_json::Value;
use syn::{Ident, LitFloat};

use crate::{object::StaticObjectBuilder, write_wide_file};

pub fn generate() {
    let object = StaticObjectBuilder::new("EntityType")
        .with_json(crate::read_asset("entity_type.json"))
        .with_init(generate_init)
        .build();

    let tokens = object.generate();

    write_wide_file(&tokens, "entity_types.rs");
}

fn generate_init(ident: Ident, value: Value) -> TokenStream {
    let width = lit_float(value.get("width").unwrap());
    let height = lit_float(value.get("height").unwrap());
    let eye_height = lit_float(value.get("eyeHeight").unwrap());

    let dimensions = quote! {
        EntityDimensions::new(#width, #height, #eye_height)
    };

    quote! {
        #ident::new(
           #dimensions
        )
    }
}

fn lit_float(value: &Value) -> LitFloat {
    let value = value.as_f64().unwrap();
    LitFloat::new(&format!("{value:.1}"), Span::call_site())
}
