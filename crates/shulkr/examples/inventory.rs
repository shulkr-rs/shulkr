use shulkr::Server;
use shulkr::auth::AuthMode;
use shulkr::event::inventory::InventoryClickEvent;
use shulkr::event::player::{PlayerConfigEvent, PlayerEvent, PlayerSpawnEvent};
use shulkr::inventory::{Inventory, InventoryType};
use shulkr::item::{ItemStack, Material};
use shulkr::world::{DimensionType, World, block::Block};

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
            event.set_position([0.5, 75.0, 0.5]);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let inventory = Inventory::new(InventoryType::CartographyTable, "Custom Inventory");

            let stack = ItemStack::new(Material::ENDER_PEARL, 16);
            inventory.add_item_stack(stack);

            event.get_player().open_inventory(inventory);
        })
        .subscribe(|event: &mut InventoryClickEvent| {
            event
                .get_player()
                .send_message(format!("slot: {}", event.slot()));
            event
                .get_player()
                .send_message(format!("stack: {:?}", event.clicked_item().material()));
            event
                .get_player()
                .send_message(format!("slot: {:?}", event.click_action()));
        });

    server.bind("127.0.0.1:25565").unwrap();
}
