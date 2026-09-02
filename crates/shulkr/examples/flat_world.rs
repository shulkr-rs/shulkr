use shulkr::Server;
use shulkr::auth::AuthMode;
use shulkr::event::player::PlayerConfigEvent;
use shulkr::world::{DimensionType, World, block::Block};

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block([bx, 70, bz], Block::STONE);
        }
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 75.0, 0.5]);
        });

    server.bind("127.0.0.1:25565").unwrap();
}
