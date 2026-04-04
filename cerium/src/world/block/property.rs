use crate::util::Direction;

pub trait AnyProperty: Sync + Send + Sized {
    fn name(&self) -> &'static str;
    fn value_count(&self) -> u16;
}

pub struct DynamicProperty {
    pub name: &'static str,
    pub count: u16,
}

impl AnyProperty for DynamicProperty {
    fn name(&self) -> &'static str {
        self.name
    }
    fn value_count(&self) -> u16 {
        self.count
    }
}

pub trait Property: 'static {
    type Value: Clone + PartialEq + std::fmt::Debug;
    const NAME: &'static str;

    fn from_index(index: usize) -> Option<Self::Value>;
    fn to_index(value: &Self::Value) -> usize;
}

#[macro_export]
macro_rules! p {
    [$name:ident] => {
        $crate::world::block::property::$name
    };
}

pub use p;

macro_rules! bool_property {
    ($name:ident, $id:expr) => {
        pub struct $name;
        impl Property for $name {
            type Value = bool;
            const NAME: &'static str = $id;
            fn from_index(index: usize) -> Option<bool> {
                match index {
                    0 => Some(true),
                    1 => Some(false),
                    _ => None,
                }
            }
            fn to_index(value: &bool) -> usize {
                if *value { 0 } else { 1 }
            }
        }
        impl AnyProperty for $name {
            fn name(&self) -> &'static str {
                $id
            }
            fn value_count(&self) -> u16 {
                2
            }
        }
    };
}

macro_rules! int_property {
    ($name:ident, $id:expr, $min:expr, $max:expr) => {
        pub struct $name;
        impl Property for $name {
            type Value = u8;
            const NAME: &'static str = $id;
            fn from_index(index: usize) -> Option<u8> {
                Some($min + index as u8)
            }
            fn to_index(value: &u8) -> usize {
                (*value - $min) as usize
            }
        }
        impl AnyProperty for $name {
            fn name(&self) -> &'static str {
                $id
            }
            fn value_count(&self) -> u16 {
                ($max - $min + 1) as u16
            }
        }
    };
}

macro_rules! enum_property {
    ($name:ident, $ty:ty, $id:expr, [$($val:expr),* $(,)?]) => {
        pub struct $name;
        impl Property for $name {
            type Value = $ty;
            const NAME: &'static str = $id;
            fn from_index(index: usize) -> Option<$ty> {
                [$($val),*].get(index).copied()
            }
            fn to_index(value: &$ty) -> usize {
                [$($val),*].iter().position(|v| v == value).unwrap()
            }
        }
        impl AnyProperty for $name {
            fn name(&self) -> &'static str { $id }
            fn value_count(&self) -> u16 { [$($val),*].len() as u16 }
        }
    };
}

macro_rules! define_properties {
    () => {};
    (Bool ($name:ident, $str:expr) $($rest:tt)*) => {
        bool_property!($name, $str);
        define_properties!($($rest)*);
    };
    (Int  ($name:ident, $str:expr) = $min:tt ..= $max:tt $($rest:tt)*) => {
        int_property!($name, $str, $min, $max);
        define_properties!($($rest)*);
    };
    (Enum ($name:ident, $str:expr) = $ty:ty, [ $( $val:expr ),* $(,)? ] $($rest:tt)*) => {
        enum_property!($name, $ty, $str, [ $( $val ),* ]);
        define_properties!($($rest)*);
    };
}

define_properties! {

    // Bool Properties

    Bool (Attached,        "attached")
    Bool (Berries,         "berries")
    Bool (Bloom,           "bloom")
    Bool (Bottom,          "bottom")
    Bool (CanSummon,       "can_summon")
    Bool (Conditional,     "conditional")
    Bool (Disarmed,        "disarmed")
    Bool (Drag,            "drag")
    Bool (Enabled,         "enabled")
    Bool (Extended,        "extended")
    Bool (Eye,             "eye")
    Bool (Falling,         "falling")
    Bool (Hanging,         "hanging")
    Bool (HasBottle0,      "has_bottle_0")
    Bool (HasBottle1,      "has_bottle_1")
    Bool (HasBottle2,      "has_bottle_2")
    Bool (HasRecord,       "has_record")
    Bool (HasBook,         "has_book")
    Bool (Inverted,        "inverted")
    Bool (InWall,          "in_wall")
    Bool (Lit,             "lit")
    Bool (Locked,          "locked")
    Bool (Natural,         "natural")
    Bool (Occupied,        "occupied")
    Bool (Open,            "open")
    Bool (Persistent,      "persistent")
    Bool (Powered,         "powered")
    Bool (Short,           "short")
    Bool (Shrieking,       "shrieking")
    Bool (SignalFire,      "signal_fire")
    Bool (Snowy,           "snowy")
    Bool (Tip,             "tip")
    Bool (Triggered,       "triggered")
    Bool (Unstable,        "unstable")
    Bool (Waterlogged,     "waterlogged")
    Bool (Up,              "up")
    Bool (Down,            "down")
    Bool (North,           "north")
    Bool (East,            "east")
    Bool (South,           "south")
    Bool (West,            "west")
    Bool (SlotOccupied0,   "slot_0_occupied")
    Bool (SlotOccupied1,   "slot_1_occupied")
    Bool (SlotOccupied2,   "slot_2_occupied")
    Bool (SlotOccupied3,   "slot_3_occupied")
    Bool (SlotOccupied4,   "slot_4_occupied")
    Bool (SlotOccupied5,   "slot_5_occupied")
    Bool (Cracked,         "cracked")
    Bool (Crafting,        "crafting")
    Bool (Ominous,         "ominous")
    Bool (Map,             "map")

    // Int Properties

    Int (FlowerAmount,          "flower_amount")    = 1..=4
    Int (SegmentAmount,         "segment_amount")   = 1..=4
    Int (Age1,                  "age")              = 0..=1
    Int (Age2,                  "age")              = 0..=2
    Int (Age3,                  "age")              = 0..=3
    Int (Age4,                  "age")              = 0..=4
    Int (Age5,                  "age")              = 0..=5
    Int (Age7,                  "age")              = 0..=7
    Int (Age15,                 "age")              = 0..=15
    Int (Age25,                 "age")              = 0..=25
    Int (Bites,                 "bites")            = 0..=6
    Int (Candles,               "candles")          = 0..=15
    Int (Delay,                 "delay")            = 1..=4
    Int (Distance,              "distance")         = 1..=7
    Int (Eggs,                  "eggs")             = 1..=4
    Int (Hatch,                 "hatch")            = 0..=2
    Int (Layers,                "layers")           = 1..=8
    Int (LevelCauldron,         "level")            = 1..=3
    Int (LevelComposter,        "level")            = 0..=8
    Int (LevelFlowing,          "level")            = 1..=8
    Int (LevelHoney,            "honey_level")      = 0..=5
    Int (Level,                 "level")            = 0..=15
    Int (Moisture,              "moisture")         = 0..=7
    Int (Note,                  "note")             = 0..=24
    Int (Pickles,               "pickles")          = 1..=4
    Int (Power,                 "power")            = 0..=15
    Int (Stage,                 "stage")            = 0..=1
    Int (StabilityDistance,     "distance")         = 0..=7
    Int (RespawnAnchorCharges,  "charges")          = 0..=4
    Int (DriedGhastHydration,   "hydration")        = 0..=3
    Int (Rotation16,            "rotation")         = 0..=15
    Int (Dusted,                "dusted")           = 0..=3

    // Enum Properties

    Enum (Facing2,              "facing")               = Direction, [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down]
    Enum (FacingHopper,         "facing")               = Direction, [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Down ]
    Enum (HorizontalFacing,     "facing")               = Direction, [Direction::North, Direction::East, Direction::South, Direction::West]
    Enum (VerticalDirection,    "vertical_direction")   = Direction, [Direction::Up, Direction::Down]
    Enum (BambooLeaves,         "leaves")               = crate::world::block::BambooLeaves, [crate::world::block::BambooLeaves::None, crate::world::block::BambooLeaves::Small, crate::world::block::BambooLeaves::Large]
    // Enum (HorizontalAxis,       "axis")                 = Axis, [Axis::X, Axis::Z]
    // Enum (Axis,                 "axis")                 = Axis, [Axis::X, Axis::Y, Axis::Z]

    // Enum (Orientation, "orientation") = Orientation, [...]
    // Enum (AttachFace,  "face")        = AttachFace,  [...]
    // Enum (BellAttachment, "attachment") = Attachment, [...]
    // Enum (EastWall,  "east")  = WallShape, [...]
    // Enum (NorthWall, "north") = WallShape, [...]
    // Enum (SouthWall, "south") = WallShape, [...]
    // Enum (WestWall,  "west")  = WallShape, [...]
    // Enum (EastRedstone,  "east")  = WireConnection, [...]
    // Enum (NorthRedstone, "north") = WireConnection, [...]
    // Enum (SouthRedstone, "south") = WireConnection, [...]
    // Enum (WestRedstone,  "west")  = WireConnection, [...]
    // Enum (DoubleBlockHalf, "half") = DoubleBlockHalf, [...]
    // Enum (Half,            "half") = BlockHalf, [...]
    // Enum (SideChainPart, "side_chain") = SideChainPart, [...]
    // Enum (RailShape, "shape") = RailShape, [...]
    // Enum (RailShapeStraight, "shape") = RailShape, [...]
    // Enum (Part, "part") = BedPart, [...]
    // Enum (ChestType, "type") = ChestType, [...]
    // Enum (ModeComparator, "mode") = ComparatorMode, [...]
    // Enum (Hinge, "hinge") = DoorHinge, [...]
    // Enum (NoteblockInstrument, "instrument") = Instrument, [...]
    // Enum (PistonType, "type") = PistonType, [...]
    // Enum (SlabType, "type") = SlabType, [...]
    // Enum (StairsShape, "shape") = StairsShape, [...]
    // Enum (StructureblockMode, "mode") = StructureBlockMode, [...]
    // Enum (Tilt, "tilt") = Tilt, [...]
    // Enum (Thickness, "thickness") = DripstoneThickness, [...]
    // Enum (SculkSensorPhase, "sculk_sensor_phase") = SculkSensorPhase, [...]
    // Enum (TrialSpawnerState, "trial_spawner_state") = TrialSpawnerState, [...]
    // Enum (VaultState, "vault_state") = VaultState, [...]
    // Enum (CreakingHeartState, "creaking_heart_state") = CreakingHeartState, [...]
    // Enum (TestBlockMode, "mode") = TestBlockMode, [...]
    // Enum (CopperGolemPose, "copper_golem_pose") = CopperGolemPose, [...]
}
