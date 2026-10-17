use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::{Ident, LitInt};

use crate::{object::StaticObjectBuilder, write_wide_file};

#[derive(Deserialize)]
pub struct FluidsJson {
    fluids: IndexMap<String, FluidJson>,
    states: Vec<FluidStateJson>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluidJson {
    default_state_id: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluidStateJson {
    fluid: String,
    is_empty: bool,
    is_source: bool,
    is_full: bool,
    is_randomly_ticking: bool,
    amount: u8,
    falling: bool,
}

pub fn generate() {
    let json: FluidsJson = serde_json::from_str(&crate::read_asset("fluid.json")).unwrap();

    let states = generate_states(&json);

    let object = StaticObjectBuilder::new("Fluid")
        .with_json(json.fluids_as_json())
        .with_init(generate_init)
        .build();

    let tokens = object.generate();

    write_wide_file(&quote! { #states #tokens }, "fluids.rs");
}

impl FluidsJson {
    fn fluids_as_json(&self) -> String {
        let entries: Vec<String> = self
            .fluids
            .iter()
            .map(|(key, fluid)| {
                format!(
                    "{}:{{\"defaultStateId\":{}}}",
                    serde_json::to_string(key).unwrap(),
                    fluid.default_state_id
                )
            })
            .collect();

        format!("{{{}}}", entries.join(","))
    }
}

fn generate_init(ident: Ident, value: serde_json::Value) -> TokenStream {
    let default_state_id = value
        .get("defaultStateId")
        .and_then(serde_json::Value::as_u64)
        .expect("defaultStateId must be a number");
    let default_state_id = LitInt::new(&default_state_id.to_string(), Span::call_site());

    quote! {
        #ident::new(#default_state_id)
    }
}

fn generate_states(json: &FluidsJson) -> TokenStream {
    let count = json.states.len();

    let states: TokenStream = json
        .states
        .iter()
        .map(|state| {
            let fluid = crate::fluid_ident(&state.fluid);
            let amount = LitInt::new(&state.amount.to_string(), Span::call_site());

            let mut flags = Vec::<TokenStream>::new();
            if state.is_empty {
                flags.push(quote! { FluidState::IS_EMPTY });
            }
            if state.is_source {
                flags.push(quote! { FluidState::IS_SOURCE });
            }
            if state.is_full {
                flags.push(quote! { FluidState::IS_FULL });
            }
            if state.is_randomly_ticking {
                flags.push(quote! { FluidState::IS_RANDOMLY_TICKING });
            }
            if state.falling {
                flags.push(quote! { FluidState::FALLING });
            }

            let flags = if flags.is_empty() {
                quote! { 0 }
            } else {
                quote! { #( #flags )|* }
            };

            quote! {
                FluidState::new(Fluid::#fluid, #amount, #flags),
            }
        })
        .collect();

    quote! {
        pub static FLUID_STATES: [FluidState; #count] = [
            #states
        ];
    }
}
