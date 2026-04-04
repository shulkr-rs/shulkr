use convert_case::{Case, Casing as _};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::write_file;

pub fn generate() {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/blocks.json")).unwrap();

    let variants: Vec<_> = entries
        .keys()
        .enumerate()
        .map(|(index, key)| {
            let ident = format_ident!(
                "{}",
                key.split_once(":")
                    .map_or(key.clone(), |v| v.1.to_owned())
                    .to_case(Case::UpperCamel)
            );
            (index as u16, ident, key.clone())
        })
        .collect();

    let enum_variants: TokenStream = variants
        .iter()
        .map(|(_, ident, _)| {
            quote! {
                #ident,
            }
        })
        .collect();

    let all_variants: TokenStream = variants
        .iter()
        .map(|(_, ident, _)| quote! { Block::#ident, })
        .collect();

    let try_from_arms: TokenStream = variants
        .iter()
        .map(|(index, ident, _)| {
            quote! {
                #index => Ok(Block::#ident),
            }
        })
        .collect();

    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum Block {
            #enum_variants
        }

        impl Block {


            pub fn all() -> &'static [Block] {
                &[#all_variants]
            }
        }

        impl TryFrom<u16> for Block {
            type Error = ();
            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    #try_from_arms
                    _ => Err(()),
                }
            }
        }
    };

    write_file(&out, "blocks.rs");
}
