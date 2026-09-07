use shulkr::{
    Server,
    auth::AuthMode,
    command::{
        Command,
        arg::{
            Arg,
            kind::{ArgKind, StringBehaviour},
        },
    },
    entity::EntityLike,
    event::player::{CommandResultEvent, PlayerConfigEvent, PlayerEvent},
    sound::{Sound, SoundCategory, SoundEvent},
    util::Key,
    world::{DimensionType, World, block::Block},
};

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);
    let world = World::new(DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block([bx, 70, bz], Block::GRASS_BLOCK);
        }
    }

    server.command_dispatcher().register_all([
        Command::new("playsound").args(vec![
            Arg::new("sound", ArgKind::String(StringBehaviour::SingleWord)),
            Arg::new(
                "volume",
                ArgKind::Float {
                    min: Some(0.0),
                    max: None,
                },
            )
            .optional(),
            Arg::new(
                "pitch",
                ArgKind::Float {
                    min: Some(0.0),
                    max: Some(2.0),
                },
            )
            .optional(),
        ]),
        Command::new("stopsound"),
    ]);

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 75.0, 0.5]);
        })
        .subscribe(|event: &mut CommandResultEvent| {
            let Some(matches) = event.matches() else {
                return;
            };
            let player = event.get_player();

            match matches.command_name() {
                "playsound" => {
                    let name = matches.get::<String>("sound").unwrap();
                    let Some(sound) = SoundEvent::from_key(Key::of(name.clone())) else {
                        player.send_message(format!("unknown sound `{name}`"));
                        return;
                    };

                    let mut sound = Sound::new(sound, SoundCategory::Master);
                    sound.set_volume(matches.get::<f32>("volume").unwrap_or(1.0));
                    sound.set_pitch(matches.get::<f32>("pitch").unwrap_or(1.0));

                    player.play_sound_at(player.position(), sound);
                }
                "stopsound" => {
                    player.stop_sounds();
                    player.send_message("stopped all sounds");
                }
                _ => {}
            }
        });

    server.bind("127.0.0.1:25565").unwrap();
}
