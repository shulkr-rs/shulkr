use cerium::Server;
use cerium::auth::AuthMode;
use cerium::entity::GameMode;
use cerium::event::player::{PlayerConfigEvent, PlayerEvent, PlayerSpawnEvent};
use cerium::world::{DimensionType, World, block::BlockState};

fn main() {
    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    let mut states = BlockState::values();
    states.sort_by_key(|s| s.id());

    for (pos, block) in states.iter().enumerate() {
        let bz = ((pos / 168) + 1) as i32;
        let bx = ((pos % 168) + 1) as i32;
        world.set_block((bz * 2) - 1, 70, (bx * 2) - 1, *block);
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position((0.5, 71., 0.5));
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            event.get_player().set_game_mode(GameMode::Creative);
            event.get_player().set_flying(true);
        });

    server.bind("127.0.0.1:25565").unwrap();
}
