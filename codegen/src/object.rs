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
    init: Option<Box<dyn Fn(Ident, &str, Value) -> TokenStream>>,
    ident: Option<Box<dyn Fn(&str) -> Ident>>,
    json: Option<String>,
}

impl StaticObjectBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            init: None,
            ident: None,
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
        self.init = Some(Box::new(move |ident, _key, value: Value| {
            f(ident, serde_json::from_value(value).unwrap())
        }));
        self
    }

    pub fn with_keyed_init<F, T>(mut self, f: F) -> Self
    where
        F: Fn(Ident, &str, T) -> TokenStream + 'static,
        T: DeserializeOwned + 'static,
    {
        self.init = Some(Box::new(move |ident, key, value: Value| {
            f(ident, key, serde_json::from_value(value).unwrap())
        }));
        self
    }

    pub fn with_ident<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) -> Ident + 'static,
    {
        self.ident = Some(Box::new(f));
        self
    }

    pub fn build(self) -> StaticObject {
        StaticObject {
            name: self.name.clone(),
            json: serde_json::from_str(&self.json.unwrap()).unwrap(),
            init: self.init.unwrap_or(Box::new(move |ident, _, _| {
                quote! { #ident::new() }
            })),
            ident: self.ident.unwrap_or(Box::new(default_ident_for)),
        }
    }
}

pub struct StaticObject {
    name: String,
    json: StaticObjectJson,
    init: Box<dyn Fn(Ident, &str, Value) -> TokenStream>,
    ident: Box<dyn Fn(&str) -> Ident>,
}

fn default_ident_for(key: &str) -> Ident {
    format_ident!(
        "{}",
        key.split_once(":")
            .map_or(key, |(_, name)| name)
            .to_case(Case::Constant)
    )
}

impl StaticObject {
    fn const_variants(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let constructor = &self.init;

        self.json
            .entries
            .iter()
            .map(move |(key, value)| {
                let ident = (self.ident)(key);
                let constructor = constructor(name.clone(), key, value.clone());

                quote! {
                    pub const #ident: #name = #constructor;
                }
            })
            .collect()
    }

    fn register_variants(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        self.json
            .entries
            .keys()
            .map(|key| {
                let ident = (self.ident)(key);
                quote! {
                    register(#key, #name::#ident);
                }
            })
            .collect()
    }

    pub fn generate(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let const_variants = self.const_variants();
        let register_variants = self.register_variants();

        quote! {
            use cerium_macros::static_registry;
            use crate::registry::Registry;

            #[static_registry]
            impl #name {
                #const_variants
            }

            pub(crate) fn register_all(registry: &mut Registry<#name>) {
                let mut register = |key: &'static str, value: #name| {
                    Registry::register(registry, key.into(), value);
                };

                #register_variants
            }
        }
    }
}
