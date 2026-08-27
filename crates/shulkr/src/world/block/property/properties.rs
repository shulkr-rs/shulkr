use shulkr_macros::PropertyEnum;

use crate::{
    util::{Axis, Direction},
    world::block::property::{
        BoolProperty, IntProperty,
        types::{EnumProperty, PropertyEnum},
    },
};

pub struct Properties;

#[expect(dead_code, reason = "")]
#[rustfmt::skip]
impl Properties {
    pub const ATTACHED: BoolProperty        = BoolProperty::new("attached");
    pub const BERRIES: BoolProperty         = BoolProperty::new("berries");
    pub const BLOOM: BoolProperty           = BoolProperty::new("bloom");
    pub const BOTTOM: BoolProperty          = BoolProperty::new("bottom");
    pub const CAN_SUMMON: BoolProperty      = BoolProperty::new("can_summon");
    pub const CONDITIONAL: BoolProperty     = BoolProperty::new("conditional");
    pub const CRACKED: BoolProperty         = BoolProperty::new("cracked");
    pub const CRAFTING: BoolProperty        = BoolProperty::new("crafting");
    pub const DISARMED: BoolProperty        = BoolProperty::new("disarmed");
    pub const DOWN: BoolProperty            = BoolProperty::new("down");
    pub const DRAG: BoolProperty            = BoolProperty::new("drag");
    pub const EAST: BoolProperty            = BoolProperty::new("east");
    pub const ENABLED: BoolProperty         = BoolProperty::new("enabled");
    pub const EXTENDED: BoolProperty        = BoolProperty::new("extended");
    pub const EYE: BoolProperty             = BoolProperty::new("eye");
    pub const FALLING: BoolProperty         = BoolProperty::new("falling");
    pub const HANGING: BoolProperty         = BoolProperty::new("hanging");
    pub const HAS_BOOK: BoolProperty        = BoolProperty::new("has_book");
    pub const HAS_BOTTLE_0: BoolProperty    = BoolProperty::new("has_bottle_0");
    pub const HAS_BOTTLE_1: BoolProperty    = BoolProperty::new("has_bottle_1");
    pub const HAS_BOTTLE_2: BoolProperty    = BoolProperty::new("has_bottle_2");
    pub const HAS_RECORD: BoolProperty      = BoolProperty::new("has_record");
    pub const INVERTED: BoolProperty        = BoolProperty::new("inverted");
    pub const IN_WALL: BoolProperty         = BoolProperty::new("in_wall");
    pub const LIT: BoolProperty             = BoolProperty::new("lit");
    pub const LOCKED: BoolProperty          = BoolProperty::new("locked");
    pub const MAP: BoolProperty             = BoolProperty::new("map");
    pub const NATURAL: BoolProperty         = BoolProperty::new("natural");
    pub const NORTH: BoolProperty           = BoolProperty::new("north");
    pub const OCCUPIED: BoolProperty        = BoolProperty::new("occupied");
    pub const OMINOUS: BoolProperty         = BoolProperty::new("ominous");
    pub const OPEN: BoolProperty            = BoolProperty::new("open");
    pub const PERSISTENT: BoolProperty      = BoolProperty::new("persistent");
    pub const POWERED: BoolProperty         = BoolProperty::new("powered");
    pub const SHORT: BoolProperty           = BoolProperty::new("short");
    pub const SHRIEKING: BoolProperty       = BoolProperty::new("shrieking");
    pub const SIGNAL_FIRE: BoolProperty     = BoolProperty::new("signal_fire");
    pub const SLOT_0_OCCUPIED: BoolProperty = BoolProperty::new("slot_0_occupied");
    pub const SLOT_1_OCCUPIED: BoolProperty = BoolProperty::new("slot_1_occupied");
    pub const SLOT_2_OCCUPIED: BoolProperty = BoolProperty::new("slot_2_occupied");
    pub const SLOT_3_OCCUPIED: BoolProperty = BoolProperty::new("slot_3_occupied");
    pub const SLOT_4_OCCUPIED: BoolProperty = BoolProperty::new("slot_4_occupied");
    pub const SLOT_5_OCCUPIED: BoolProperty = BoolProperty::new("slot_5_occupied");
    pub const SNOWY: BoolProperty           = BoolProperty::new("snowy");
    pub const SOUTH: BoolProperty           = BoolProperty::new("south");
    pub const TIP: BoolProperty             = BoolProperty::new("tip");
    pub const TRIGGERED: BoolProperty       = BoolProperty::new("triggered");
    pub const UNSTABLE: BoolProperty        = BoolProperty::new("unstable");
    pub const UP: BoolProperty              = BoolProperty::new("up");
    pub const WATERLOGGED: BoolProperty     = BoolProperty::new("waterlogged");
    pub const WEST: BoolProperty            = BoolProperty::new("west");
}

#[expect(dead_code, reason = "")]
#[rustfmt::skip]
impl Properties {
    pub const AGE_1: IntProperty                = IntProperty::new("age", 0, 1);
    pub const AGE_15: IntProperty               = IntProperty::new("age", 0, 15);
    pub const AGE_2: IntProperty                = IntProperty::new("age", 0, 2);
    pub const AGE_25: IntProperty               = IntProperty::new("age", 0, 25);
    pub const AGE_3: IntProperty                = IntProperty::new("age", 0, 3);
    pub const AGE_4: IntProperty                = IntProperty::new("age", 0, 4);
    pub const AGE_5: IntProperty                = IntProperty::new("age", 0, 5);
    pub const AGE_7: IntProperty                = IntProperty::new("age", 0, 7);
    pub const BITES: IntProperty                = IntProperty::new("bites", 0, 6);
    pub const CANDLES: IntProperty              = IntProperty::new("candles", 1, 4);
    pub const DELAY: IntProperty                = IntProperty::new("delay", 1, 4);
    pub const DISTANCE: IntProperty             = IntProperty::new("distance", 1, 7);
    pub const DRIED_GHAST_HYDRATION_LEVELS: IntProperty = IntProperty::new("hydration", 0, 3);
    pub const DUSTED: IntProperty               = IntProperty::new("dusted", 0, 3);
    pub const EGGS: IntProperty                 = IntProperty::new("eggs", 1, 4);
    pub const FLOWER_AMOUNT: IntProperty        = IntProperty::new("flower_amount", 1, 4);
    pub const HATCH: IntProperty                = IntProperty::new("hatch", 0, 2);
    pub const LAYERS: IntProperty               = IntProperty::new("layers", 1, 8);
    pub const LEVEL: IntProperty                = IntProperty::new("level", 0, 15);
    pub const LEVEL_CAULDRON: IntProperty       = IntProperty::new("level", 1, 3);
    pub const LEVEL_COMPOSTER: IntProperty      = IntProperty::new("level", 0, 8);
    pub const LEVEL_FLOWING: IntProperty        = IntProperty::new("level", 1, 8);
    pub const LEVEL_HONEY: IntProperty          = IntProperty::new("honey_level", 0, 5);
    pub const MOISTURE: IntProperty             = IntProperty::new("moisture", 0, 7);
    pub const NOTE: IntProperty                 = IntProperty::new("note", 0, 24);
    pub const PICKLES: IntProperty              = IntProperty::new("pickles", 1, 4);
    pub const POWER: IntProperty                = IntProperty::new("power", 0, 15);
    pub const RESPAWN_ANCHOR_CHARGES: IntProperty = IntProperty::new("charges", 0, 4);
    pub const ROTATION_16: IntProperty          = IntProperty::new("rotation", 0, 15);
    pub const SEGMENT_AMOUNT: IntProperty       = IntProperty::new("segment_amount", 1, 4);
    pub const STABILITY_DISTANCE: IntProperty   = IntProperty::new("distance", 0, 7);
    pub const STAGE: IntProperty                = IntProperty::new("stage", 0, 1);
}

#[rustfmt::skip]
impl Properties {
    pub const ATTACH_FACE: EnumProperty<AttachFace> = EnumProperty::new(
        "face",
        &[
            AttachFace::Floor,
            AttachFace::Wall,
            AttachFace::Ceiling
        ],
    );
    pub const AXIS: EnumProperty<Axis> = EnumProperty::new(
        "axis",
        &[
            Axis::X,
            Axis::Y,
            Axis::Z
        ]
    );
    pub const BAMBOO_LEAVES: EnumProperty<BambooLeaves> = EnumProperty::new(
        "leaves",
        &[
            BambooLeaves::None,
            BambooLeaves::Small,
            BambooLeaves::Large
        ],
    );
    pub const BED_PART: EnumProperty<BedPart> = EnumProperty::new(
        "part",
        &[
            BedPart::Head,
            BedPart::Foot
        ]
    );
    pub const BELL_ATTACHMENT: EnumProperty<BellAttachType> = EnumProperty::new(
        "attachment",
        &[
            BellAttachType::Floor,
            BellAttachType::Ceiling,
            BellAttachType::SingleWall,
            BellAttachType::DoubleWall,
        ],
    );
    pub const CHEST_TYPE: EnumProperty<ChestType> = EnumProperty::new(
        "type",
        &[
            ChestType::Single,
            ChestType::Left,
            ChestType::Right
        ],
    );
    pub const COPPER_GOLEM_POSE: EnumProperty<CopperGolemPose> = EnumProperty::new(
        "copper_golem_pose",
        &[
            CopperGolemPose::Standing,
            CopperGolemPose::Sitting,
            CopperGolemPose::Running,
            CopperGolemPose::Star
        ],
    );
    pub const CREAKING_HEART_STATE: EnumProperty<CreakingHeartState> = EnumProperty::new(
        "creaking_heart_state",
        &[
            CreakingHeartState::Uprooted,
            CreakingHeartState::Dormant,
            CreakingHeartState::Awake,
        ],
    );
    pub const DOOR_HINGE: EnumProperty<DoorHingeSide> = EnumProperty::new(
        "hinge",
        &[
            DoorHingeSide::Left,
            DoorHingeSide::Right
        ]
    );
    pub const DOUBLE_BLOCK_HALF: EnumProperty<DoubleBlockHalf> = EnumProperty::new(
        "half",
        &[
            DoubleBlockHalf::Upper,
            DoubleBlockHalf::Lower
        ]
    );
    pub const EAST_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "east",
        &[
            RedstoneSide::Up,
            RedstoneSide::Side,
            RedstoneSide::None
        ],
    );
    pub const EAST_WALL: EnumProperty<WallSide> = EnumProperty::new(
        "east",
        &[
            WallSide::None,
            WallSide::Low,
            WallSide::Tall
        ]
    );
    pub const FACING: EnumProperty<Direction> = EnumProperty::new(
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
    pub const HALF: EnumProperty<Half> = EnumProperty::new(
        "half",
        &[
            Half::Top,
            Half::Bottom
        ]
    );
    pub const HORIZONTAL_AXIS: EnumProperty<Axis> = EnumProperty::new(
        "axis",
        &[
            Axis::X,
            Axis::Z
        ]
    );
    pub const HORIZONTAL_FACING: EnumProperty<Direction> = EnumProperty::new(
        "facing",
        &[
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    );
    pub const MODE_COMPARATOR: EnumProperty<ComparatorMode> = EnumProperty::new(
        "mode",
        &[
            ComparatorMode::Compare,
            ComparatorMode::Subtract
        ]
    );
    pub const NORTH_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "north",
        &[
            RedstoneSide::Up,
            RedstoneSide::Side,
            RedstoneSide::None
        ],
    );
    pub const NORTH_WALL: EnumProperty<WallSide> = EnumProperty::new(
        "north",
        &[
            WallSide::None,
            WallSide::Low,
            WallSide::Tall
        ]
    );
    pub const NOTEBLOCK_INSTRUMENT: EnumProperty<NoteBlockInstrument> = EnumProperty::new(
        "instrument",
        &[
            NoteBlockInstrument::Harp,
            NoteBlockInstrument::Basedrum,
            NoteBlockInstrument::Snare,
            NoteBlockInstrument::Hat,
            NoteBlockInstrument::Bass,
            NoteBlockInstrument::Flute,
            NoteBlockInstrument::Bell,
            NoteBlockInstrument::Guitar,
            NoteBlockInstrument::Chime,
            NoteBlockInstrument::Xylophone,
            NoteBlockInstrument::IronXylophone,
            NoteBlockInstrument::CowBell,
            NoteBlockInstrument::Didgeridoo,
            NoteBlockInstrument::Bit,
            NoteBlockInstrument::Banjo,
            NoteBlockInstrument::Pling,
            NoteBlockInstrument::Trumpet,
            NoteBlockInstrument::TrumpetExposed,
            NoteBlockInstrument::TrumpetOxidized,
            NoteBlockInstrument::TrumpetWeathered,
            NoteBlockInstrument::Zombie,
            NoteBlockInstrument::Skeleton,
            NoteBlockInstrument::Creeper,
            NoteBlockInstrument::Dragon,
            NoteBlockInstrument::WitherSkeleton,
            NoteBlockInstrument::Piglin,
            NoteBlockInstrument::CustomHead,
        ],
    );
    pub const ORIENTATION: EnumProperty<FrontAndTop> = EnumProperty::new(
        "orientation",
        &[
            FrontAndTop::DownEast,
            FrontAndTop::DownNorth,
            FrontAndTop::DownSouth,
            FrontAndTop::DownWest,
            FrontAndTop::UpEast,
            FrontAndTop::UpNorth,
            FrontAndTop::UpSouth,
            FrontAndTop::UpWest,
            FrontAndTop::WestUp,
            FrontAndTop::EastUp,
            FrontAndTop::NorthUp,
            FrontAndTop::SouthUp,
        ],
    );
    pub const PISTON_TYPE: EnumProperty<PistonType> = EnumProperty::new(
        "type",
        &[
            PistonType::Normal,
            PistonType::Sticky
        ]
    );
    pub const POTENT_SULFUR_STATE: EnumProperty<PotentSulfurState> = EnumProperty::new(
        "potent_sulfur_state",
        &[
            PotentSulfurState::Dry,
            PotentSulfurState::Wet,
            PotentSulfurState::Dormant,
            PotentSulfurState::Erupting,
            PotentSulfurState::Continuous,
        ],
    );
    pub const RAIL_SHAPE: EnumProperty<RailShape> = EnumProperty::new(
        "shape",
        &[
            RailShape::NorthSouth,
            RailShape::EastWest,
            RailShape::AscendingEast,
            RailShape::AscendingWest,
            RailShape::AscendingNorth,
            RailShape::AscendingSouth,
            RailShape::SouthEast,
            RailShape::SouthWest,
            RailShape::NorthWest,
            RailShape::NorthEast,
        ],
    );
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
    pub const SCULK_SENSOR_PHASE: EnumProperty<SculkSensorPhase> = EnumProperty::new(
        "sculk_sensor_phase",
        &[
            SculkSensorPhase::Inactive,
            SculkSensorPhase::Active,
            SculkSensorPhase::Cooldown,
        ],
    );
    pub const SIDE_CHAIN_PART: EnumProperty<SideChainPart> = EnumProperty::new(
        "side_chain",
        &[
            SideChainPart::Unconnected,
            SideChainPart::Right,
            SideChainPart::Center,
            SideChainPart::Left,
        ],
    );
    pub const SLAB_TYPE: EnumProperty<SlabType> = EnumProperty::new(
        "type",
        &[
            SlabType::Top,
            SlabType::Bottom,
            SlabType::Double
        ]
    );
    pub const SOUTH_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "south",
        &[
            RedstoneSide::Up,
            RedstoneSide::Side,
            RedstoneSide::None
        ],
    );
    pub const SOUTH_WALL: EnumProperty<WallSide> = EnumProperty::new(
        "south",
        &[
            WallSide::None,
            WallSide::Low,
            WallSide::Tall
        ]
    );
    pub const SPELEOTHEM_THICKNESS: EnumProperty<SpeleothemThickness> = EnumProperty::new(
        "thickness",
        &[
            SpeleothemThickness::TipMerge,
            SpeleothemThickness::Tip,
            SpeleothemThickness::Frustum,
            SpeleothemThickness::Middle,
            SpeleothemThickness::Base,
        ],
    );
    pub const STAIRS_SHAPE: EnumProperty<StairsShape> = EnumProperty::new(
        "shape",
        &[
            StairsShape::Straight,
            StairsShape::InnerLeft,
            StairsShape::InnerRight,
            StairsShape::OuterLeft,
            StairsShape::OuterRight,
        ],
    );
    pub const STRUCTUREBLOCK_MODE: EnumProperty<StructureMode> = EnumProperty::new(
        "mode",
        &[
            StructureMode::Save,
            StructureMode::Load,
            StructureMode::Corner,
            StructureMode::Data,
        ],
    );
    pub const TEST_BLOCK_MODE: EnumProperty<TestBlockMode> = EnumProperty::new(
        "mode",
        &[
            TestBlockMode::Start,
            TestBlockMode::Log,
            TestBlockMode::Fail,
            TestBlockMode::Accept,
        ],
    );
    pub const TILT: EnumProperty<Tilt> = EnumProperty::new(
        "tilt",
        &[
            Tilt::None,
            Tilt::Unstable,
            Tilt::Partial,
            Tilt::Full
        ],
    );
    pub const TRIAL_SPAWNER_STATE: EnumProperty<TrialSpawnerState> = EnumProperty::new(
        "trial_spawner_state",
        &[
            TrialSpawnerState::Inactive,
            TrialSpawnerState::WaitingForPlayers,
            TrialSpawnerState::Active,
            TrialSpawnerState::WaitingForRewardEjection,
            TrialSpawnerState::EjectingReward,
            TrialSpawnerState::Cooldown,
        ],
    );
    pub const VAULT_STATE: EnumProperty<VaultState> = EnumProperty::new(
        "vault_state",
        &[
            VaultState::Inactive,
            VaultState::Active,
            VaultState::Unlocking,
            VaultState::Ejecting,
        ],
    );
    pub const VERTICAL_DIRECTION: EnumProperty<Direction> = EnumProperty::new(
        "vertical_direction",
        &[
            Direction::Up,
            Direction::Down
        ]
    );
    pub const WEST_REDSTONE: EnumProperty<RedstoneSide> = EnumProperty::new(
        "west",
        &[
            RedstoneSide::Up,
            RedstoneSide::Side,
            RedstoneSide::None
        ],
    );
    pub const WEST_WALL: EnumProperty<WallSide> = EnumProperty::new(
        "west",
        &[
            WallSide::None,
            WallSide::Low,
            WallSide::Tall
        ]
    );
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

impl PropertyEnum for Axis {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum CopperGolemPose {
    Standing,
    Sitting,
    Running,
    Star,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum PotentSulfurState {
    Dry,
    Wet,
    Dormant,
    Erupting,
    Continuous,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum TestBlockMode {
    Start,
    Log,
    Fail,
    Accept,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum CreakingHeartState {
    Uprooted,
    Dormant,
    Awake,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum VaultState {
    Inactive,
    Active,
    Unlocking,
    Ejecting,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum TrialSpawnerState {
    Inactive,
    WaitingForPlayers,
    Active,
    WaitingForRewardEjection,
    EjectingReward,
    Cooldown,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum SpeleothemThickness {
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum SculkSensorPhase {
    Inactive,
    Active,
    Cooldown,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum Tilt {
    None,
    Unstable,
    Partial,
    Full,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum StairsShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum SlabType {
    Bottom,
    Top,
    Double,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum PistonType {
    Normal,
    Sticky,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum NoteBlockInstrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Trumpet,
    TrumpetExposed,
    TrumpetOxidized,
    TrumpetWeathered,
    Zombie,
    Skeleton,
    Creeper,
    Dragon,
    WitherSkeleton,
    Piglin,
    CustomHead,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum DoorHingeSide {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum ChestType {
    Single,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum BedPart {
    Head,
    Foot,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum SideChainPart {
    Unconnected,
    Right,
    Center,
    Left,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum DoubleBlockHalf {
    Upper,
    Lower,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum RedstoneSide {
    Up,
    Side,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum WallSide {
    None,
    Low,
    Tall,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum BellAttachType {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum AttachFace {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Clone, Debug, PartialEq, Eq, PropertyEnum)]
pub enum FrontAndTop {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}
