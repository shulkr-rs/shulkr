use shulkr::{
    Server,
    auth::AuthMode,
    command::{
        CommandContext, CommandSource, argument,
        arguments::{DoubleArg, FloatArg, IntArg, StringArg},
        literal,
    },
    event::player::{PlayerConfigEvent, PlayerEvent, PlayerRequestGameModeEvent, PlayerSpawnEvent},
    world::{DimensionType, World, block::Block},
};

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);
    world.load_chunk(0, 0);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block([bx, 70, bz], Block::GRASS_BLOCK);
        }
    }

    let dispatcher = server.command_dispatcher();

    dispatcher.register(
        literal("whoami").executes(|cx: &CommandContext<CommandSource>| {
            let player = cx.source().as_player()?;
            player.send_message(player.name());
            Ok(1)
        }),
    );

    let teleport = dispatcher.register(literal("teleport").then(
        argument("x", DoubleArg::new()).then(argument("y", DoubleArg::new()).then(
            argument("z", DoubleArg::new()).executes(|cx: &CommandContext<CommandSource>| {
                let x = cx.get::<DoubleArg>("x")?;
                let y = cx.get::<DoubleArg>("y")?;
                let z = cx.get::<DoubleArg>("z")?;

                let player = cx.source().as_player()?;
                player.send_message(format!("Teleporting to {x} {y} {z}"));
                player.teleport_to([x, y, z]);
                Ok(1)
            }),
        )),
    ));

    dispatcher.register(literal("tp").redirect(teleport));

    dispatcher.register(
        literal("give").then(argument("item", StringArg::single_word()).then(
            argument("count", IntArg::between(1, 64)).executes(
                |cx: &CommandContext<CommandSource>| {
                    let item = cx.get::<StringArg>("item")?;
                    let count = cx.get::<IntArg>("count")?;

                    cx.source()
                        .as_player()?
                        .send_message(format!("Giving {count}x {item}"));
                    Ok(count)
                },
            ),
        )),
    );

    dispatcher.register(
        literal("msg").then(
            argument("target", StringArg::single_word())
                .suggests(|mut builder, cx: &CommandContext<CommandSource>| {
                    let Ok(player) = cx.source().as_player() else {
                        return builder.build();
                    };
                    let names: Vec<String> = player
                        .server()
                        .players()
                        .lock()
                        .iter()
                        .map(|player| player.name().clone())
                        .collect();
                    builder.suggest_matching(names);
                    builder.build()
                })
                .then(argument("message", StringArg::greedy()).executes(
                    |cx: &CommandContext<CommandSource>| {
                        let target = cx.get::<StringArg>("target")?;
                        let message = cx.get::<StringArg>("message")?;

                        cx.source()
                            .as_player()?
                            .send_message(format!("[you -> {target}] {message}"));
                        Ok(1)
                    },
                )),
        ),
    );

    dispatcher.register(literal("flyspeed").then(
        argument("multiplier", FloatArg::between(0.1, 20.0)).executes(
            |cx: &CommandContext<CommandSource>| {
                let multiplier = cx.get::<FloatArg>("multiplier")?;
                if let Ok(player) = cx.source().as_player() {
                    player.set_flying_speed(0.05 * multiplier);
                    player.send_message(format!("fly speed set to {multiplier}x"));
                }
                Ok(1)
            },
        ),
    ));

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 75.0, 0.5]);
        })
        .subscribe(|event: &mut PlayerSpawnEvent| {
            event.get_player().set_permission_level(4);
        })
        .subscribe(|event: &mut PlayerRequestGameModeEvent| {
            event
                .get_player()
                .set_game_mode(event.requested_game_mode());
        });

    server.bind("127.0.0.1:25565").unwrap();
}
