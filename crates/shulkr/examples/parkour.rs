use parking_lot::Mutex;
use rand::{RngExt, seq::IndexedRandom as _};
use rustc_hash::FxBuildHasher;
use shulkr::{
    Server,
    auth::AuthMode,
    entity::{EntityLike, Player},
    event::player::{PlayerConfigEvent, PlayerEvent, PlayerMoveEvent},
    sound::SoundEvent,
    util::{BlockPosition, HashMap, Position},
    world::{DimensionType, World, block::Block},
};

const SPAWN: BlockPosition = BlockPosition::new(0, 100, 0);

const POSSIBLE_BLOCKS: [Block; 7] = [
    Block::ORANGE_WOOL,
    Block::YELLOW_WOOL,
    Block::LIME_WOOL,
    Block::CYAN_WOOL,
    Block::LIGHT_BLUE_WOOL,
    Block::MAGENTA_WOOL,
    Block::PINK_WOOL,
];

static GAME_STATES: Mutex<HashMap<Player, GameState>> =
    Mutex::new(HashMap::with_hasher(FxBuildHasher));

struct GameState {
    first: BlockPosition,
    second: BlockPosition,
    to_remove: Vec<BlockPosition>,
}

fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new(AuthMode::Online);

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            let world = World::new(DimensionType::OVERWORLD);
            world.set_block(SPAWN, Block::DIAMOND_BLOCK);

            let mut state = GameState {
                first: SPAWN,
                second: SPAWN,
                to_remove: Vec::new(),
            };
            reset_world(&mut state, &world);

            event.set_world(world);
            event.set_position([0.5, 65.0, 0.5]);
            GAME_STATES.lock().insert(event.get_player().clone(), state);
        })
        .subscribe(|event: &mut PlayerMoveEvent| {
            let player = event.get_player();
            let position = BlockPosition::from(event.new_position() - [0, 1, 0]);

            let mut game_states = GAME_STATES.lock();
            if let Some(state) = game_states.get_mut(player)
                && position.z() >= state.first.z()
            {
                let new_block = generate_block(state.second, &player.world());
                state.to_remove.push(new_block);
                state.first = state.second;
                state.second = new_block;
                player.play_sound_at(new_block, SoundEvent::ENTITY_CHICKEN_EGG);
            }
        })
        .subscribe(|event: &mut PlayerMoveEvent| {
            let player = event.get_player();

            let mut game_states = GAME_STATES.lock();
            let Some(state) = game_states.get_mut(player) else {
                return;
            };

            if (event.new_position().y() as i32) < SPAWN.y() - 32 {
                reset_world(state, &player.world());
                player.teleport_to(Position::from(SPAWN) + [0.5, 1.0, 0.5]);
            }
        });

    server.bind("127.0.0.1:25565").unwrap();
}

fn generate_block(start: BlockPosition, world: &World) -> BlockPosition {
    let mut rng = rand::rng();
    let y = rng.random_range(-1..2);
    let x = rng.random_range(-1..2);
    let z = if y > 0 { 3 } else { 4 };

    let position = BlockPosition::new(start.x() + x, start.y() + y, start.z() + z);
    world.set_block(position, *POSSIBLE_BLOCKS.choose(&mut rand::rng()).unwrap());
    position
}

fn reset_world(state: &mut GameState, world: &World) {
    for pos in state.to_remove.drain(..) {
        world.set_block(pos, Block::AIR);
    }

    let first = generate_block(SPAWN, world);
    let second = generate_block(first, world);

    state.first = first;
    state.second = second;
    state.to_remove.push(first);
    state.to_remove.push(second);
}
