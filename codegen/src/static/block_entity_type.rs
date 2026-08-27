use crate::object::StaticObjectBuilder;
use crate::write_wide_file;

pub fn generate() {
    let object = StaticObjectBuilder::new("BlockEntityType")
        .with_json(crate::read_asset("block_entity_type.json"))
        .build();

    let tokens = object.generate();

    write_wide_file(&tokens, "block_entity_types.rs");
}
