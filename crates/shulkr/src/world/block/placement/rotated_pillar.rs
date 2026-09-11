use crate::world::block::{
    Block, BlockState,
    placement::{BlockPlaceContext, BlockPlacement},
    property::Properties,
};

pub struct RotatedPillar;

impl BlockPlacement for RotatedPillar {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        let face = cx.clicked_face();
        Some(block.default_state().with(Properties::AXIS, face.axis()))
    }
}
