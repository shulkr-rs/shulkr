use crate::world::block::{
    Block, BlockState,
    placement::{BlockPlaceContext, BlockPlacement},
    property::{BedPart, Properties},
};

pub struct BedBlock;

impl BlockPlacement for BedBlock {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        let facing = cx.horizontal_direction();
        let head_pos = cx.position().relative(facing);
        let can = cx.can_replace(head_pos);

        if !can {
            return None;
        }

        Some(
            block
                .default_state()
                .with(Properties::HORIZONTAL_FACING, facing)
                .with(Properties::BED_PART, BedPart::Foot),
        )
    }

    fn after_place(&self, state: BlockState, cx: &BlockPlaceContext) {
        let facing = state.get(Properties::HORIZONTAL_FACING).unwrap();
        let head = state.with(Properties::BED_PART, BedPart::Head);
        let position = cx.position().relative(facing);

        match cx.player() {
            Some(player) => cx.world().place_block(player.clone(), position, head),
            None => cx.world().set_block(position, head),
        }
    }
}
