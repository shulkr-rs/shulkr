use proc_macro2::Literal;
use quote::quote;
use serde::Deserialize;

use crate::{read_asset, write_file};

#[derive(Deserialize)]
struct Version {
    name: String,
    world_version: i32,
    series_id: String,
    protocol_version: i32,
    pack_version: PackVersion,
}

#[derive(Deserialize)]
struct PackVersion {
    resource_major: i32,
    resource_minor: i32,
    data_major: i32,
    data_minor: i32,
}

pub fn generate() {
    let version: Version = serde_json::from_str(&read_asset("version.json"))
        .unwrap_or_else(|e| panic!("failed to parse `version.json`: {e}"));

    let name = version.name;
    let series = version.series_id;
    let world_version = Literal::i32_unsuffixed(version.world_version);
    let protocol_version = Literal::i32_unsuffixed(version.protocol_version);

    let pack = version.pack_version;
    let resource_major = Literal::i32_unsuffixed(pack.resource_major);
    let resource_minor = Literal::i32_unsuffixed(pack.resource_minor);
    let data_major = Literal::i32_unsuffixed(pack.data_major);
    let data_minor = Literal::i32_unsuffixed(pack.data_minor);

    let out = quote! {
        pub const MINECRAFT_VERSION: &str = #name;
        pub const SERIES: &str = #series;
        pub const PROTOCOL_VERSION: i32 = #protocol_version;
        pub const DATA_VERSION: i32 = #world_version;
        pub const RESOURCE_PACK_VERSION: (i32, i32) = (#resource_major, #resource_minor);
        pub const DATA_PACK_VERSION: (i32, i32) = (#data_major, #data_minor);
    };

    write_file(&out, "version.rs");
}
