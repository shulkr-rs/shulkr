use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{Ident, LitFloat};

use crate::{object::StaticObjectBuilder, write_wide_file};

pub fn generate() {
    let object = StaticObjectBuilder::new("EnvironmentAttribute")
        .with_json(crate::read_asset("environment_attribute.json"))
        .with_keyed_init(generate_init)
        .with_ident(|key| {
            let path = key.split_once(':').map_or(key, |(_, path)| path);
            let name = path.rsplit('/').next().unwrap_or(path);
            format_ident!("{}", name.to_case(Case::Constant))
        })
        .build();

    let tokens = object.generate();
    write_wide_file(&tokens, "environment_attributes.rs");
}

fn variant(value: &Value, case: Case) -> Ident {
    format_ident!("{}", value.as_str().unwrap().to_case(case))
}

fn float(value: &Value) -> LitFloat {
    LitFloat::new(
        &format!("{}f32", value.as_f64().unwrap()),
        proc_macro2::Span::call_site(),
    )
}

fn generate_init(ident: Ident, key: &str, value: Value) -> TokenStream {
    let path = key.split_once(':').map_or(key, |(_, path)| path);
    let ty = value["type"].as_str().unwrap();
    let ty_ident = format_ident!("{}", ty);
    let data = &value["value"];

    let attribute_value = match ty {
        "Boolean" => {
            let b = data.as_bool().unwrap();
            quote! { AttributeValue::Boolean(#b) }
        }
        "Float" | "AngleDegrees" => {
            let f = float(data);
            quote! { AttributeValue::Float(#f) }
        }
        "RgbColor" | "ArgbColor" => {
            let color = data.as_str().unwrap();
            quote! { color(#color) }
        }
        "TriState" => {
            let variant = variant(data, Case::UpperCamel);
            quote! { AttributeValue::TriState(TriState::#variant) }
        }
        "MoonPhase" => {
            let variant = variant(data, Case::UpperCamel);
            quote! { AttributeValue::MoonPhase(MoonPhase::#variant) }
        }
        "Activity" => {
            let full = data.as_str().unwrap();
            let activity = full.split_once(':').map_or(full, |(_, path)| path);
            quote! { AttributeValue::Activity(Key::const_vanilla(#activity)) }
        }
        "BedRule" => {
            let can_sleep = variant(&data["can_sleep"], Case::UpperCamel);
            let can_set_spawn = variant(&data["can_set_spawn"], Case::UpperCamel);

            let explodes = match data.get("explodes").and_then(Value::as_bool) {
                Some(b) => quote! { Some(#b) },
                None => quote! { None },
            };

            let error_message = match data.get("error_message") {
                Some(v) if v.as_object().is_some_and(|o| o.len() == 1) => {
                    let translate = v["translate"].as_str().unwrap();
                    quote! { Some(TextComponent::const_translatable(#translate)) }
                }
                Some(v) => panic!(
                    "BedRule.error_message only supports a bare {{\"translate\": ..}}, got {v}"
                ),
                None => quote! { None },
            };

            quote! {
                AttributeValue::BedRule(BedRule {
                    can_sleep: BedRuleKind::#can_sleep,
                    can_set_spawn: BedRuleKind::#can_set_spawn,
                    explodes: #explodes,
                    error_message: #error_message,
                })
            }
        }
        "Particle" => {
            let kind = data["type"].as_str().unwrap();
            quote! {
                AttributeValue::Particle(Particle {
                    kind: std::borrow::Cow::Borrowed(#kind),
                    options: std::collections::BTreeMap::new(),
                })
            }
        }
        "AmbientParticles" => {
            assert!(
                data.as_array().is_some_and(|a| a.is_empty()),
                "non-empty AmbientParticles isn't supported by codegen yet"
            );
            quote! { AttributeValue::AmbientParticles(Vec::new()) }
        }
        "BackgroundMusic" => {
            quote! {
                AttributeValue::BackgroundMusic(BackgroundMusic {
                    default_music: None,
                    creative_music: None,
                    underwater_music: None,
                })
            }
        }
        "AmbientSounds" => {
            quote! {
                AttributeValue::AmbientSounds(AmbientSounds {
                    sound_loop: None,
                    mood: None,
                    additions: None,
                })
            }
        }
        other => panic!("unknown environment attribute type `{other}`"),
    };

    quote! {
        #ident::new(#path, AttributeType::#ty_ident, #attribute_value)
    }
}
