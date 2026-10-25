use shulkr::{
    Server,
    auth::AuthMode,
    command::{
        CommandContext, CommandSource, Error, argument,
        arguments::{FloatArg, StringArg},
        literal,
    },
    entity::EntityLike,
    event::player::PlayerConfigEvent,
    sound::{Sound, SoundCategory, SoundEvent},
    util::Key,
    world::{DimensionType, World, block::Block},
};

fn play_sound(cx: &CommandContext<CommandSource>, volume: f32, pitch: f32) -> Result<i32, Error> {
    let player = cx.source().as_player()?;
    let name = cx.get::<StringArg>("sound")?;
    let Some(sound) = SoundEvent::from_key(Key::of(name.clone())) else {
        player.send_message(format!("unknown sound `{name}`"));
        return Ok(0);
    };

    let mut sound = Sound::new(sound, SoundCategory::Master);
    sound.set_volume(volume);
    sound.set_pitch(pitch);

    player.play_sound_at(player.position(), sound);
    Ok(1)
}

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
        literal("playsound").then(
            argument("sound", StringArg::single_word())
                .executes(|cx: &CommandContext<CommandSource>| play_sound(cx, 1.0, 1.0))
                .then(
                    argument("volume", FloatArg::at_least(0.0))
                        .executes(|cx: &CommandContext<CommandSource>| {
                            play_sound(cx, cx.get::<FloatArg>("volume")?, 1.0)
                        })
                        .then(argument("pitch", FloatArg::between(0.0, 2.0)).executes(
                            |cx: &CommandContext<CommandSource>| {
                                play_sound(
                                    cx,
                                    cx.get::<FloatArg>("volume")?,
                                    cx.get::<FloatArg>("pitch")?,
                                )
                            },
                        )),
                ),
        ),
    );

    dispatcher.register(
        literal("stopsound").executes(|cx: &CommandContext<CommandSource>| {
            let player = cx.source().as_player()?;
            player.stop_sounds();
            player.send_message("stopped all sounds");
            Ok(1)
        }),
    );

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 75.0, 0.5]);
        });

    server.bind("127.0.0.1:25565").unwrap();
}
