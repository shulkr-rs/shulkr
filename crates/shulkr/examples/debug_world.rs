use shulkr::Server;
use shulkr::auth::AuthMode;
use shulkr::entity::GameMode;
use shulkr::event::player::{
    PlayerConfigEvent, PlayerEvent, PlayerRequestGameModeEvent, PlayerSpawnEvent,
};
use shulkr::world::{DimensionType, World, block::BlockState};

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    let mut states = BlockState::all();
    states.sort_by_key(|s| s.state_id());

    for (pos, block) in states.iter().enumerate() {
        let bz = ((pos / 168) + 1) as i32;
        let bx = ((pos % 168) + 1) as i32;
        world.set_block([(bz * 2) - 1, 70, (bx * 2) - 1], *block);
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
