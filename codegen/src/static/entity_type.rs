use crate::object::StaticObjectBuilder;
use crate::write_wide_file;

pub fn generate() {
    let object = StaticObjectBuilder::new("EntityType")
        .with_json(crate::read_asset("entity_type.json"))
        .build();

    let tokens = object.generate();

    write_wide_file(&tokens, "entity_types.rs");
}
