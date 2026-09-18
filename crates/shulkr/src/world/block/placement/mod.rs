mod bed;
mod rotated_pillar;
mod sign;
mod trapdoor;

use std::sync::LazyLock;

use parking_lot::RwLock;

use crate::{
    entity::{Hand, Player},
    item::ItemStack,
    registry::Registries,
    util::{BlockPosition, Direction, HashMap, Point},
    world::{
        World,
        block::{Block, BlockState, hit_result::BlockHitResult, property::Properties},
    },
};

pub struct BlockPlaceContext {
    player: Option<Player>,
    world: World,
    hand: Hand,
    item_stack: ItemStack,
    hit_result: BlockHitResult,
}

impl BlockPlaceContext {
    pub fn new(
        player: Option<Player>,
        world: World,
        hand: Hand,
        item_stack: ItemStack,
        hit_result: BlockHitResult,
    ) -> Self {
        Self {
            player,
            world,
            hand,
            item_stack,
            hit_result,
        }
    }

    pub fn player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn hand(&self) -> Hand {
        self.hand
    }

    pub fn item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn hit_result(&self) -> &BlockHitResult {
        &self.hit_result
    }

    pub fn clicked_position(&self) -> BlockPosition {
        self.hit_result.position()
    }

    pub fn clicked_face(&self) -> Direction {
        self.hit_result.direction()
    }

    pub fn clicked_location(&self) -> Point {
        self.hit_result.location()
    }

    pub fn position(&self) -> BlockPosition {
        self.clicked_position()
            .relative(self.hit_result.direction())
    }

    pub fn can_replace(&self, position: BlockPosition) -> bool {
        self.world.get_block(position).as_block() == Block::AIR
    }

    pub fn horizontal_direction(&self) -> Direction {
        if let Some(player) = &self.player {
            Direction::from_yrot(player.head_roation() as f64)
        } else {
            Direction::North
        }
    }

    pub fn direction(&self) -> Direction {
        self.hit_result.direction()
    }

    pub fn rotation(&self) -> f32 {
        self.player
            .as_ref()
            .map(|p| p.head_roation())
            .unwrap_or_default()
    }
}

pub fn state_for_placement(block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
    if REG.is_registered(block) {
        REG.place(block, cx)
    } else {
        Some(block.default_state())
    }
}

pub trait BlockPlacement: Send + Sync {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState>;

    #[allow(unused_variables, reason = "function does nothing by default")]
    fn after_place(&self, state: BlockState, cx: &BlockPlaceContext) {}
}

pub static REG: LazyLock<BlockPlaceRegistry> = LazyLock::new(BlockPlaceRegistry::new);

pub struct BlockPlaceRegistry {
    placements: RwLock<HashMap<Block, Box<dyn BlockPlacement>>>,
}

impl Default for BlockPlaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockPlaceRegistry {
    pub fn new() -> Self {
        let this = Self {
            placements: RwLock::new(HashMap::default()),
        };

        // TODO: proper registration
        for block in Registries::BLOCK.values() {
            if block.default_state().get(Properties::AXIS).is_some() {
                this.register(*block, rotated_pillar::RotatedPillar);
            }

            if block.default_state().get(Properties::ROTATION_16).is_some() {
                this.register(*block, sign::SignBlock);
            }

            if Registries::BLOCK
                .key_of(block.0)
                .is_some_and(|k| k.to_string().contains("trapdoor"))
            {
                this.register(*block, trapdoor::TrapDoorBlock);
            }

            if Registries::BLOCK
                .key_of(block.0)
                .is_some_and(|k| k.to_string().contains("bed"))
            {
                this.register(*block, bed::BedBlock);
            }
        }

        this
    }

    pub fn register(&self, block: Block, placement: impl BlockPlacement + 'static) {
        self.placements.write().insert(block, Box::new(placement));
    }

    pub fn is_registered(&self, block: Block) -> bool {
        self.placements.read().contains_key(&block)
    }

    pub fn place(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        let guard = self.placements.read();
        let placement = guard.get(&block)?;
        placement.place_block(block, cx)
    }

    pub fn after_place(&self, block: Block, state: BlockState, cx: &BlockPlaceContext) {
        let guard = self.placements.read();
        if let Some(placement) = guard.get(&block) {
            placement.after_place(state, cx);
        }
    }
}
