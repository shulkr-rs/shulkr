use shulkr::{
    Server,
    auth::AuthMode,
    entity::EntityLike,
    event::player::{PlayerConfigEvent, PlayerEvent as _, PlayerMoveEvent, PlayerSpawnEvent},
    scoreboard::{
        NumberFormat,
        below_name::BelowName,
        sidebar::{Sidebar, SidebarLine},
    },
    util::{Position, Viewable as _},
    world::{DimensionType, World, block::Block},
};
use std::sync::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Offline);
    let world = World::new(DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block([bx, 70, bz], Block::GRASS_BLOCK);
        }
    }

    let sidebar = Arc::new(Sidebar::new("sb-test", "Scoreboard Title"));
    sidebar.insert_line("line-0", SidebarLine::new("Line 1", 9, None));
    sidebar.insert_line("line-1", SidebarLine::new("Line 2", 8, None));
    sidebar.insert_line("line-3", SidebarLine::new("Line 3", 7, None));
    sidebar.insert_line("line-4", SidebarLine::new("", 6, Some(NumberFormat::Blank)));
    sidebar.insert_line(
        "line-5",
        SidebarLine::new("Your Position:", 5, Some(NumberFormat::Blank)),
    );
    sidebar.insert_line(
        "line-6",
        SidebarLine::new("X N/A", 4, Some(NumberFormat::Blank)),
    );
    sidebar.insert_line(
        "line-7",
        SidebarLine::new("Y N/A", 3, Some(NumberFormat::Blank)),
    );
    sidebar.insert_line(
        "line-8",
        SidebarLine::new("Z N/A", 2, Some(NumberFormat::Blank)),
    );
    sidebar.insert_line("line-9", SidebarLine::new("", 1, None));
    sidebar.insert_line("line-10", SidebarLine::new("Line 10", 0, None));

    let sidebar1 = sidebar.clone();
    let sidebar2 = sidebar.clone();

    let below_name = BelowName::new("below-name", "Hello");

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position([0.5, 75.0, 0.5]);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();
            let (x, y, z) = format_xyz(player.position());

            sidebar1.update_line("line-6", |l| l.set_text(x.clone()));
            sidebar1.update_line("line-7", |l| l.set_text(y.clone()));
            sidebar1.update_line("line-8", |l| l.set_text(z.clone()));
            sidebar1.add_viewer(player.clone());

            below_name.set_score(player.name(), 100);
            below_name.add_viewer(player.clone());
        })
        .subscribe(move |event: &mut PlayerMoveEvent| {
            let (x, y, z) = format_xyz(event.new_position());

            sidebar2.update_line("line-6", |l| l.set_text(x.clone()));
            sidebar2.update_line("line-7", |l| l.set_text(y.clone()));
            sidebar2.update_line("line-8", |l| l.set_text(z.clone()));
        });

    server.bind("127.0.0.1:25565").unwrap();
}

fn format_xyz(position: Position) -> (String, String, String) {
    (
        format!("X {:.3}", position.x()),
        format!("Y {:.3}", position.y()),
        format!("Z {:.3}", position.z()),
    )
}
