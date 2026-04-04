use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Once, OnceLock},
};

use crate::{
    registry::RegistryHolder,
    util::{Direction, Identifier},
};

/// Represents a single block state.
#[derive(Clone, Copy)]
pub struct BlockState {
    id: u16,
    block: Block,
}

impl BlockState {
    /// Returns the [BlockState] corrosponding to the given state id.
    /// If no corrosponding block state is found, *None* is returned.
    pub fn from_id(id: u16) -> Option<BlockState> {
        let Some(registry) = REGISTY.get() else {
            return None;
        };

        let Some(block_id) = registry.state_to_id.get(id as usize) else {
            return None;
        };

        Block::from_id(*block_id).map(|block| BlockState { id, block })
    }

    /// Returns the [BlockState] corrosponding to the given key.
    /// If no corrosponding block state is found, *None* is returned.
    pub fn from_key(key: impl Into<Identifier>) -> Option<BlockState> {
        Block::from_key(key).map(|block| block.default_state())
    }

    /// Returns the id of the block state.
    ///
    /// Note: this returns the id of the **block state** not the block.
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn block(&self) -> Block {
        self.block
    }

    pub fn block_entity(&self) -> Option<&BlockEntityInfo> {
        None
    }

    pub fn get_property<T, P>(&self, property: &P) -> Option<T>
    where
        P: Property<T>,
    {
        let block = &self.block;

        let property_index = block
            .properties
            .iter()
            .position(|prop| prop.name() == property.as_any().name())
            .expect("Property not found on this block");

        let base_state_id = block.min_state_id;
        let relative_index = self.id - base_state_id;

        let mut index = relative_index;
        let mut property_value_index = 0;

        for (i, prop) in block.properties.iter().enumerate().rev() {
            let count = prop.possible_values().len() as u16;
            let current_index = (index % count) as usize;

            if i == property_index {
                property_value_index = current_index;
            }
            index /= count;
        }

        Some(property.value_from_index(property_value_index))
    }

    pub fn has_property<T, P>(&self, property: &P) -> bool
    where
        P: Property<T>,
    {
        self.block
            .properties()
            .iter()
            .find(|p| p.name() == property.name())
            .is_some()
    }

    pub fn set_property<T, P>(&mut self, property: &P, value: T)
    where
        P: Property<T>,
    {
        let block = self.block;

        // Find the property index in the block's property list
        let property_index = block
            .properties
            .iter()
            .position(|prop| prop.name() == property.as_any().name())
            .unwrap_or_else(|| {
                panic!(
                    "Property {} not found on block {}",
                    property.as_any().name(),
                    block.id
                )
            });

        // Get the base state ID for this block (O(1) lookup)

        let base_state_id = block.min_state_id;

        // Calculate the relative state index
        let relative_index = self.id - base_state_id;

        // Decode all property indices from the relative state index

        let mut index = relative_index;
        let mut property_indices = vec![0; block.properties.len()];

        for (i, prop) in block.properties.iter().enumerate().rev() {
            let count = prop.possible_values().len() as u16;
            property_indices[i] = (index % count) as usize;
            index /= count;
        }

        // Update the specific property's index
        let new_value_index = property.get_internal_index(&value);
        property_indices[property_index] = new_value_index;

        // Re-encode the property indices back to a state ID
        let (new_relative_index, _) = property_indices
            .iter()
            .zip(block.properties.iter())
            .rev()
            .fold(
                (0u16, 1u16),
                |(current_index, multiplier), (&value_idx, prop)| {
                    let count = prop.possible_values().len() as u16;
                    (
                        current_index + value_idx as u16 * multiplier,
                        multiplier * count,
                    )
                },
            );

        self.id = base_state_id + new_relative_index;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityInfo {
    pub namespace: String,
    pub id: i32,
}

impl AsRef<BlockState> for BlockState {
    fn as_ref(&self) -> &BlockState {
        self
    }
}

impl From<Block> for BlockState {
    fn from(value: Block) -> Self {
        value.default_state()
    }
}

impl From<&BlockState> for BlockState {
    fn from(value: &BlockState) -> Self {
        *value
    }
}

macro_rules! define_blocks {
    (pub enum Block {
        $($name:ident = $const:ident = $key:expr),* $(,)?
    }) => {

        $(static $const: BlockHolder = BlockHolder::new($key);)*

        #[allow(unused)]
        #[derive(Debug, Clone, Copy)]
        pub enum Block {
            $($name),*
        }

        impl Block {
            #[inline]
            const fn this(&self) -> &__private::Block {
                match self {
                    $(
                        Block::$name => &$const,
                    )*
                }
            }
        }

    };
}

include!("../registry/generated/blocks.rs");

impl Block {
    /// Returns the [Block] corrosponding to the given id.
    /// If no corrosponding block is found, *None* is returned.
    pub fn from_id(id: u16) -> Option<Block> {
        let Some(registry) = REGISTY.get() else {
            return None;
        };

        let Some(index) = registry.by_id.get(id as usize) else {
            return None;
        };

        registry.entries.get(*index as usize).copied()
    }

    /// Returns the [Block] corrosponding to the given key.
    /// If no corrosponding block is found, *None* is returned.
    pub fn from_key(key: impl Into<Identifier>) -> Option<Block> {
        let Some(registry) = REGISTY.get() else {
            return None;
        };

        let Some(index) = registry.by_key.get(&key.into().to_string()) else {
            return None;
        };

        registry.entries.get(*index as usize).copied()
    }

    pub fn default_state(&self) -> BlockState {
        BlockState {
            id: self.this().default_state,
            block: *self,
        }
    }
}

impl Deref for Block {
    type Target = __private::Block;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.this()
    }
}

mod __private {
    use crate::world::block_odl::AnyProperty;

    #[derive(Clone)]
    pub struct Block {
        pub id: u16,
        pub default_state: u16,
        pub properties: &'static [&'static dyn AnyProperty],
        pub min_state_id: u16,
    }

    impl Block {
        pub fn id(&self) -> u16 {
            self.id
        }

        pub fn properties(&self) -> &'static [&'static dyn AnyProperty] {
            self.properties
        }
    }
}

fn init(registry: &mut BlockRegistry) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        __init(registry);
    });
}

pub static REGISTY: OnceLock<BlockRegistry> = OnceLock::new();

pub struct BlockRegistry {
    entries: Vec<Block>,
    by_id: Vec<usize>,
    by_key: HashMap<String, usize>,
    state_to_id: Vec<u16>,
    next_state_id: u16,
}

impl BlockRegistry {
    pub fn init() {
        let mut this = Self {
            entries: Vec::new(),
            by_id: Vec::new(),
            by_key: HashMap::new(),
            state_to_id: Vec::new(),
            next_state_id: 0,
        };
        init(&mut this);
        REGISTY.get_or_init(|| this);
    }

    pub(crate) fn register(
        &mut self,
        holder: &BlockHolder,
        bock_enum: Block,
        key: &str,
        mut block: __private::Block,
    ) {
        let index = self.entries.len();

        self.by_id.push(index);
        self.by_key.insert(key.to_owned(), index);

        let mut state_count = 1;
        for property in block.properties {
            state_count *= property.possible_values().len();
        }

        let min_state_id = self.next_state_id;

        for _ in 0..state_count {
            // self.state_to_block_lookup.push(block);
            self.state_to_id.push(index as u16);
        }

        self.next_state_id += u16::try_from(state_count).unwrap();
        block.min_state_id = min_state_id;
        holder.set(block);
        self.entries.push(bock_enum);
    }
}

type BlockHolder = RegistryHolder<__private::Block>;

pub trait Property<T>
where
    Self: Sized + AnyProperty,
{
    fn possible_values(&self) -> Box<[T]>;

    fn value_from_index(&self, index: usize) -> T;

    fn get_internal_index(&self, value: &T) -> usize;

    fn as_any(&self) -> &dyn AnyProperty {
        self
    }
}

pub trait AnyProperty: Sync {
    fn name(&self) -> &'static str;
    fn possible_values(&self) -> Box<[&str]>;
}

#[derive(Debug, Clone)]
pub struct BoolProperty {
    name: &'static str,
}

impl BoolProperty {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Property<bool> for BoolProperty {
    fn possible_values(&self) -> Box<[bool]> {
        Box::new([false, true])
    }

    fn get_internal_index(&self, value: &bool) -> usize {
        if *value { 0 } else { 1 }
    }

    fn value_from_index(&self, index: usize) -> bool {
        index == 0
    }
}

impl AnyProperty for BoolProperty {
    fn name(&self) -> &'static str {
        self.name
    }

    fn possible_values(&self) -> Box<[&str]> {
        ["false", "true"].into()
    }
}

pub struct StringProperty {
    name: &'static str,
    possible_values: &'static [&'static str],
}

impl StringProperty {
    pub const fn new(name: &'static str, possible_values: &'static [&'static str]) -> Self {
        Self {
            name,
            possible_values,
        }
    }
}

impl Property<&'static str> for StringProperty {
    fn possible_values(&self) -> Box<[&'static str]> {
        self.possible_values.iter().map(|v| *v).collect()
    }

    fn get_internal_index(&self, value: &&'static str) -> usize {
        self.possible_values
            .iter()
            .position(|v| v == value)
            .unwrap()
    }

    fn value_from_index(&self, index: usize) -> &'static str {
        self.possible_values[index]
    }
}

impl AnyProperty for StringProperty {
    fn name(&self) -> &'static str {
        self.name
    }

    fn possible_values(&self) -> Box<[&str]> {
        self.possible_values.iter().map(|v| *v).collect()
    }
}

pub trait PropertyEnum: Sync {
    fn as_str(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct EnumProperty<T: PropertyEnum + 'static> {
    name: &'static str,
    possible_values: &'static [T],
}

impl<T: PropertyEnum> EnumProperty<T> {
    pub const fn new(name: &'static str, possible_values: &'static [T]) -> Self {
        Self {
            name,
            possible_values,
        }
    }
}

impl<T: PropertyEnum + Clone + Copy + PartialEq> Property<T> for EnumProperty<T> {
    fn possible_values(&self) -> Box<[T]> {
        self.possible_values.into()
    }

    fn get_internal_index(&self, value: &T) -> usize {
        self.possible_values
            .iter()
            .position(|v| v == value)
            .unwrap()
    }

    fn value_from_index(&self, index: usize) -> T {
        self.possible_values[index]
    }
}

impl<T: PropertyEnum + Clone> AnyProperty for EnumProperty<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn possible_values(&self) -> Box<[&str]> {
        self.possible_values.iter().map(|v| v.as_str()).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl TryFrom<i32> for BlockFace {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Bottom,
            1 => Self::Top,
            2 => Self::North,
            3 => Self::South,
            4 => Self::West,
            5 => Self::East,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachFace {
    Floor,
    Wall,
    Ceiling,
}

impl PropertyEnum for AttachFace {
    fn as_str(&self) -> &str {
        match self {
            AttachFace::Floor => "floor",
            AttachFace::Wall => "wall",
            AttachFace::Ceiling => "ceiling",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl PropertyEnum for Axis {
    fn as_str(&self) -> &str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }
}

impl PropertyEnum for Direction {
    fn as_str(&self) -> &str {
        match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::North => "north",
            Direction::South => "south",
            Direction::West => "west",
            Direction::East => "east",
        }
    }
}

static NUM_STR: [&str; 26] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25",
];

#[derive(Debug, Clone)]
pub struct IntProperty {
    pub min: u8,
    pub max: u8,
    pub name: &'static str,
}

impl IntProperty {
    pub const fn new(name: &'static str, min: u8, max: u8) -> Self {
        Self { name, min, max }
    }
}

impl AnyProperty for IntProperty {
    fn possible_values(&self) -> Box<[&str]> {
        (self.min..=self.max).map(|v| NUM_STR[v as usize]).collect()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl Property<u8> for IntProperty {
    fn possible_values(&self) -> Box<[u8]> {
        (self.min..=self.max).collect()
    }

    fn get_internal_index(&self, value: &u8) -> usize {
        if *value <= self.max {
            (*value - self.min) as usize
        } else {
            0
        }
    }

    fn value_from_index(&self, index: usize) -> u8 {
        self.min + (index as u8)
    }
}

macro_rules! property {
    (pub enum $enum:ident {
        $($name:ident = $const:expr),* $(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $enum {
             $($name),*
        }

        impl $enum {
            pub const fn values() -> &'static [Self] {
                &[$(Self::$name),*]
            }
        }

        impl PropertyEnum for $enum {
            fn as_str(&self) -> &str {
                match self {
                    $(Self::$name => $const),*
                }
            }
        }
    };
}

property! {
    pub enum Orientation {
        NorthUp = "north_up",
        EastUp = "east_up",
        SouthUp = "south_up",
        WestUp = "west_up",
        UpNorth = "up_north",
        UpEast = "up_east",
        UpSouth = "up_south",
        UpWest = "up_west",
        NorthEast = "north_east",
        NorthWest = "north_west",
        SouthEast = "south_east",
        SouthWest = "south_west",
    }
}

property! {
    pub enum Instrument {
        Harp = "harp",
        Basedrum = "basedrum",
        Snare = "snare",
        Hat = "hat",
        Bass = "bass",
        Flute = "flute",
        Bell = "bell",
        Guitar = "guitar",
        Chime = "chime",
        Xylophone = "xylophone",
        IronXylophone = "iron_xylophone",
        CowBell = "cow_bell",
        Didgeridoo = "didgeridoo",
        Bit = "bit",
        Banjo = "banjo",
        Pling = "pling",
        Zombie = "zombie",
        Skeleton = "skeleton",
        Creeper = "creeper",
        Dragon = "dragon",
        WitherSkeleton = "wither_skeleton",
        Piglin = "piglin",
        CustomHead = "custom_head",
    }
}

property! {
    pub enum Attachment {
        Floor = "floor",
        Ceiling = "ceiling",
        SingleWall = "single_wall",
        DoubleWall = "double_wall",
    }
}

property! {
    pub enum RailShape {
        NorthSouth = "north_south",
        EastWest = "east_west",
        AscendingEast = "ascending_east",
        AscendingWest = "ascending_west",
        AscendingNorth = "ascending_north",
        AscendingSouth = "ascending_south",
        SouthEast = "south_east",
        SouthWest = "south_west",
        NorthWest = "north_west",
        NorthEast = "north_east",
    }
}

property! {
    pub enum WallShape {
        None = "none",
        Low = "low",
        Tall = "tall",
    }
}

property! {
    pub enum WireConnection {
        Up = "up",
        Side = "side",
        None = "none",
    }
}

property! {
    pub enum BedPart {
        Head = "head",
        Foot = "foot",
    }
}

property! {
    pub enum ChestType {
        Single = "single",
        Left = "left",
        Right = "right",
    }
}

property! {
    pub enum StairsShape {
        Straight = "straight",
        InnerLeft = "inner_left",
        InnerRight = "inner_right",
        OuterLeft = "outer_left",
        OuterRight = "outer_right",
    }
}

property! {
    pub enum TrialSpawnerState {
        Inactive = "inactive",
        WaitingForPlayers = "waiting_for_players",
        Active = "active",
        WaitingForRewardEjection = "waiting_for_reward_ejection",
        EjectingReward = "ejecting_reward",
        Cooldown = "cooldown",
    }
}

property! {
    pub enum VaultState {
        Inactive = "inactive",
        Active = "active",
        Unlocking = "unlocking",
        Ejecting = "ejecting",
    }
}

property! {
    pub enum DoubleBlockHalf {
        Upper = "upper",
        Lower = "lower",
    }
}

property! {
    pub enum BlockHalf {
        Top = "top",
        Bottom = "bottom",
    }
}

property! {
    pub enum SideChainPart {
        Unconnected = "unconnected",
        Right = "right",
        Center = "center",
        Left = "left"
    }
}

property! {
    pub enum ComparatorMode {
        Compare = "compare",
        Subtract = "subtract",
    }
}

property! {
    pub enum DoorHinge {
        Left = "left",
        Right = "right",
    }
}

property! {
    pub enum Tilt {
        None = "unconnected",
        Unstable = "unstable",
        Partial = "partial",
        Full = "full"
    }
}

property! {
    pub enum BambooLeaves {
        None = "none",
        Small = "small",
        Large = "large",
    }
}

property! {
    pub enum DripstoneThickness {
        TipMerge = "tip_merge",
        Tip = "tip",
        Frustum = "frustum",
        Middle = "middle",
        Base = "base"
    }
}

property! {
    pub enum PistonType {
        Default = "normal",
        Sticky = "sticky",
    }
}

property! {
    pub enum SlabType {
        Top = "normal",
        Bottom = "sticky",
        Double = "double"
    }
}

property! {
    pub enum StructureBlockMode {
        Save = "save",
        Load = "load",
        Corner = "corner",
        Data = "data",
    }
}

property! {
    pub enum SculkSensorPhase {
        Inactive = "save",
        Active = "load",
        Cooldown = "cooldown",
    }
}

property! {
    pub enum CreakingHeartState {
        Uprooted = "Uprooted",
        Dormant = "dormant",
        Awake = "awake",
    }
}

property! {
    pub enum TestBlockMode {
        Start = "start",
        Log = "log",
        Fail = "fail",
        Accept = "accept",
    }
}

property! {
    pub enum CopperGolemPose {
        Standing = "standing",
        Sitting = "sitting",
        Running = "running",
        Star = "star",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        BlockRegistry::init();

        let mut block = Block::CommandBlock.default_state();
        println!("default_state: {:?}", block.id);
        println!(
            "CONDITIONAL: {:?}",
            block.get_property(&BlockProperty::CONDITIONAL)
        );
        println!("FACING: {:?}", block.get_property(&BlockProperty::FACING));
        block.set_property(&BlockProperty::FACING, "east");
        block.set_property(&BlockProperty::CONDITIONAL, "true");
        println!("new state: {:?}", block.id);
        println!(
            "CONDITIONAL: {:?}",
            block.get_property(&BlockProperty::CONDITIONAL)
        );
        println!("FACING: {:?}", block.get_property(&BlockProperty::FACING));
    }
}

pub struct BlockProperty;
// p![bloom]
#[rustfmt::skip]
#[allow(unused)]
impl BlockProperty {
    pub const ATTACHED: BoolProperty = BoolProperty::new("attached");
    pub const BERRIES: BoolProperty = BoolProperty::new("berries");
    pub const BLOOM: BoolProperty = BoolProperty::new("bloom");
    pub const BOTTOM: BoolProperty = BoolProperty::new("bottom");
    pub const CAN_SUMMON: BoolProperty = BoolProperty::new("can_summon");
    pub const CONDITIONAL: StringProperty = StringProperty::new("conditional", &["true", "false"]);
    pub const CONDITIONAL2: BoolProperty = BoolProperty::new("conditional");
    pub const DISARMED: BoolProperty = BoolProperty::new("disarmed");
    pub const DRAG: BoolProperty = BoolProperty::new("drag");
    pub const ENABLED: BoolProperty = BoolProperty::new("enabled");
    pub const EXTENDED: BoolProperty = BoolProperty::new("extended");
    pub const EYE: BoolProperty = BoolProperty::new("eye");
    pub const FALLING: BoolProperty = BoolProperty::new("falling");
    pub const HANGING: BoolProperty = BoolProperty::new("hanging");
    pub const HAS_BOTTLE_0: BoolProperty = BoolProperty::new("has_bottle_0");
    pub const HAS_BOTTLE_1: BoolProperty = BoolProperty::new("has_bottle_1");
    pub const HAS_BOTTLE_2: BoolProperty = BoolProperty::new("has_bottle_2");
    pub const HAS_RECORD: BoolProperty = BoolProperty::new("has_record");
    pub const HAS_BOOK: BoolProperty = BoolProperty::new("has_book");
    pub const INVERTED: BoolProperty = BoolProperty::new("inverted");
    pub const IN_WALL: BoolProperty = BoolProperty::new("in_wall");
    pub const LIT: BoolProperty = BoolProperty::new("lit");
    pub const LOCKED: BoolProperty = BoolProperty::new("locked");
    pub const NATURAL: BoolProperty = BoolProperty::new("natural");
    pub const OCCUPIED: BoolProperty = BoolProperty::new("occupied");
    pub const OPEN: BoolProperty = BoolProperty::new("open");
    pub const PERSISTENT: BoolProperty = BoolProperty::new("persistent");
    pub const POWERED: BoolProperty = BoolProperty::new("powered");
    pub const SHORT: BoolProperty = BoolProperty::new("short");
    pub const SHRIEKING: BoolProperty = BoolProperty::new("shrieking");
    pub const SIGNAL_FIRE: BoolProperty = BoolProperty::new("signal_fire");
    pub const SNOWY: BoolProperty = BoolProperty::new("snowy");
    pub const TIP: BoolProperty = BoolProperty::new("tip");
    pub const TRIGGERED: BoolProperty = BoolProperty::new("triggered");
    pub const UNSTABLE: BoolProperty = BoolProperty::new("unstable");
    pub const WATERLOGGED: BoolProperty = BoolProperty::new("waterlogged");
    pub const HORIZONTAL_AXIS: EnumProperty<Axis> = EnumProperty::new("axis", &[Axis::X, Axis::Z]);
    pub const AXIS: EnumProperty<Axis> = EnumProperty::new("axis", &[Axis::X, Axis::Y, Axis::Z]);
    pub const UP: BoolProperty = BoolProperty::new("up");
    pub const DOWN: BoolProperty = BoolProperty::new("down");
    pub const NORTH: BoolProperty = BoolProperty::new("north");
    pub const EAST: BoolProperty = BoolProperty::new("east");
    pub const SOUTH: BoolProperty = BoolProperty::new("south");
    pub const WEST: BoolProperty = BoolProperty::new("west");
    pub const FACING: StringProperty = StringProperty::new("facing", &[
        "north", "east", "south", "west", "up", "down",
    ]);
    pub const FACING2: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::Up,
            Direction::Down,
        ],
    );
    pub const FACING_HOPPER: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::Down,
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    );
    pub const HORIZONTAL_FACING: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ],
    );
    pub const FLOWER_AMOUNT: IntProperty = IntProperty::new("flower_amount", 1, 4);
    pub const SEGMENT_AMOUNT: IntProperty = IntProperty::new("segment_amount", 1, 4);

    // Additional enum types needed for properties
    pub const ORIENTATION: EnumProperty<Orientation> = EnumProperty::new("orientation", Orientation::values());
    pub const ATTACH_FACE: EnumProperty<AttachFace> = EnumProperty::new("face", &[AttachFace::Floor, AttachFace::Wall, AttachFace::Ceiling]);
    pub const BELL_ATTACHMENT: EnumProperty<Attachment> = EnumProperty::new("attachment", Attachment::values());

    pub const EAST_WALL: EnumProperty<WallShape> = EnumProperty::new("east", WallShape::values());
    pub const NORTH_WALL: EnumProperty<WallShape> = EnumProperty::new("north", WallShape::values());
    pub const SOUTH_WALL: EnumProperty<WallShape> = EnumProperty::new("south", WallShape::values());
    pub const WEST_WALL: EnumProperty<WallShape> = EnumProperty::new("west", WallShape::values());

    pub const EAST_REDSTONE: EnumProperty<WireConnection> = EnumProperty::new("east", WireConnection::values());
    pub const NORTH_REDSTONE: EnumProperty<WireConnection> = EnumProperty::new("north", WireConnection::values());
    pub const SOUTH_REDSTONE: EnumProperty<WireConnection> = EnumProperty::new("south", WireConnection::values());
    pub const WEST_REDSTONE: EnumProperty<WireConnection> = EnumProperty::new("west", WireConnection::values());

    pub const DOUBLE_BLOCK_HALF: EnumProperty<DoubleBlockHalf> = EnumProperty::new("half", DoubleBlockHalf::values());
    pub const HALF: EnumProperty<BlockHalf> = EnumProperty::new("half", BlockHalf::values());
    pub const SIDE_CHAIN_PART: EnumProperty<SideChainPart> = EnumProperty::new("side_chain", SideChainPart::values());
    pub const RAIL_SHAPE: EnumProperty<RailShape> = EnumProperty::new("shape", RailShape::values());
    pub const RAIL_SHAPE_STRAIGHT: EnumProperty<RailShape> = EnumProperty::new(
        "shape",
        &[
            RailShape::NorthSouth,
            RailShape::EastWest,
            RailShape::AscendingEast,
            RailShape::AscendingWest,
            RailShape::AscendingNorth,
            RailShape::AscendingSouth,
        ],
    );

    // Age properties
    pub const AGE_1: IntProperty = IntProperty::new("age", 0, 1);
    pub const AGE_2: IntProperty = IntProperty::new("age", 0, 2);
    pub const AGE_3: IntProperty = IntProperty::new("age", 0, 3);
    pub const AGE_4: IntProperty = IntProperty::new("age", 0, 4);
    pub const AGE_5: IntProperty = IntProperty::new("age", 0, 5);
    pub const AGE_7: IntProperty = IntProperty::new("age", 0, 7);
    pub const AGE_15: IntProperty = IntProperty::new("age", 0, 15);
    pub const AGE_25: IntProperty = IntProperty::new("age", 0, 25);

    // Other integer properties
    pub const BITES: IntProperty = IntProperty::new("bites", 0, 6);
    pub const CANDLES: IntProperty = IntProperty::new("candles", 1, 4);
    pub const DELAY: IntProperty = IntProperty::new("delay", 1, 4);
    pub const DISTANCE: IntProperty = IntProperty::new("distance", 1, 7);
    pub const EGGS: IntProperty = IntProperty::new("eggs", 1, 4);
    pub const HATCH: IntProperty = IntProperty::new("hatch", 0, 2);
    pub const LAYERS: IntProperty = IntProperty::new("layers", 1, 8);
    pub const LEVEL_CAULDRON: IntProperty = IntProperty::new("level", 1, 3);
    pub const LEVEL_COMPOSTER: IntProperty = IntProperty::new("level", 0, 8);
    pub const LEVEL_FLOWING: IntProperty = IntProperty::new("level", 1, 8);
    pub const LEVEL_HONEY: IntProperty = IntProperty::new("honey_level", 0, 5);
    pub const LEVEL: IntProperty = IntProperty::new("level", 0, 15);
    pub const MOISTURE: IntProperty = IntProperty::new("moisture", 0, 7);
    pub const NOTE: IntProperty = IntProperty::new("note", 0, 24);
    pub const PICKLES: IntProperty = IntProperty::new("pickles", 1, 4);
    pub const POWER: IntProperty = IntProperty::new("power", 0, 15);
    pub const STAGE: IntProperty = IntProperty::new("stage", 0, 1);
    pub const STABILITY_DISTANCE: IntProperty = IntProperty::new("distance", 0, 7);
    pub const RESPAWN_ANCHOR_CHARGES: IntProperty = IntProperty::new("charges", 0, 4);
    pub const DRIED_GHAST_HYDRATION_LEVELS: IntProperty = IntProperty::new("hydration", 0, 3);
    pub const ROTATION_16: IntProperty = IntProperty::new("rotation", 0, 15);
    pub const DUSTED: IntProperty = IntProperty::new("dusted", 0, 3);

    // Enum properties
    pub const PART: EnumProperty<BedPart> = EnumProperty::new("part", BedPart::values());
    pub const CHEST_TYPE: EnumProperty<ChestType> = EnumProperty::new("type", ChestType::values());
    pub const MODE_COMPARATOR: EnumProperty<ComparatorMode> = EnumProperty::new("mode", ComparatorMode::values());
    pub const HINGE: EnumProperty<DoorHinge> = EnumProperty::new("hinge", DoorHinge::values());
    pub const NOTEBLOCK_INSTRUMENT: EnumProperty<Instrument> = EnumProperty::new("instrument", Instrument::values());
    pub const PISTON_TYPE: EnumProperty<PistonType> = EnumProperty::new("type", PistonType::values());
    pub const SLAB_TYPE: EnumProperty<SlabType> = EnumProperty::new("type", SlabType::values());
    pub const STAIRS_SHAPE: EnumProperty<StairsShape> = EnumProperty::new("shape", StairsShape::values());
    pub const STRUCTUREBLOCK_MODE: EnumProperty<StructureBlockMode> = EnumProperty::new("mode", StructureBlockMode::values());
    pub const BAMBOO_LEAVES: EnumProperty<BambooLeaves> = EnumProperty::new("leaves", BambooLeaves::values());
    pub const TILT: EnumProperty<Tilt> = EnumProperty::new("tilt", Tilt::values());
    pub const VERTICAL_DIRECTION: EnumProperty<Direction> = EnumProperty::new("vertical_direction", &[Direction::Up, Direction::Down]);
    pub const THICKNESS: EnumProperty<DripstoneThickness> = EnumProperty::new("thickness", DripstoneThickness::values());
    pub const SCULK_SENSOR_PHASE: EnumProperty<SculkSensorPhase> = EnumProperty::new("sculk_sensor_phase",SculkSensorPhase::values());
    pub const TRIAL_SPAWNER_STATE: EnumProperty<TrialSpawnerState> = EnumProperty::new("trial_spawner_state", TrialSpawnerState::values());
    pub const VAULT_STATE: EnumProperty<VaultState> = EnumProperty::new("vault_state", VaultState::values());
    pub const CREAKING_HEART_STATE: EnumProperty<CreakingHeartState> = EnumProperty::new("creaking", CreakingHeartState::values());
    pub const TEST_BLOCK_MODE: EnumProperty<TestBlockMode> = EnumProperty::new("mode", TestBlockMode::values());
    pub const COPPER_GOLEM_POSE: EnumProperty<CopperGolemPose> = EnumProperty::new("copper_golem_pose",CopperGolemPose::values());

    // Additional boolean properties
    pub const SLOT_0_OCCUPIED: BoolProperty = BoolProperty::new("slot_0_occupied");
    pub const SLOT_1_OCCUPIED: BoolProperty = BoolProperty::new("slot_1_occupied");
    pub const SLOT_2_OCCUPIED: BoolProperty = BoolProperty::new("slot_2_occupied");
    pub const SLOT_3_OCCUPIED: BoolProperty = BoolProperty::new("slot_3_occupied");
    pub const SLOT_4_OCCUPIED: BoolProperty = BoolProperty::new("slot_4_occupied");
    pub const SLOT_5_OCCUPIED: BoolProperty = BoolProperty::new("slot_5_occupied");
    pub const CRACKED: BoolProperty = BoolProperty::new("cracked");
    pub const CRAFTING: BoolProperty = BoolProperty::new("crafting");
    pub const OMINOUS: BoolProperty = BoolProperty::new("ominous");
    pub const MAP: BoolProperty = BoolProperty::new("map");
}
