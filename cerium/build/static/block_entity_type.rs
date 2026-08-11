use crate::{object::StaticObjectBuilder, write_file};

pub fn generate() {
    println!("cargo:rerun-if-changed=build_assets/block_entity_type.json");

    let object = StaticObjectBuilder::new("BlockEntityType")
        .with_json(include_str!("../../build_assets/block_entity_type.json").to_owned())
        .build();

    let tokens = object.generate();
    write_file(&tokens, "block_entity_types.rs");
}
