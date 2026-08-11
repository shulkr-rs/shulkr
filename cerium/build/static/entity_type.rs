use crate::{object::StaticObjectBuilder, write_file};

pub fn generate() {
    println!("cargo:rerun-if-changed=build_assets/entity_types.json");

    let object = StaticObjectBuilder::new("EntityType")
        .with_json(include_str!("../../build_assets/entity_type.json").to_owned())
        .build();

    let tokens = object.generate();
    write_file(&tokens, "entity_types.rs");
}
