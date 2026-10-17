use crate::{
    util::Direction,
    world::block::{
        Block, BlockState,
        placement::{BlockPlaceContext, BlockPlacement},
        property::{Half, Properties},
    },
};

pub struct TrapDoorBlock;

impl BlockPlacement for TrapDoorBlock {
    fn place_block(&self, block: Block, cx: &BlockPlaceContext) -> Option<BlockState> {
        let clicked_face = cx.clicked_face();
        let mut state = block.default_state();

        if clicked_face.axis().is_horizontal() {
            state.set(Properties::HORIZONTAL_FACING, clicked_face);
            state.set(
                Properties::HALF,
                if cx.clicked_location().y() - cx.clicked_position().y() as f64 > 0.5 {
                    Half::Top
                } else {
                    Half::Bottom
                },
            );
        } else {
            state.set(
                Properties::HORIZONTAL_FACING,
                cx.horizontal_direction().opposite(),
            );
            state.set(
                Properties::HALF,
                if cx.clicked_face() == Direction::Up {
                    Half::Bottom
                } else {
                    Half::Top
                },
            );
        }

        Some(state)
    }
}
