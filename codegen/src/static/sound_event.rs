use quote::format_ident;
use syn::Ident;

use crate::{object::StaticObjectBuilder, write_wide_file};

pub fn generate() {
    let object = StaticObjectBuilder::new("SoundEvent")
        .with_json(crate::read_asset("sound_event.json"))
        .with_ident(ident_for)
        .build();

    let tokens = object.generate();

    write_wide_file(&tokens, "sound_events.rs");
}

fn ident_for(key: &str) -> Ident {
    let path = key.split_once(':').map_or(key, |(_, path)| path);
    format_ident!("{}", path.replace(['.', '/'], "_").to_uppercase())
}
