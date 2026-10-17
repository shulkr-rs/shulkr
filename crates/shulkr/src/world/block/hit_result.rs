use crate::{
    protocol::decode::{Decode, DecodeError, PacketRead},
    util::{BlockPosition, Direction, Point},
};

#[derive(Debug, Clone)]
pub struct BlockHitResult {
    location: Point,
    direction: Direction,
    position: BlockPosition,
    inside_block: bool,
    world_border_hit: bool,
}

impl BlockHitResult {
    pub fn location(&self) -> Point {
        self.location
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn position(&self) -> BlockPosition {
        self.position
    }

    pub fn is_inside_block(&self) -> bool {
        self.inside_block
    }

    pub fn is_world_border(&self) -> bool {
        self.world_border_hit
    }
}

impl Decode for BlockHitResult {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let position = r.read_position()?;
        let direction = Direction::try_from(r.read_varint()?)?;
        let cursor_x = r.read_f32()?;
        let cursor_y = r.read_f32()?;
        let cursor_z = r.read_f32()?;
        let inside_block = r.read_bool()?;
        let world_border_hit = r.read_bool()?;

        Ok(BlockHitResult {
            location: Point::new(
                position.x() as f64 + cursor_x as f64,
                position.y() as f64 + cursor_y as f64,
                position.z() as f64 + cursor_z as f64,
            ),
            direction,
            position,
            inside_block,
            world_border_hit,
        })
    }
}
