use std::collections::HashMap;

use convert_case::{Case, Casing as _};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use syn::{Ident, LitFloat, LitInt};

use crate::{object::StaticObjectBuilder, write_wide_file};

#[derive(Serialize, Deserialize)]
pub struct BlocksJson {
    shapes: Vec<ShapeJson>,
    blocks: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShapeJson {
    aabbs: Vec<String>,
}

pub fn generate() {
    let json: BlocksJson = serde_json::from_str(&crate::read_asset("block.json")).unwrap();

    let object = StaticObjectBuilder::new("Block")
        .with_json(json.blocks.to_string())
        .with_init(generate_init)
        .build();

    let mut aabb_cache = Vec::<TokenStream>::new();
    let mut aabb_indices = HashMap::<String, usize>::new();

    let shapes = json.shapes.iter().map(|json| {
        let aabbs: TokenStream = json
            .aabbs
            .iter()
            .map(|aabb| {
                let index = match aabb_indices.get(aabb) {
                    Some(&index) => index,
                    None => {
                        let index = aabb_cache.len();

                        let values: Vec<LitFloat> = aabb
                            .split(',')
                            .map(|value| {
                                let value: f64 = value.parse().unwrap();
                                LitFloat::new(&format!("{value:?}"), Span::call_site())
                            })
                            .collect();

                        let [min_x, min_y, min_z, max_x, max_y, max_z] =
                            values.try_into().unwrap_or_else(|_| unreachable!());

                        aabb_cache.push(quote! {
                            Aabb::new(
                                #min_x,
                                #min_y,
                                #min_z,
                                #max_x,
                                #max_y,
                                #max_z,
                            ),
                        });

                        aabb_indices.insert(aabb.clone(), index);

                        index
                    }
                };

                let index = syn::Index::from(index);

                quote! {
                    &AABB_CACHE[#index],
                }
            })
            .collect();

        quote! {
            VoxelShape::new(&[#aabbs]),
        }
    });

    let shape_count = json.shapes.len();
    let shapes: TokenStream = shapes.collect();

    let aabb_count = aabb_cache.len();

    let aabb_cache: TokenStream = aabb_cache.into_iter().collect();

    let caches = quote! {
        pub static AABB_CACHE: [Aabb; #aabb_count] = [
            #aabb_cache
        ];

        pub static SHAPE_CACHE: [VoxelShape; #shape_count] = [
            #shapes
        ];
    };

    let tokens = object.generate();
    write_wide_file(&quote! { #caches #tokens }, "blocks.rs");
}

fn generate_init(ident: Ident, value: Value) -> TokenStream {
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

    let min_state_id_lit = LitInt::new(&min_state_id.to_string(), Span::call_site());

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

    let shapes = generate_shapes(&value, min_state_id);
    let fluid_states = generate_fluid_states(&value, min_state_id);
    let state_properties = generate_state_properties(&value, min_state_id);
    let offset = generate_offset(&value);
    let transforms = generate_transforms(&value, min_state_id);

    quote! {
        #ident::new(
            #default_state_id,
            #min_state_id_lit,
            &[ #( &Properties::#props ),* ],
            #block_entity,
            #shapes,
            #fluid_states,
            #state_properties,
            #offset,
            #transforms,
        )
    }
}

fn generate_transforms(value: &Value, min_state_id: u64) -> TokenStream {
    let Some(transforms) = value.get("transforms").and_then(Value::as_object) else {
        return quote! { &[] };
    };

    let max_state_id = value
        .get("maxStateId")
        .and_then(Value::as_u64)
        .expect("maxStateId must be a number");

    const KINDS: [&str; 5] = ["cw90", "cw180", "ccw90", "leftRight", "frontBack"];

    let mut values = Vec::new();
    for state_id in min_state_id..=max_state_id {
        let entry = transforms.get(&state_id.to_string());
        for kind in KINDS {
            let transformed = entry
                .and_then(|entry| entry.get(kind))
                .and_then(Value::as_u64)
                .unwrap_or(state_id);
            values.push(LitInt::new(&transformed.to_string(), Span::call_site()));
        }
    }

    quote! {
        &[ #( #values ),* ]
    }
}

fn generate_offset(value: &Value) -> TokenStream {
    let Some(offset) = value.get("offset").and_then(Value::as_object) else {
        return quote! { None };
    };

    let kind = offset
        .get("type")
        .and_then(Value::as_str)
        .expect("offset type must be a string");
    let kind = match kind {
        "XZ" => quote! { OffsetType::Xz },
        "XYZ" => quote! { OffsetType::Xyz },
        other => panic!("unknown offset type `{other}`"),
    };

    let max_horizontal = offset
        .get("maxHorizontal")
        .and_then(Value::as_f64)
        .expect("maxHorizontal must be a number");
    let max_horizontal = LitFloat::new(&format!("{max_horizontal:?}"), Span::call_site());

    let max_vertical = offset
        .get("maxVertical")
        .and_then(Value::as_f64)
        .expect("maxVertical must be a number");
    let max_vertical = LitFloat::new(&format!("{max_vertical:?}"), Span::call_site());

    quote! {
        Some(BlockOffset::new(#kind, #max_horizontal, #max_vertical))
    }
}

fn generate_state_properties(value: &Value, min_state_id: u64) -> TokenStream {
    let state_properties = value
        .get("stateProperties")
        .and_then(Value::as_object)
        .expect("stateProperties must be an object");

    let default = state_properties
        .get("*")
        .and_then(Value::as_u64)
        .expect("stateProperties must contain '*'");

    let max_state_id = value
        .get("maxStateId")
        .and_then(Value::as_u64)
        .expect("maxStateId must be a number");

    let mut properties = vec![default; (max_state_id - min_state_id + 1) as usize];

    for (state, value) in state_properties {
        if state == "*" {
            continue;
        }

        let state_id = state.parse::<u64>().expect("invalid state ID");
        let index = (state_id - min_state_id) as usize;

        properties[index] = value
            .as_u64()
            .expect("state property mapping must be a number");
    }

    let values = properties
        .into_iter()
        .map(|value| LitInt::new(&value.to_string(), Span::call_site()));

    quote! {
        &[ #( #values ),* ]
    }
}

fn generate_fluid_states(value: &Value, min_state_id: u64) -> TokenStream {
    let fluid_states = value
        .get("fluidStates")
        .and_then(Value::as_object)
        .expect("fluidStates must be an object");

    let default = fluid_states
        .get("*")
        .and_then(Value::as_u64)
        .expect("fluidStates must contain '*'");

    let max_state_id = value
        .get("maxStateId")
        .and_then(Value::as_u64)
        .expect("maxStateId must be a number");

    let mut states = vec![default; (max_state_id - min_state_id + 1) as usize];

    for (state, fluid_state) in fluid_states {
        if state == "*" {
            continue;
        }

        let state_id = state.parse::<u64>().expect("invalid state ID");
        let index = (state_id - min_state_id) as usize;

        states[index] = fluid_state
            .as_u64()
            .expect("fluid state mapping must be a number");
    }

    if states.iter().all(|&state| state == 0) {
        return quote! { &[] };
    }

    let values = states
        .into_iter()
        .map(|value| LitInt::new(&value.to_string(), Span::call_site()));

    quote! {
        &[ #( #values ),* ]
    }
}

fn generate_shapes(value: &Value, min_state_id: u64) -> TokenStream {
    let shapes = value
        .get("shapes")
        .and_then(Value::as_object)
        .expect("shapes must be an object");

    let default = shapes
        .get("*")
        .and_then(Value::as_str)
        .expect("shapes must contain '*'");

    let default = pack_shapes(default);

    let max_state_id = value
        .get("maxStateId")
        .and_then(Value::as_u64)
        .expect("maxStateId must be a number");

    let mut state_shapes = vec![default; (max_state_id - min_state_id + 1) as usize];

    for (state, shapes) in shapes {
        if state == "*" {
            continue;
        }

        let state_id = state.parse::<u64>().expect("invalid state ID");
        let index = (state_id - min_state_id) as usize;

        state_shapes[index] = pack_shapes(shapes.as_str().expect("shape mapping must be a string"));
    }

    let values = state_shapes
        .into_iter()
        .map(|value| LitInt::new(&value.to_string(), Span::call_site()));

    quote! {
        &[ #( #values ),* ]
    }
}

fn pack_shapes(value: &str) -> u64 {
    let values: [u64; 6] = value
        .split(',')
        .map(|v| v.parse().expect("shape ID must be an integer"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("expected 6 shape IDs");

    assert!(values.iter().all(|&v| v < 1024), "shape ID exceeds 10 bits");

    values
        .into_iter()
        .enumerate()
        .fold(0, |packed, (index, value)| packed | value << (index * 10))
}
