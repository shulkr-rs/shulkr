use shulkr::{
    Server,
    auth::AuthMode,
    entity::GameMode,
    event::player::{PlayerConfigEvent, PlayerEvent, PlayerRequestGameModeEvent, PlayerSpawnEvent},
    world::{DimensionType, World, block::BlockState},
};

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    let mut states = BlockState::all();
    states.sort_by_key(|s| s.state_id());

    let width = (states.len() as f64).sqrt().ceil() as usize;
    for (pos, block) in states.iter().enumerate() {
        let x = (pos / width * 2 + 1) as i32;
        let z = (pos % width * 2 + 1) as i32;
        world.load_chunk(x.div_euclid(16), z.div_euclid(16));
        world.set_block([x, 70, z], *block);
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 71., 0.5]);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();
            player.set_game_mode(GameMode::Creative);
            player.set_flying(true);
            player.set_permission_level(4);
        })
        .subscribe(|event: &mut PlayerRequestGameModeEvent| {
            event
                .get_player()
                .set_game_mode(event.requested_game_mode());
        });

    server.bind("127.0.0.1:25565").unwrap();
}
