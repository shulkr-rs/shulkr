use cerium::Server;
use cerium::auth::AuthMode;
use cerium::command::Command;
use cerium::event::player::{CommandResultEvent, PlayerConfigEvent, PlayerEvent};
use cerium::world::{DimensionType, World, block::Block};

fn main() {
    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block(bx, 70, bz, Block::GrassBlock);
        }
    }

    server.command_dispatcher().register(Command::new("whoami"));

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position((0.5, 75.0, 0.5));
        })
        .subscribe(|event: &mut CommandResultEvent| {
            let Some(matches) = event.matches() else {
                return;
            };
            if matches.command_name() != "whoami" {
                return;
            }

            let player = event.get_player();
            player.send_message(player.name());
        });

    server.bind("127.0.0.1:25565").unwrap();
}
