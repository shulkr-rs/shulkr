use shulkr::{
    Server,
    auth::AuthMode,
    entity::GameMode,
    event::player::{PlayerConfigEvent, PlayerEvent as _, PlayerSpawnEvent},
    world::{DimensionType, World, loader::anvil::AnvilLoader},
};
use std::path::PathBuf;

const RADIUS: i32 = 12;

fn main() {
    tracing_subscriber::fmt::init();

    let arg = std::env::args().nth(1).expect("usage: anvil <region-dir>");

    let server = Server::new(AuthMode::Online);
    let mut world = World::new(DimensionType::OVERWORLD);

    let mut loader = AnvilLoader::new(PathBuf::from(&arg));

    let mut loaded = 0;
    for cz in -RADIUS..=RADIUS {
        for cx in -RADIUS..=RADIUS {
            if let Some(chunk) = loader.load_chunk(cx, cz) {
                world.set_chunk(chunk);
                loaded += 1;
            }
        }
    }
    println!("loaded {loaded} chunks from anvil");

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 71., 0.5]);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            event.get_player().set_game_mode(GameMode::Creative);
            event.get_player().set_flying(true);
        });

    server.bind("127.0.0.1:25565").unwrap();
}
