use super::*;
use crate::{
    registry::{Id, Registry, registries},
    util::{
        BlockPosition, Direction, Key, OffsetVoxelShape, Point, SupportType, is_offset_face_sturdy,
        is_side_solid,
    },
    world::{
        block::property::Property,
        fluid::{FLUID_STATES, FluidState},
    },
};
use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
};
use std::{collections::BTreeMap, fmt, sync::OnceLock};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockState {
    pub block: Block,
    pub state_id: u16,
}

impl BlockState {
    pub fn block_entity(&self) -> Option<&BlockEntityType> {
        self.block.data().block_entity.as_ref()
    }

    /// Returns the id of the underlying [Block].
    pub const fn id(&self) -> u16 {
        self.block.0
    }

    /// Returns the id of the [BlockState].
    pub const fn state_id(&self) -> u16 {
        self.state_id
    }

    /// Returns this [BlockState] back into a [`Block`].
    pub const fn as_block(&self) -> Block {
        self.block
    }

    /// Returns the value of a specific [Property].
    ///
    /// If the Block does not have this value [None] is returned.
    pub fn get<P>(&self, property: P) -> Option<P::Value>
    where
        P: Property,
    {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            if prop.name() == property.name() {
                let index = (relative / stride) % count;
                return Some(property.by_index(index as usize));
            }
            stride *= count;
        }
        None
    }

    pub fn has_property<P>(&self, property: P) -> bool
    where
        P: Property,
    {
        let names = property.get_possible_value_names();
        self.block.data().properties.iter().any(|prop| {
            prop.name() == property.name() && *prop.get_possible_value_names() == *names
        })
    }

    pub fn try_get<P>(&self, property: P) -> P::Value
    where
        P: Property,
    {
        self.get(property).expect("failed to get property")
    }

    /// Sets the value of a specific [Property].
    ///
    /// Panics if the block does not have this property.
    pub fn set<P>(&mut self, property: P, value: P::Value)
    where
        P: Property,
    {
        let Some(state_id) = self.set_index(property.name(), property.index_of(&value)) else {
            panic!(
                "Property {} not found on block {}",
                property.name(),
                Id::from(self.block)
            );
        };
        self.state_id = state_id;
    }

    pub fn with<P>(&self, property: P, value: P::Value) -> BlockState
    where
        P: Property,
    {
        let state_id = self.set_index(property.name(), property.index_of(&value));
        if let Some(state_id) = state_id {
            return BlockState {
                block: self.block,
                state_id,
            };
        }

        panic!(
            "Property {} not found on block {}",
            property.name(),
            Id::from(self.block)
        );
    }

    #[cfg(any(feature = "anvil", feature = "polar"))]
    pub(crate) fn with_index(&self, property: &dyn Property, index: usize) -> Option<BlockState> {
        self.set_index(property.name(), index)
            .map(|state_id| BlockState {
                block: self.block,
                state_id,
            })
    }

    pub fn get_by_name(&self, name: &str) -> Option<&'static str> {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            if prop.name() == name {
                let index = (relative / stride) % count;
                return Some(prop.value_name_from_index(index as usize));
            }
            stride *= count;
        }
        None
    }

    pub fn with_by_name(&self, name: &str, value: &str) -> Option<BlockState> {
        let prop = self
            .block
            .data()
            .properties
            .iter()
            .find(|prop| prop.name() == name)?;
        let index = (0..prop.len()).find(|&index| prop.value_name_from_index(index) == value)?;
        self.set_index(name, index).map(|state_id| BlockState {
            block: self.block,
            state_id,
        })
    }

    pub fn copy_properties_to(&self, block: Block) -> BlockState {
        let mut target = block.default_state();
        for (name, value) in self.properties() {
            if let Some(next) = target.with_by_name(name, value) {
                target = next;
            }
        }
        target
    }

    pub fn parse(input: &str) -> Option<BlockState> {
        let input = input.trim();
        let (name, properties) = match input.split_once('[') {
            None => (input, None),
            Some((name, rest)) => (name, Some(rest.strip_suffix(']')?)),
        };

        let mut state = Block::from_key(Key::of(name.trim().to_owned()))?.default_state();

        for entry in properties.into_iter().flat_map(|rest| rest.split(',')) {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }
            let Some((property, value)) = entry.split_once('=') else {
                continue;
            };
            if let Some(next) = state.with_by_name(property.trim(), value.trim()) {
                state = next;
            }
        }

        Some(state)
    }

    pub fn with_properties<'a>(
        &self,
        properties: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> BlockState {
        let mut state = *self;
        for (property, value) in properties {
            if let Some(next) = state.with_by_name(property, value) {
                state = next;
            }
        }
        state
    }

    pub fn properties(&self) -> Vec<(&'static str, &'static str)> {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        let mut properties = Vec::new();
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            let index = (relative / stride) % count;
            properties.push((prop.name(), prop.value_name_from_index(index as usize)));
            stride *= count;
        }
        properties.reverse();
        properties
    }

    fn set_index(&self, name: &str, index: usize) -> Option<u16> {
        let relative = self.state_id - self.block.data().min_state_id;
        let mut stride = 1u16;
        for prop in self.block.data().properties.iter().rev() {
            let count = prop.len() as u16;
            if prop.name() == name {
                let old_index = (relative / stride) % count;
                return Some(self.state_id - old_index * stride + index as u16 * stride);
            }
            stride *= count;
        }
        None
    }

    /// Returns a [BlockState] based on the given `state_id`. If no corrosponding BlockState is found [None] is returned.
    pub fn from_id(state_id: u16) -> Option<BlockState> {
        let block_index = block_state_table().block_by_state(state_id)?;
        let block = **registries::BLOCK.by_id(block_index as Id)?;
        Some(BlockState { block, state_id })
    }

    /// Returns a [BlockState] based on the given `key`. If no corrosponding BlockState is found [None] is returned.
    pub fn from_key(key: impl Into<Key>) -> Option<BlockState> {
        Block::from_key(key).map(|b| b.default_state())
    }

    pub fn all() -> Vec<BlockState> {
        registries::BLOCK
            .values()
            .flat_map(|block| {
                let min_state_id = block.data().min_state_id;
                (min_state_id..min_state_id + block.data().state_count()).map(move |state_id| {
                    BlockState {
                        block: *block,
                        state_id,
                    }
                })
            })
            .collect()
    }

    const fn packed_shapes(&self) -> u64 {
        self.block.data().shapes[(self.state_id() - self.block.data().min_state_id) as usize]
    }

    const fn shape(&self, shift: u32) -> &'static VoxelShape {
        let id = shape_id(self.packed_shapes(), shift);
        &SHAPE_CACHE[id]
    }

    pub const fn outline_shape(&self) -> &'static VoxelShape {
        self.shape(OUTLINE_SHIFT)
    }

    pub const fn collision_shape(&self) -> &'static VoxelShape {
        self.shape(COLLISION_SHIFT)
    }

    pub const fn interaction_shape(&self) -> &'static VoxelShape {
        self.shape(INTERACTION_SHIFT)
    }

    pub const fn occlusion_shape(&self) -> &'static VoxelShape {
        self.shape(OCCLUSION_SHIFT)
    }

    pub const fn visual_shape(&self) -> &'static VoxelShape {
        self.shape(VISUAL_SHIFT)
    }

    pub const fn support_shape(&self) -> &'static VoxelShape {
        self.shape(SUPPORT_SHIFT)
    }

    pub const fn get_static_collision_shape(&self) -> &'static VoxelShape {
        self.collision_shape()
    }

    pub const fn get_occlusion_shape(&self) -> &'static VoxelShape {
        self.occlusion_shape()
    }

    pub fn offset_at(&self, pos: impl Into<BlockPosition>) -> Point {
        match &self.block.data().offset {
            None => Point::ZERO,
            Some(offset) => {
                let pos = pos.into();
                offset.at(pos.x(), pos.z())
            }
        }
    }

    pub fn get_collision_shape_at(&self, pos: impl Into<BlockPosition>) -> OffsetVoxelShape {
        OffsetVoxelShape::new(*self.collision_shape(), self.offset_at(pos))
    }

    pub fn get_support_shape_at(&self, pos: impl Into<BlockPosition>) -> OffsetVoxelShape {
        OffsetVoxelShape::new(*self.support_shape(), self.offset_at(pos))
    }

    pub fn get_outline_shape_at(&self, pos: impl Into<BlockPosition>) -> OffsetVoxelShape {
        OffsetVoxelShape::new(*self.outline_shape(), self.offset_at(pos))
    }

    pub fn is_side_solid_for_at(
        &self,
        pos: impl Into<BlockPosition>,
        direction: Direction,
        support_type: SupportType,
    ) -> bool {
        is_offset_face_sturdy(self.get_support_shape_at(pos), direction, support_type)
    }

    pub fn is_side_solid_at(&self, pos: impl Into<BlockPosition>, direction: Direction) -> bool {
        self.is_side_solid_for_at(pos, direction, SupportType::Full)
    }

    #[inline]
    const fn state_properties(&self) -> u32 {
        let properties = self.block.data().state_properties;
        properties[(self.state_id - self.block.data().min_state_id) as usize]
    }

    #[inline]
    const fn has_flag(&self, flag: u32) -> bool {
        self.state_properties() >> StateFlags::SHIFT & flag != 0
    }

    pub const fn get_light_emission(&self) -> u8 {
        (self.state_properties() & 0xF) as u8
    }

    pub const fn get_light_dampening(&self) -> u8 {
        (self.state_properties() >> 4 & 0xF) as u8
    }

    pub const fn liquid(&self) -> bool {
        self.has_flag(StateFlags::IS_LIQUID)
    }

    pub const fn is_solid(&self) -> bool {
        self.has_flag(StateFlags::IS_SOLID)
    }

    pub const fn is_solid_render(&self) -> bool {
        self.has_flag(StateFlags::IS_SOLID_RENDER)
    }

    pub const fn can_occlude(&self) -> bool {
        self.has_flag(StateFlags::CAN_OCCLUDE)
    }

    pub const fn is_replaceable(&self) -> bool {
        self.has_flag(StateFlags::CAN_BE_REPLACED)
    }

    pub const fn use_shape_for_light_occlusion(&self) -> bool {
        self.has_flag(StateFlags::USE_SHAPE_FOR_LIGHT_OCCLUSION)
    }

    pub const fn propagates_skylight_down(&self) -> bool {
        self.has_flag(StateFlags::PROPAGATES_SKYLIGHT_DOWN)
    }

    pub const fn is_randomly_ticking(&self) -> bool {
        self.has_flag(StateFlags::IS_RANDOMLY_TICKING)
    }

    pub const fn ignited_by_lava(&self) -> bool {
        self.has_flag(StateFlags::IGNITED_BY_LAVA)
    }

    pub const fn has_dynamic_shape(&self) -> bool {
        self.has_flag(StateFlags::HAS_DYNAMIC_SHAPE)
    }

    pub const fn blocks_motion(&self) -> bool {
        !matches!(self.block, Block::COBWEB | Block::BAMBOO_SAPLING) && self.is_solid()
    }

    fn transformed(&self, transform: Transform) -> BlockState {
        let transforms = self.block.data().transforms;
        if transforms.is_empty() {
            return *self;
        }

        let relative = (self.state_id - self.block.data().min_state_id) as usize;
        let state_id = transforms[relative * Transform::COUNT + transform.index()];

        BlockState {
            block: self.block,
            state_id,
        }
    }

    pub fn rotate(&self, rotation: Rotation) -> BlockState {
        match Transform::of_rotation(rotation) {
            None => *self,
            Some(transform) => self.transformed(transform),
        }
    }

    pub fn mirror(&self, mirror: Mirror) -> BlockState {
        match Transform::of_mirror(mirror) {
            None => *self,
            Some(transform) => self.transformed(transform),
        }
    }

    pub fn is(&self, block: Block) -> bool {
        self.as_block() == block
    }

    pub const fn is_air(&self) -> bool {
        self.has_flag(StateFlags::IS_AIR)
    }

    pub fn is_side_solid(&self, side: Direction) -> bool {
        is_side_solid(*self.support_shape(), side, SupportType::Full)
    }
    pub fn get_fluid_state(&self) -> FluidState {
        let fluid_states = self.block.data().fluid_states;
        if fluid_states.is_empty() {
            return FluidState::EMPTY;
        }

        let relative = (self.state_id - self.block.data().min_state_id) as usize;
        FLUID_STATES[fluid_states[relative] as usize]
    }

    pub fn has_fluid(&self) -> bool {
        !self.get_fluid_state().is_empty()
    }
}

const fn shape_id(packed: u64, shift: u32) -> usize {
    ((packed >> shift) & SHAPE_MASK) as usize
}

const SHAPE_BITS: u32 = 10;
const SHAPE_MASK: u64 = (1 << SHAPE_BITS) - 1;

const OUTLINE_SHIFT: u32 = 0;
const COLLISION_SHIFT: u32 = 10;
const INTERACTION_SHIFT: u32 = 20;
const OCCLUSION_SHIFT: u32 = 30;
const VISUAL_SHIFT: u32 = 40;
const SUPPORT_SHIFT: u32 = 50;

impl From<Block> for BlockState {
    fn from(block: Block) -> Self {
        block.default_state()
    }
}

impl Default for BlockState {
    fn default() -> Self {
        Block::AIR.default_state()
    }
}

impl fmt::Debug for BlockState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.block.try_key() {
            Some(key) => write!(f, "{key}")?,
            None => return write!(f, "BlockState({})", self.state_id),
        }

        let properties = self.properties();
        if properties.is_empty() {
            return Ok(());
        }

        f.write_str("[")?;
        for (index, (name, value)) in properties.iter().enumerate() {
            if index > 0 {
                f.write_str(",")?;
            }
            write!(f, "{name}={value}")?;
        }
        f.write_str("]")
    }
}

impl<'de> Deserialize<'de> for BlockState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(alias = "id")]
            Name,
            #[serde(alias = "properties")]
            Properties,
            #[serde(other)]
            Other,
        }

        struct BlockStateVisitor;

        impl<'de> Visitor<'de> for BlockStateVisitor {
            type Value = BlockState;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a block id or a block state with a Name and optional Properties")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let name = Key::of(value.to_owned());
                Block::from_key(name.clone())
                    .map(|block| block.default_state())
                    .ok_or_else(|| de::Error::custom(format!("unknown block {name}")))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<Key> = None;
                let mut properties: Option<BTreeMap<String, String>> = None;

                while let Some(field) = map.next_key::<Field>()? {
                    match field {
                        Field::Name => name = Some(map.next_value()?),
                        Field::Properties => properties = Some(map.next_value()?),
                        Field::Other => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("Name"))?;
                let block = Block::from_key(name.clone())
                    .ok_or_else(|| de::Error::custom(format!("unknown block {name}")))?;

                let mut state = block.default_state();
                for (property, value) in properties.into_iter().flatten() {
                    if let Some(next) = state.with_by_name(&property, &value) {
                        state = next;
                    }
                }

                Ok(state)
            }
        }

        deserializer.deserialize_any(BlockStateVisitor)
    }
}

impl Serialize for BlockState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key = self
            .block
            .try_key()
            .ok_or_else(|| serde::ser::Error::custom("block is not registered"))?;
        let properties = self.properties();

        let mut state = serializer.serialize_struct("BlockState", 2)?;
        state.serialize_field("Name", key)?;
        if properties.is_empty() {
            state.skip_field("Properties")?;
        } else {
            let properties: BTreeMap<&str, &str> = properties.into_iter().collect();
            state.serialize_field("Properties", &properties)?;
        }
        state.end()
    }
}

pub struct BlockStateTable {
    state_to_block: Vec<u16>,
}

impl BlockStateTable {
    pub fn build(blocks: &Registry<Block>) -> Self {
        let total = blocks
            .values()
            .map(|block| {
                let data = block.data();
                (data.min_state_id + data.state_count()) as usize
            })
            .max()
            .unwrap_or(0);

        let mut state_to_block = vec![0u16; total];

        for (index, block) in blocks.values().enumerate() {
            let data = block.data();
            let start = data.min_state_id as usize;
            let end = start + data.state_count() as usize;
            state_to_block[start..end].fill(index as u16);
        }

        Self { state_to_block }
    }

    pub fn block_by_state(&self, state_id: u16) -> Option<u16> {
        self.state_to_block.get(state_id as usize).copied()
    }
}

static BLOCK_STATE_TABLE: OnceLock<BlockStateTable> = OnceLock::new();

pub fn block_state_table() -> &'static BlockStateTable {
    BLOCK_STATE_TABLE.get_or_init(|| BlockStateTable::build(&registries::BLOCK))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::block::{Properties, SlabType};

    fn parse(json: &str) -> BlockState {
        serde_json::from_str(json).expect("block state should parse")
    }

    #[test]
    fn a_name_only_state_is_the_block_default() {
        crate::registry::registries::register_all();
        let state = parse(r#"{"Name": "minecraft:oak_log"}"#);

        assert_eq!(state, Block::OAK_LOG.default_state());
    }

    #[test]
    fn properties_are_applied_by_name() {
        crate::registry::registries::register_all();
        let state = parse(r#"{"Name": "minecraft:oak_log", "Properties": {"axis": "x"}}"#);

        assert_eq!(state.as_block(), Block::OAK_LOG);
        assert_eq!(state.get_by_name("axis"), Some("x"));
        assert_ne!(state, Block::OAK_LOG.default_state());
    }

    #[test]
    fn an_unprefixed_name_is_read_as_vanilla() {
        crate::registry::registries::register_all();
        assert_eq!(
            parse(r#"{"Name": "oak_log"}"#),
            Block::OAK_LOG.default_state()
        );
    }

    #[test]
    fn unknown_properties_and_values_leave_the_default_in_place() {
        crate::registry::registries::register_all();
        let default_axis = Block::OAK_LOG.default_state().get_by_name("axis");

        let unknown_property =
            parse(r#"{"Name": "minecraft:oak_log", "Properties": {"nope": "x"}}"#);
        assert_eq!(unknown_property, Block::OAK_LOG.default_state());

        let unknown_value =
            parse(r#"{"Name": "minecraft:oak_log", "Properties": {"axis": "nope"}}"#);
        assert_eq!(unknown_value.get_by_name("axis"), default_axis);
    }

    #[test]
    fn unknown_fields_are_ignored() {
        crate::registry::registries::register_all();
        let state = parse(r#"{"Name": "minecraft:oak_log", "extra": 7}"#);

        assert_eq!(state, Block::OAK_LOG.default_state());
    }

    #[test]
    fn an_unknown_block_is_an_error() {
        let result: Result<BlockState, _> =
            serde_json::from_str(r#"{"Name": "minecraft:not_a_block"}"#);

        assert!(result.is_err());
    }

    #[test]
    fn a_missing_name_is_an_error() {
        let result: Result<BlockState, _> = serde_json::from_str(r#"{"Properties": {}}"#);

        assert!(result.is_err());
    }

    #[test]
    fn states_round_trip_through_serde() {
        crate::registry::registries::register_all();
        let state = parse(r#"{"Name": "minecraft:oak_log", "Properties": {"axis": "x"}}"#);

        let json = serde_json::to_string(&state).expect("block state should serialize");
        assert_eq!(parse(&json), state);
    }

    #[test]
    fn properties_lists_every_property_of_the_state() {
        let state = Block::OAK_LOG.default_state();

        let properties = state.properties();
        assert!(properties.iter().any(|(name, _)| *name == "axis"));
        for (name, value) in properties {
            assert_eq!(state.get_by_name(name), Some(value));
        }
    }

    #[test]
    fn debug_prints_the_key_and_properties() {
        crate::registry::registries::register_all();
        let state = parse(r#"{"Name": "minecraft:oak_log", "Properties": {"axis": "x"}}"#);

        assert_eq!(format!("{state:?}"), "minecraft:oak_log[axis=x]");
    }

    #[test]
    fn light_levels_come_from_the_state_table() {
        assert_eq!(Block::TORCH.default_state().get_light_emission(), 14);
        assert_eq!(Block::LAVA.default_state().get_light_emission(), 15);
        assert_eq!(Block::SEA_LANTERN.default_state().get_light_emission(), 15);
        assert_eq!(Block::STONE.default_state().get_light_emission(), 0);

        assert_eq!(Block::STONE.default_state().get_light_dampening(), 15);
        assert_eq!(Block::AIR.default_state().get_light_dampening(), 0);
        assert_eq!(Block::WATER.default_state().get_light_dampening(), 1);
    }

    #[test]
    fn solidity_flags_match_vanilla() {
        let stone = Block::STONE.default_state();
        assert!(stone.is_solid());
        assert!(stone.is_solid_render());
        assert!(stone.can_occlude());

        let glass = Block::GLASS.default_state();
        assert!(glass.is_solid());
        assert!(!glass.is_solid_render());
        assert!(!glass.can_occlude());

        let air = Block::AIR.default_state();
        assert!(!air.is_solid());
        assert!(air.is_air());
        assert!(air.is_replaceable());
    }

    #[test]
    fn blocks_motion_keeps_vanillas_exceptions() {
        assert!(Block::STONE.default_state().blocks_motion());
        assert!(!Block::AIR.default_state().blocks_motion());

        let cobweb = Block::COBWEB.default_state();
        assert!(cobweb.is_solid());
        assert!(!cobweb.blocks_motion());
    }

    #[test]
    fn liquid_is_the_fluid_block_itself_not_waterlogging() {
        assert!(Block::WATER.default_state().liquid());
        assert!(Block::LAVA.default_state().liquid());

        let waterlogged = Block::OAK_SLAB
            .default_state()
            .with(Properties::WATERLOGGED, true);
        assert!(!waterlogged.liquid());
        assert!(waterlogged.has_fluid());
    }

    #[test]
    fn flags_vary_per_state() {
        let bottom = Block::OAK_SLAB
            .default_state()
            .with(Properties::SLAB_TYPE, SlabType::Bottom);
        let double = Block::OAK_SLAB
            .default_state()
            .with(Properties::SLAB_TYPE, SlabType::Double);

        assert!(!bottom.is_solid_render());
        assert!(double.is_solid_render());
    }

    #[test]
    fn the_default_state_is_air() {
        assert_eq!(BlockState::default(), Block::AIR.default_state());
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use crate::world::block::Properties;

    #[test]
    fn parses_a_plain_block_name() {
        crate::registry::registries::register_all();
        assert_eq!(
            BlockState::parse("minecraft:stone"),
            Some(Block::STONE.default_state())
        );
        assert_eq!(
            BlockState::parse("stone"),
            Some(Block::STONE.default_state())
        );
        assert_eq!(BlockState::parse("minecraft:not_a_block"), None);
    }

    #[test]
    fn parses_properties() {
        crate::registry::registries::register_all();
        let state = BlockState::parse("minecraft:oak_log[axis=x]").expect("should parse");

        assert_eq!(state.as_block(), Block::OAK_LOG);
        assert_eq!(state.get_by_name("axis"), Some("x"));
    }

    #[test]
    fn ignores_unknown_properties_and_values() {
        crate::registry::registries::register_all();
        let default_axis = Block::OAK_LOG.default_state().get_by_name("axis");

        let unknown = BlockState::parse("minecraft:oak_log[nope=1,axis=nope]").expect("parses");
        assert_eq!(unknown.get_by_name("axis"), default_axis);
    }

    #[test]
    fn copies_shared_properties_between_blocks() {
        let slab = Block::STONE_BRICK_SLAB
            .default_state()
            .with(Properties::WATERLOGGED, true);

        let mossy = slab.copy_properties_to(Block::MOSSY_STONE_BRICK_SLAB);

        assert_eq!(mossy.as_block(), Block::MOSSY_STONE_BRICK_SLAB);
        assert_eq!(mossy.get_by_name("waterlogged"), Some("true"));
    }

    #[test]
    fn with_properties_sets_each_named_property() {
        let state = Block::OAK_LOG
            .default_state()
            .with_properties([("axis", "z"), ("nope", "1")]);

        assert_eq!(state.get_by_name("axis"), Some("z"));
    }
}
