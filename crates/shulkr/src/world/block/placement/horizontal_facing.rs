use crate::world::block::{
    Block, BlockState,
    placement::{BlockPlaceContext, BlockPlacement},
    property::Properties,
};

pub struct HorizontalFacing;

impl BlockPlacement for HorizontalFacing {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        Some(block.default_state().with(
            Properties::HORIZONTAL_FACING,
            cx.horizontal_direction().opposite(),
        ))
    }
}
