use crate::world::block::{
    Block, BlockState,
    placement::{BlockPlaceContext, BlockPlacement},
    property::Properties,
};

pub fn convert_to_segment(rot_degrees: f32) -> i32 {
    (rot_degrees * (16.0 / 360.0) + 0.5).floor() as i32 & 15
}

pub struct SignBlock;

impl BlockPlacement for SignBlock {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        let facing = cx.horizontal_direction();
        if !cx.can_replace(cx.position().relative(facing)) {
            return None;
        }

        Some(block.default_state().with(
            Properties::ROTATION_16,
            convert_to_segment(cx.rotation() + 180.) as u8,
        ))
    }
}
