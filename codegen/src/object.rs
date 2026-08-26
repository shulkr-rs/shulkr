use convert_case::{Case, Casing as _};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use syn::Ident;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct StaticObjectJson {
    entries: IndexMap<String, Value>,
}

pub struct StaticObjectBuilder {
    name: String,
    init: Option<Box<dyn Fn(Ident, Value) -> TokenStream>>,
    json: Option<String>,
}

impl StaticObjectBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            init: None,
            json: None,
        }
    }

    pub fn with_json(mut self, input: String) -> Self {
        self.json = Some(input);
        self
    }

    pub fn with_init<F, T>(mut self, f: F) -> Self
    where
        F: Fn(Ident, T) -> TokenStream + 'static,
        T: DeserializeOwned + 'static,
    {
        self.init = Some(Box::new(move |ident, value: Value| {
            f(ident, serde_json::from_value(value).unwrap())
        }));
        self
    }

    pub fn build(self) -> StaticObject {
        StaticObject {
            name: self.name.clone(),
            json: serde_json::from_str(&self.json.unwrap()).unwrap(),
            init: self.init.unwrap_or(Box::new(move |ident, _| {
                quote! { #ident }
            })),
        }
    }
}

pub struct StaticObject {
    name: String,
    json: StaticObjectJson,
    init: Box<dyn Fn(Ident, Value) -> TokenStream>,
}

impl StaticObject {
    fn parse_variants(&self) -> TokenStream {
        let variants: TokenStream = self
            .json
            .entries
            .keys()
            .map(|key| {
                let ident = format_ident!(
                    "{}",
                    key.split_once(":")
                        .map_or(key.clone(), |v| v.1.to_owned())
                        .to_case(Case::UpperCamel)
                );

                quote! {
                    #ident,
                }
            })
            .collect();

        variants
    }

    pub fn static_variants(&self) -> TokenStream {
        let name = format_ident!("{}Data", &self.name);
        let constructor = &self.init;

        self.json
            .entries
            .iter()
            .map(move |(key, value)| {
                let ident = format_ident!(
                    "{}",
                    key.split_once(":")
                        .map_or(key.clone(), |v| v.1.to_owned())
                        .to_case(Case::Constant)
                );
                let constructor = constructor(name.clone(), value.clone());

                quote! {
                    pub static #ident: #name = #constructor;
                }
            })
            .collect()
    }

    fn register_variants(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let variants: TokenStream = self
            .json
            .entries
            .keys()
            .map(|key| {
                let ident = format_ident!(
                    "{}",
                    key.split_once(":")
                        .map_or(key.clone(), |v| v.1.to_owned())
                        .to_case(Case::UpperCamel)
                );

                quote! {
                    register(#key, #name::#ident);
                }
            })
            .collect();

        variants
    }

    pub fn generate(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let variants = self.parse_variants();
        let static_variants = self.static_variants();
        let register_variants = self.register_variants();

        quote! {
            use cerium_macros::StaticObject;
            use cerium_macros::UnitEnum;
            use crate::registry::Registry;

            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StaticObject, UnitEnum)]
            #[repr(u16)]
            pub enum #name {
                #variants
            }

            #static_variants

            pub(crate) fn register_all(registry: &mut Registry<#name>) {
                let mut register = |key: &'static str, value: #name| {
                    Registry::register(registry, key.into(), value);
                };

                #register_variants
            }
        }
    }
}
