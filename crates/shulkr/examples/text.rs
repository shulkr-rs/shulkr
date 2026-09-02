use shulkr::{
    Server,
    auth::AuthMode,
    entity::Player,
    event::player::{PlayerConfigEvent, PlayerEvent, PlayerSpawnEvent},
    scoreboard::objective::Objective,
    text::{HoverEvent, NamedColor, TextComponent},
    util::Viewable,
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

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 71., 0.5]);

            let player = event.get_player();
            handle_player_config(player);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();

            let objective = Objective::new("test", "Hello?");
            objective.add_viewer(player.clone());
        });

    server.bind("127.0.0.1:25565").unwrap();
}

fn handle_player_config(player: &Player) {
    let component = TextComponent::new()
        .child(TextComponent::text("HOWDY!").bold().color(NamedColor::Red))
        .child(TextComponent::NEW_LINE)
        .child("Second Line!")
        .on_hover(HoverEvent::show_text("Hello! I'm a HoverEvent."));

    println!("{}", serde_json::to_string(&component).unwrap());

    player.send_message(component);

    let header = "This is a Header.";
    let footer = TextComponent::new()
        .child("This is a footer.")
        .child(TextComponent::NEW_LINE)
        .child("This is the second line of the footer. ")
        .child(TextComponent::text("YAY!").bold().color(NamedColor::Gold));

    player.set_header_and_footer(header, footer);
}
