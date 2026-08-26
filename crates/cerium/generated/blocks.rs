// This file was auto-generated. Do not edit it manually.

use crate::registry::Registry;
use cerium_macros::StaticObject;
use cerium_macros::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StaticObject, UnitEnum)]
#[repr(u16)]
pub enum Block {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    CherryPlanks,
    DarkOakPlanks,
    PaleOakWood,
    PaleOakPlanks,
    MangrovePlanks,
    BambooPlanks,
    BambooMosaic,
    OakSapling,
    SpruceSapling,
    BirchSapling,
    JungleSapling,
    AcaciaSapling,
    CherrySapling,
    DarkOakSapling,
    PaleOakSapling,
    MangrovePropagule,
    Bedrock,
    Water,
    Lava,
    Sand,
    SuspiciousSand,
    RedSand,
    Gravel,
    SuspiciousGravel,
    GoldOre,
    DeepslateGoldOre,
    IronOre,
    DeepslateIronOre,
    CoalOre,
    DeepslateCoalOre,
    NetherGoldOre,
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    AcaciaLog,
    CherryLog,
    DarkOakLog,
    PaleOakLog,
    MangroveLog,
    MangroveRoots,
    MuddyMangroveRoots,
    BambooBlock,
    StrippedSpruceLog,
    StrippedBirchLog,
    StrippedJungleLog,
    StrippedAcaciaLog,
    StrippedCherryLog,
    StrippedDarkOakLog,
    StrippedPaleOakLog,
    StrippedOakLog,
    StrippedMangroveLog,
    StrippedBambooBlock,
    OakWood,
    SpruceWood,
    BirchWood,
    JungleWood,
    AcaciaWood,
    CherryWood,
    DarkOakWood,
    MangroveWood,
    StrippedOakWood,
    StrippedSpruceWood,
    StrippedBirchWood,
    StrippedJungleWood,
    StrippedAcaciaWood,
    StrippedCherryWood,
    StrippedDarkOakWood,
    StrippedPaleOakWood,
    StrippedMangroveWood,
    OakLeaves,
    SpruceLeaves,
    BirchLeaves,
    JungleLeaves,
    AcaciaLeaves,
    CherryLeaves,
    DarkOakLeaves,
    PaleOakLeaves,
    MangroveLeaves,
    AzaleaLeaves,
    FloweringAzaleaLeaves,
    Sponge,
    WetSponge,
    Glass,
    LapisOre,
    DeepslateLapisOre,
    LapisBlock,
    Dispenser,
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    NoteBlock,
    WhiteBed,
    OrangeBed,
    MagentaBed,
    LightBlueBed,
    YellowBed,
    LimeBed,
    PinkBed,
    GrayBed,
    LightGrayBed,
    CyanBed,
    PurpleBed,
    BlueBed,
    BrownBed,
    GreenBed,
    RedBed,
    BlackBed,
    PoweredRail,
    DetectorRail,
    StickyPiston,
    Cobweb,
    ShortGrass,
    Fern,
    DeadBush,
    Bush,
    ShortDryGrass,
    TallDryGrass,
    Seagrass,
    TallSeagrass,
    Piston,
    PistonHead,
    WhiteWool,
    OrangeWool,
    MagentaWool,
    LightBlueWool,
    YellowWool,
    LimeWool,
    PinkWool,
    GrayWool,
    LightGrayWool,
    CyanWool,
    PurpleWool,
    BlueWool,
    BrownWool,
    GreenWool,
    RedWool,
    BlackWool,
    MovingPiston,
    Dandelion,
    GoldenDandelion,
    Torchflower,
    Poppy,
    BlueOrchid,
    Allium,
    AzureBluet,
    RedTulip,
    OrangeTulip,
    WhiteTulip,
    PinkTulip,
    OxeyeDaisy,
    Cornflower,
    WitherRose,
    LilyOfTheValley,
    BrownMushroom,
    RedMushroom,
    GoldBlock,
    IronBlock,
    Bricks,
    Tnt,
    Bookshelf,
    ChiseledBookshelf,
    AcaciaShelf,
    BambooShelf,
    BirchShelf,
    CherryShelf,
    CrimsonShelf,
    DarkOakShelf,
    JungleShelf,
    MangroveShelf,
    OakShelf,
    PaleOakShelf,
    SpruceShelf,
    WarpedShelf,
    MossyCobblestone,
    Obsidian,
    Torch,
    WallTorch,
    Fire,
    SoulFire,
    Spawner,
    CreakingHeart,
    OakStairs,
    Chest,
    RedstoneWire,
    DiamondOre,
    DeepslateDiamondOre,
    DiamondBlock,
    CraftingTable,
    Wheat,
    Farmland,
    Furnace,
    OakSign,
    SpruceSign,
    BirchSign,
    AcaciaSign,
    CherrySign,
    JungleSign,
    DarkOakSign,
    PaleOakSign,
    MangroveSign,
    BambooSign,
    OakDoor,
    Ladder,
    Rail,
    CobblestoneStairs,
    OakWallSign,
    SpruceWallSign,
    BirchWallSign,
    AcaciaWallSign,
    CherryWallSign,
    JungleWallSign,
    DarkOakWallSign,
    PaleOakWallSign,
    MangroveWallSign,
    BambooWallSign,
    OakHangingSign,
    SpruceHangingSign,
    BirchHangingSign,
    AcaciaHangingSign,
    CherryHangingSign,
    JungleHangingSign,
    DarkOakHangingSign,
    PaleOakHangingSign,
    CrimsonHangingSign,
    WarpedHangingSign,
    MangroveHangingSign,
    BambooHangingSign,
    OakWallHangingSign,
    SpruceWallHangingSign,
    BirchWallHangingSign,
    AcaciaWallHangingSign,
    CherryWallHangingSign,
    JungleWallHangingSign,
    DarkOakWallHangingSign,
    PaleOakWallHangingSign,
    MangroveWallHangingSign,
    CrimsonWallHangingSign,
    WarpedWallHangingSign,
    BambooWallHangingSign,
    Lever,
    StonePressurePlate,
    IronDoor,
    OakPressurePlate,
    SprucePressurePlate,
    BirchPressurePlate,
    JunglePressurePlate,
    AcaciaPressurePlate,
    CherryPressurePlate,
    DarkOakPressurePlate,
    PaleOakPressurePlate,
    MangrovePressurePlate,
    BambooPressurePlate,
    RedstoneOre,
    DeepslateRedstoneOre,
    RedstoneTorch,
    RedstoneWallTorch,
    StoneButton,
    Snow,
    Ice,
    SnowBlock,
    Cactus,
    CactusFlower,
    Clay,
    SugarCane,
    Jukebox,
    OakFence,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    PolishedBasalt,
    SoulTorch,
    SoulWallTorch,
    CopperTorch,
    CopperWallTorch,
    Glowstone,
    NetherPortal,
    CarvedPumpkin,
    JackOLantern,
    Cake,
    Repeater,
    WhiteStainedGlass,
    OrangeStainedGlass,
    MagentaStainedGlass,
    LightBlueStainedGlass,
    YellowStainedGlass,
    LimeStainedGlass,
    PinkStainedGlass,
    GrayStainedGlass,
    LightGrayStainedGlass,
    CyanStainedGlass,
    PurpleStainedGlass,
    BlueStainedGlass,
    BrownStainedGlass,
    GreenStainedGlass,
    RedStainedGlass,
    BlackStainedGlass,
    OakTrapdoor,
    SpruceTrapdoor,
    BirchTrapdoor,
    JungleTrapdoor,
    AcaciaTrapdoor,
    CherryTrapdoor,
    DarkOakTrapdoor,
    PaleOakTrapdoor,
    MangroveTrapdoor,
    BambooTrapdoor,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    PackedMud,
    MudBricks,
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    BrownMushroomBlock,
    RedMushroomBlock,
    MushroomStem,
    IronBars,
    CopperBars,
    ExposedCopperBars,
    WeatheredCopperBars,
    OxidizedCopperBars,
    WaxedCopperBars,
    WaxedExposedCopperBars,
    WaxedWeatheredCopperBars,
    WaxedOxidizedCopperBars,
    IronChain,
    CopperChain,
    ExposedCopperChain,
    WeatheredCopperChain,
    OxidizedCopperChain,
    WaxedCopperChain,
    WaxedExposedCopperChain,
    WaxedWeatheredCopperChain,
    WaxedOxidizedCopperChain,
    GlassPane,
    Pumpkin,
    Melon,
    AttachedPumpkinStem,
    AttachedMelonStem,
    PumpkinStem,
    MelonStem,
    Vine,
    GlowLichen,
    ResinClump,
    OakFenceGate,
    BrickStairs,
    StoneBrickStairs,
    MudBrickStairs,
    Mycelium,
    LilyPad,
    ResinBlock,
    ResinBricks,
    ResinBrickStairs,
    ResinBrickSlab,
    ResinBrickWall,
    ChiseledResinBricks,
    NetherBricks,
    NetherBrickFence,
    NetherBrickStairs,
    NetherWart,
    EnchantingTable,
    BrewingStand,
    Cauldron,
    WaterCauldron,
    LavaCauldron,
    PowderSnowCauldron,
    EndPortal,
    EndPortalFrame,
    EndStone,
    DragonEgg,
    RedstoneLamp,
    Cocoa,
    SandstoneStairs,
    EmeraldOre,
    DeepslateEmeraldOre,
    EnderChest,
    TripwireHook,
    Tripwire,
    EmeraldBlock,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    MossyCobblestoneWall,
    FlowerPot,
    PottedTorchflower,
    PottedOakSapling,
    PottedSpruceSapling,
    PottedBirchSapling,
    PottedJungleSapling,
    PottedAcaciaSapling,
    PottedCherrySapling,
    PottedDarkOakSapling,
    PottedPaleOakSapling,
    PottedMangrovePropagule,
    PottedFern,
    PottedDandelion,
    PottedGoldenDandelion,
    PottedPoppy,
    PottedBlueOrchid,
    PottedAllium,
    PottedAzureBluet,
    PottedRedTulip,
    PottedOrangeTulip,
    PottedWhiteTulip,
    PottedPinkTulip,
    PottedOxeyeDaisy,
    PottedCornflower,
    PottedLilyOfTheValley,
    PottedWitherRose,
    PottedRedMushroom,
    PottedBrownMushroom,
    PottedDeadBush,
    PottedCactus,
    Carrots,
    Potatoes,
    OakButton,
    SpruceButton,
    BirchButton,
    JungleButton,
    AcaciaButton,
    CherryButton,
    DarkOakButton,
    PaleOakButton,
    MangroveButton,
    BambooButton,
    SkeletonSkull,
    SkeletonWallSkull,
    WitherSkeletonSkull,
    WitherSkeletonWallSkull,
    ZombieHead,
    ZombieWallHead,
    PlayerHead,
    PlayerWallHead,
    CreeperHead,
    CreeperWallHead,
    DragonHead,
    DragonWallHead,
    PiglinHead,
    PiglinWallHead,
    Anvil,
    ChippedAnvil,
    DamagedAnvil,
    TrappedChest,
    LightWeightedPressurePlate,
    HeavyWeightedPressurePlate,
    Comparator,
    DaylightDetector,
    RedstoneBlock,
    NetherQuartzOre,
    Hopper,
    QuartzBlock,
    ChiseledQuartzBlock,
    QuartzPillar,
    QuartzStairs,
    ActivatorRail,
    Dropper,
    WhiteTerracotta,
    OrangeTerracotta,
    MagentaTerracotta,
    LightBlueTerracotta,
    YellowTerracotta,
    LimeTerracotta,
    PinkTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    CyanTerracotta,
    PurpleTerracotta,
    BlueTerracotta,
    BrownTerracotta,
    GreenTerracotta,
    RedTerracotta,
    BlackTerracotta,
    WhiteStainedGlassPane,
    OrangeStainedGlassPane,
    MagentaStainedGlassPane,
    LightBlueStainedGlassPane,
    YellowStainedGlassPane,
    LimeStainedGlassPane,
    PinkStainedGlassPane,
    GrayStainedGlassPane,
    LightGrayStainedGlassPane,
    CyanStainedGlassPane,
    PurpleStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    GreenStainedGlassPane,
    RedStainedGlassPane,
    BlackStainedGlassPane,
    AcaciaStairs,
    CherryStairs,
    DarkOakStairs,
    PaleOakStairs,
    MangroveStairs,
    BambooStairs,
    BambooMosaicStairs,
    SlimeBlock,
    Barrier,
    Light,
    IronTrapdoor,
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs,
    PrismarineBrickStairs,
    DarkPrismarineStairs,
    PrismarineSlab,
    PrismarineBrickSlab,
    DarkPrismarineSlab,
    SeaLantern,
    HayBlock,
    WhiteCarpet,
    OrangeCarpet,
    MagentaCarpet,
    LightBlueCarpet,
    YellowCarpet,
    LimeCarpet,
    PinkCarpet,
    GrayCarpet,
    LightGrayCarpet,
    CyanCarpet,
    PurpleCarpet,
    BlueCarpet,
    BrownCarpet,
    GreenCarpet,
    RedCarpet,
    BlackCarpet,
    Terracotta,
    CoalBlock,
    PackedIce,
    Sunflower,
    Lilac,
    RoseBush,
    Peony,
    TallGrass,
    LargeFern,
    WhiteBanner,
    OrangeBanner,
    MagentaBanner,
    LightBlueBanner,
    YellowBanner,
    LimeBanner,
    PinkBanner,
    GrayBanner,
    LightGrayBanner,
    CyanBanner,
    PurpleBanner,
    BlueBanner,
    BrownBanner,
    GreenBanner,
    RedBanner,
    BlackBanner,
    WhiteWallBanner,
    OrangeWallBanner,
    MagentaWallBanner,
    LightBlueWallBanner,
    YellowWallBanner,
    LimeWallBanner,
    PinkWallBanner,
    GrayWallBanner,
    LightGrayWallBanner,
    CyanWallBanner,
    PurpleWallBanner,
    BlueWallBanner,
    BrownWallBanner,
    GreenWallBanner,
    RedWallBanner,
    BlackWallBanner,
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs,
    OakSlab,
    SpruceSlab,
    BirchSlab,
    JungleSlab,
    AcaciaSlab,
    CherrySlab,
    DarkOakSlab,
    PaleOakSlab,
    MangroveSlab,
    BambooSlab,
    BambooMosaicSlab,
    StoneSlab,
    SmoothStoneSlab,
    SandstoneSlab,
    CutSandstoneSlab,
    PetrifiedOakSlab,
    CobblestoneSlab,
    BrickSlab,
    StoneBrickSlab,
    MudBrickSlab,
    NetherBrickSlab,
    QuartzSlab,
    RedSandstoneSlab,
    CutRedSandstoneSlab,
    PurpurSlab,
    SmoothStone,
    SmoothSandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    AcaciaFenceGate,
    CherryFenceGate,
    DarkOakFenceGate,
    PaleOakFenceGate,
    MangroveFenceGate,
    BambooFenceGate,
    SpruceFence,
    BirchFence,
    JungleFence,
    AcaciaFence,
    CherryFence,
    DarkOakFence,
    PaleOakFence,
    MangroveFence,
    BambooFence,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    CherryDoor,
    DarkOakDoor,
    PaleOakDoor,
    MangroveDoor,
    BambooDoor,
    EndRod,
    ChorusPlant,
    ChorusFlower,
    PurpurBlock,
    PurpurPillar,
    PurpurStairs,
    EndStoneBricks,
    TorchflowerCrop,
    PitcherCrop,
    PitcherPlant,
    Beetroots,
    DirtPath,
    EndGateway,
    RepeatingCommandBlock,
    ChainCommandBlock,
    FrostedIce,
    MagmaBlock,
    NetherWartBlock,
    RedNetherBricks,
    BoneBlock,
    StructureVoid,
    Observer,
    ShulkerBox,
    WhiteShulkerBox,
    OrangeShulkerBox,
    MagentaShulkerBox,
    LightBlueShulkerBox,
    YellowShulkerBox,
    LimeShulkerBox,
    PinkShulkerBox,
    GrayShulkerBox,
    LightGrayShulkerBox,
    CyanShulkerBox,
    PurpleShulkerBox,
    BlueShulkerBox,
    BrownShulkerBox,
    GreenShulkerBox,
    RedShulkerBox,
    BlackShulkerBox,
    WhiteGlazedTerracotta,
    OrangeGlazedTerracotta,
    MagentaGlazedTerracotta,
    LightBlueGlazedTerracotta,
    YellowGlazedTerracotta,
    LimeGlazedTerracotta,
    PinkGlazedTerracotta,
    GrayGlazedTerracotta,
    LightGrayGlazedTerracotta,
    CyanGlazedTerracotta,
    PurpleGlazedTerracotta,
    BlueGlazedTerracotta,
    BrownGlazedTerracotta,
    GreenGlazedTerracotta,
    RedGlazedTerracotta,
    BlackGlazedTerracotta,
    WhiteConcrete,
    OrangeConcrete,
    MagentaConcrete,
    LightBlueConcrete,
    YellowConcrete,
    LimeConcrete,
    PinkConcrete,
    GrayConcrete,
    LightGrayConcrete,
    CyanConcrete,
    PurpleConcrete,
    BlueConcrete,
    BrownConcrete,
    GreenConcrete,
    RedConcrete,
    BlackConcrete,
    WhiteConcretePowder,
    OrangeConcretePowder,
    MagentaConcretePowder,
    LightBlueConcretePowder,
    YellowConcretePowder,
    LimeConcretePowder,
    PinkConcretePowder,
    GrayConcretePowder,
    LightGrayConcretePowder,
    CyanConcretePowder,
    PurpleConcretePowder,
    BlueConcretePowder,
    BrownConcretePowder,
    GreenConcretePowder,
    RedConcretePowder,
    BlackConcretePowder,
    Kelp,
    KelpPlant,
    DriedKelpBlock,
    TurtleEgg,
    SnifferEgg,
    DriedGhast,
    DeadTubeCoralBlock,
    DeadBrainCoralBlock,
    DeadBubbleCoralBlock,
    DeadFireCoralBlock,
    DeadHornCoralBlock,
    TubeCoralBlock,
    BrainCoralBlock,
    BubbleCoralBlock,
    FireCoralBlock,
    HornCoralBlock,
    DeadTubeCoral,
    DeadBrainCoral,
    DeadBubbleCoral,
    DeadFireCoral,
    DeadHornCoral,
    TubeCoral,
    BrainCoral,
    BubbleCoral,
    FireCoral,
    HornCoral,
    DeadTubeCoralFan,
    DeadBrainCoralFan,
    DeadBubbleCoralFan,
    DeadFireCoralFan,
    DeadHornCoralFan,
    TubeCoralFan,
    BrainCoralFan,
    BubbleCoralFan,
    FireCoralFan,
    HornCoralFan,
    DeadTubeCoralWallFan,
    DeadBrainCoralWallFan,
    DeadBubbleCoralWallFan,
    DeadFireCoralWallFan,
    DeadHornCoralWallFan,
    TubeCoralWallFan,
    BrainCoralWallFan,
    BubbleCoralWallFan,
    FireCoralWallFan,
    HornCoralWallFan,
    SeaPickle,
    BlueIce,
    Conduit,
    BambooSapling,
    Bamboo,
    PottedBamboo,
    VoidAir,
    CaveAir,
    BubbleColumn,
    PolishedGraniteStairs,
    SmoothRedSandstoneStairs,
    MossyStoneBrickStairs,
    PolishedDioriteStairs,
    MossyCobblestoneStairs,
    EndStoneBrickStairs,
    StoneStairs,
    SmoothSandstoneStairs,
    SmoothQuartzStairs,
    GraniteStairs,
    AndesiteStairs,
    RedNetherBrickStairs,
    PolishedAndesiteStairs,
    DioriteStairs,
    PolishedGraniteSlab,
    SmoothRedSandstoneSlab,
    MossyStoneBrickSlab,
    PolishedDioriteSlab,
    MossyCobblestoneSlab,
    EndStoneBrickSlab,
    SmoothSandstoneSlab,
    SmoothQuartzSlab,
    GraniteSlab,
    AndesiteSlab,
    RedNetherBrickSlab,
    PolishedAndesiteSlab,
    DioriteSlab,
    BrickWall,
    PrismarineWall,
    RedSandstoneWall,
    MossyStoneBrickWall,
    GraniteWall,
    StoneBrickWall,
    MudBrickWall,
    NetherBrickWall,
    AndesiteWall,
    RedNetherBrickWall,
    SandstoneWall,
    EndStoneBrickWall,
    DioriteWall,
    Scaffolding,
    Loom,
    Barrel,
    Smoker,
    BlastFurnace,
    CartographyTable,
    FletchingTable,
    Grindstone,
    Lectern,
    SmithingTable,
    Stonecutter,
    Bell,
    Lantern,
    SoulLantern,
    CopperLantern,
    ExposedCopperLantern,
    WeatheredCopperLantern,
    OxidizedCopperLantern,
    WaxedCopperLantern,
    WaxedExposedCopperLantern,
    WaxedWeatheredCopperLantern,
    WaxedOxidizedCopperLantern,
    Campfire,
    SoulCampfire,
    SweetBerryBush,
    WarpedStem,
    StrippedWarpedStem,
    WarpedHyphae,
    StrippedWarpedHyphae,
    WarpedNylium,
    WarpedFungus,
    WarpedWartBlock,
    WarpedRoots,
    NetherSprouts,
    CrimsonStem,
    StrippedCrimsonStem,
    CrimsonHyphae,
    StrippedCrimsonHyphae,
    CrimsonNylium,
    CrimsonFungus,
    Shroomlight,
    WeepingVines,
    WeepingVinesPlant,
    TwistingVines,
    TwistingVinesPlant,
    CrimsonRoots,
    CrimsonPlanks,
    WarpedPlanks,
    CrimsonSlab,
    WarpedSlab,
    CrimsonPressurePlate,
    WarpedPressurePlate,
    CrimsonFence,
    WarpedFence,
    CrimsonTrapdoor,
    WarpedTrapdoor,
    CrimsonFenceGate,
    WarpedFenceGate,
    CrimsonStairs,
    WarpedStairs,
    CrimsonButton,
    WarpedButton,
    CrimsonDoor,
    WarpedDoor,
    CrimsonSign,
    WarpedSign,
    CrimsonWallSign,
    WarpedWallSign,
    StructureBlock,
    Jigsaw,
    TestBlock,
    TestInstanceBlock,
    Composter,
    Target,
    BeeNest,
    Beehive,
    HoneyBlock,
    HoneycombBlock,
    NetheriteBlock,
    AncientDebris,
    CryingObsidian,
    RespawnAnchor,
    PottedCrimsonFungus,
    PottedWarpedFungus,
    PottedCrimsonRoots,
    PottedWarpedRoots,
    Lodestone,
    Blackstone,
    BlackstoneStairs,
    BlackstoneWall,
    BlackstoneSlab,
    PolishedBlackstone,
    PolishedBlackstoneBricks,
    CrackedPolishedBlackstoneBricks,
    ChiseledPolishedBlackstone,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneBrickStairs,
    PolishedBlackstoneBrickWall,
    GildedBlackstone,
    PolishedBlackstoneStairs,
    PolishedBlackstoneSlab,
    PolishedBlackstonePressurePlate,
    PolishedBlackstoneButton,
    PolishedBlackstoneWall,
    ChiseledNetherBricks,
    CrackedNetherBricks,
    QuartzBricks,
    Candle,
    WhiteCandle,
    OrangeCandle,
    MagentaCandle,
    LightBlueCandle,
    YellowCandle,
    LimeCandle,
    PinkCandle,
    GrayCandle,
    LightGrayCandle,
    CyanCandle,
    PurpleCandle,
    BlueCandle,
    BrownCandle,
    GreenCandle,
    RedCandle,
    BlackCandle,
    CandleCake,
    WhiteCandleCake,
    OrangeCandleCake,
    MagentaCandleCake,
    LightBlueCandleCake,
    YellowCandleCake,
    LimeCandleCake,
    PinkCandleCake,
    GrayCandleCake,
    LightGrayCandleCake,
    CyanCandleCake,
    PurpleCandleCake,
    BlueCandleCake,
    BrownCandleCake,
    GreenCandleCake,
    RedCandleCake,
    BlackCandleCake,
    AmethystBlock,
    BuddingAmethyst,
    AmethystCluster,
    LargeAmethystBud,
    MediumAmethystBud,
    SmallAmethystBud,
    Tuff,
    TuffSlab,
    TuffStairs,
    TuffWall,
    PolishedTuff,
    PolishedTuffSlab,
    PolishedTuffStairs,
    PolishedTuffWall,
    ChiseledTuff,
    TuffBricks,
    TuffBrickSlab,
    TuffBrickStairs,
    TuffBrickWall,
    ChiseledTuffBricks,
    Sulfur,
    PotentSulfur,
    SulfurSlab,
    SulfurStairs,
    SulfurWall,
    PolishedSulfur,
    PolishedSulfurSlab,
    PolishedSulfurStairs,
    PolishedSulfurWall,
    SulfurBricks,
    SulfurBrickSlab,
    SulfurBrickStairs,
    SulfurBrickWall,
    ChiseledSulfur,
    Cinnabar,
    CinnabarSlab,
    CinnabarStairs,
    CinnabarWall,
    PolishedCinnabar,
    PolishedCinnabarSlab,
    PolishedCinnabarStairs,
    PolishedCinnabarWall,
    CinnabarBricks,
    CinnabarBrickSlab,
    CinnabarBrickStairs,
    CinnabarBrickWall,
    ChiseledCinnabar,
    Calcite,
    TintedGlass,
    PowderSnow,
    SculkSensor,
    CalibratedSculkSensor,
    Sculk,
    SculkVein,
    SculkCatalyst,
    SculkShrieker,
    CopperBlock,
    ExposedCopper,
    WeatheredCopper,
    OxidizedCopper,
    WaxedCopperBlock,
    WaxedExposedCopper,
    WaxedWeatheredCopper,
    WaxedOxidizedCopper,
    CopperOre,
    DeepslateCopperOre,
    CutCopper,
    ExposedCutCopper,
    WeatheredCutCopper,
    OxidizedCutCopper,
    WaxedCutCopper,
    WaxedExposedCutCopper,
    WaxedWeatheredCutCopper,
    WaxedOxidizedCutCopper,
    ChiseledCopper,
    ExposedChiseledCopper,
    WeatheredChiseledCopper,
    OxidizedChiseledCopper,
    WaxedChiseledCopper,
    WaxedExposedChiseledCopper,
    WaxedWeatheredChiseledCopper,
    WaxedOxidizedChiseledCopper,
    CutCopperStairs,
    ExposedCutCopperStairs,
    WeatheredCutCopperStairs,
    OxidizedCutCopperStairs,
    WaxedCutCopperStairs,
    WaxedExposedCutCopperStairs,
    WaxedWeatheredCutCopperStairs,
    WaxedOxidizedCutCopperStairs,
    CutCopperSlab,
    ExposedCutCopperSlab,
    WeatheredCutCopperSlab,
    OxidizedCutCopperSlab,
    WaxedCutCopperSlab,
    WaxedExposedCutCopperSlab,
    WaxedWeatheredCutCopperSlab,
    WaxedOxidizedCutCopperSlab,
    CopperDoor,
    ExposedCopperDoor,
    WeatheredCopperDoor,
    OxidizedCopperDoor,
    WaxedCopperDoor,
    WaxedExposedCopperDoor,
    WaxedWeatheredCopperDoor,
    WaxedOxidizedCopperDoor,
    CopperTrapdoor,
    ExposedCopperTrapdoor,
    WeatheredCopperTrapdoor,
    OxidizedCopperTrapdoor,
    WaxedCopperTrapdoor,
    WaxedExposedCopperTrapdoor,
    WaxedWeatheredCopperTrapdoor,
    WaxedOxidizedCopperTrapdoor,
    CopperGrate,
    ExposedCopperGrate,
    WeatheredCopperGrate,
    OxidizedCopperGrate,
    WaxedCopperGrate,
    WaxedExposedCopperGrate,
    WaxedWeatheredCopperGrate,
    WaxedOxidizedCopperGrate,
    CopperBulb,
    ExposedCopperBulb,
    WeatheredCopperBulb,
    OxidizedCopperBulb,
    WaxedCopperBulb,
    WaxedExposedCopperBulb,
    WaxedWeatheredCopperBulb,
    WaxedOxidizedCopperBulb,
    CopperChest,
    ExposedCopperChest,
    WeatheredCopperChest,
    OxidizedCopperChest,
    WaxedCopperChest,
    WaxedExposedCopperChest,
    WaxedWeatheredCopperChest,
    WaxedOxidizedCopperChest,
    CopperGolemStatue,
    ExposedCopperGolemStatue,
    WeatheredCopperGolemStatue,
    OxidizedCopperGolemStatue,
    WaxedCopperGolemStatue,
    WaxedExposedCopperGolemStatue,
    WaxedWeatheredCopperGolemStatue,
    WaxedOxidizedCopperGolemStatue,
    LightningRod,
    ExposedLightningRod,
    WeatheredLightningRod,
    OxidizedLightningRod,
    WaxedLightningRod,
    WaxedExposedLightningRod,
    WaxedWeatheredLightningRod,
    WaxedOxidizedLightningRod,
    DripstoneBlock,
    PointedDripstone,
    SulfurSpike,
    CaveVines,
    CaveVinesPlant,
    SporeBlossom,
    Azalea,
    FloweringAzalea,
    MossCarpet,
    PinkPetals,
    Wildflowers,
    LeafLitter,
    MossBlock,
    BigDripleaf,
    BigDripleafStem,
    SmallDripleaf,
    HangingRoots,
    RootedDirt,
    Mud,
    Deepslate,
    CobbledDeepslate,
    CobbledDeepslateStairs,
    CobbledDeepslateSlab,
    CobbledDeepslateWall,
    PolishedDeepslate,
    PolishedDeepslateStairs,
    PolishedDeepslateSlab,
    PolishedDeepslateWall,
    DeepslateTiles,
    DeepslateTileStairs,
    DeepslateTileSlab,
    DeepslateTileWall,
    DeepslateBricks,
    DeepslateBrickStairs,
    DeepslateBrickSlab,
    DeepslateBrickWall,
    ChiseledDeepslate,
    CrackedDeepslateBricks,
    CrackedDeepslateTiles,
    InfestedDeepslate,
    SmoothBasalt,
    RawIronBlock,
    RawCopperBlock,
    RawGoldBlock,
    PottedAzaleaBush,
    PottedFloweringAzaleaBush,
    OchreFroglight,
    VerdantFroglight,
    PearlescentFroglight,
    Frogspawn,
    ReinforcedDeepslate,
    DecoratedPot,
    Crafter,
    TrialSpawner,
    Vault,
    HeavyCore,
    PaleMossBlock,
    PaleMossCarpet,
    PaleHangingMoss,
    OpenEyeblossom,
    ClosedEyeblossom,
    PottedOpenEyeblossom,
    PottedClosedEyeblossom,
    FireflyBush,
}
pub static AIR: BlockData = BlockData::new(0, 0, &[], None);
pub static STONE: BlockData = BlockData::new(1, 1, &[], None);
pub static GRANITE: BlockData = BlockData::new(2, 2, &[], None);
pub static POLISHED_GRANITE: BlockData = BlockData::new(3, 3, &[], None);
pub static DIORITE: BlockData = BlockData::new(4, 4, &[], None);
pub static POLISHED_DIORITE: BlockData = BlockData::new(5, 5, &[], None);
pub static ANDESITE: BlockData = BlockData::new(6, 6, &[], None);
pub static POLISHED_ANDESITE: BlockData = BlockData::new(7, 7, &[], None);
pub static GRASS_BLOCK: BlockData = BlockData::new(9, 8, &[&Properties::SNOWY], None);
pub static DIRT: BlockData = BlockData::new(10, 10, &[], None);
pub static COARSE_DIRT: BlockData = BlockData::new(11, 11, &[], None);
pub static PODZOL: BlockData = BlockData::new(13, 12, &[&Properties::SNOWY], None);
pub static COBBLESTONE: BlockData = BlockData::new(14, 14, &[], None);
pub static OAK_PLANKS: BlockData = BlockData::new(15, 15, &[], None);
pub static SPRUCE_PLANKS: BlockData = BlockData::new(16, 16, &[], None);
pub static BIRCH_PLANKS: BlockData = BlockData::new(17, 17, &[], None);
pub static JUNGLE_PLANKS: BlockData = BlockData::new(18, 18, &[], None);
pub static ACACIA_PLANKS: BlockData = BlockData::new(19, 19, &[], None);
pub static CHERRY_PLANKS: BlockData = BlockData::new(20, 20, &[], None);
pub static DARK_OAK_PLANKS: BlockData = BlockData::new(21, 21, &[], None);
pub static PALE_OAK_WOOD: BlockData = BlockData::new(23, 22, &[&Properties::AXIS], None);
pub static PALE_OAK_PLANKS: BlockData = BlockData::new(25, 25, &[], None);
pub static MANGROVE_PLANKS: BlockData = BlockData::new(26, 26, &[], None);
pub static BAMBOO_PLANKS: BlockData = BlockData::new(27, 27, &[], None);
pub static BAMBOO_MOSAIC: BlockData = BlockData::new(28, 28, &[], None);
pub static OAK_SAPLING: BlockData = BlockData::new(29, 29, &[&Properties::STAGE], None);
pub static SPRUCE_SAPLING: BlockData = BlockData::new(31, 31, &[&Properties::STAGE], None);
pub static BIRCH_SAPLING: BlockData = BlockData::new(33, 33, &[&Properties::STAGE], None);
pub static JUNGLE_SAPLING: BlockData = BlockData::new(35, 35, &[&Properties::STAGE], None);
pub static ACACIA_SAPLING: BlockData = BlockData::new(37, 37, &[&Properties::STAGE], None);
pub static CHERRY_SAPLING: BlockData = BlockData::new(39, 39, &[&Properties::STAGE], None);
pub static DARK_OAK_SAPLING: BlockData = BlockData::new(41, 41, &[&Properties::STAGE], None);
pub static PALE_OAK_SAPLING: BlockData = BlockData::new(43, 43, &[&Properties::STAGE], None);
pub static MANGROVE_PROPAGULE: BlockData = BlockData::new(
    50,
    45,
    &[
        &Properties::AGE_4,
        &Properties::HANGING,
        &Properties::STAGE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BEDROCK: BlockData = BlockData::new(85, 85, &[], None);
pub static WATER: BlockData = BlockData::new(86, 86, &[&Properties::LEVEL], None);
pub static LAVA: BlockData = BlockData::new(102, 102, &[&Properties::LEVEL], None);
pub static SAND: BlockData = BlockData::new(118, 118, &[], None);
pub static SUSPICIOUS_SAND: BlockData = BlockData::new(
    119,
    119,
    &[&Properties::DUSTED],
    Some(BlockEntityType::BrushableBlock),
);
pub static RED_SAND: BlockData = BlockData::new(123, 123, &[], None);
pub static GRAVEL: BlockData = BlockData::new(124, 124, &[], None);
pub static SUSPICIOUS_GRAVEL: BlockData = BlockData::new(
    125,
    125,
    &[&Properties::DUSTED],
    Some(BlockEntityType::BrushableBlock),
);
pub static GOLD_ORE: BlockData = BlockData::new(129, 129, &[], None);
pub static DEEPSLATE_GOLD_ORE: BlockData = BlockData::new(130, 130, &[], None);
pub static IRON_ORE: BlockData = BlockData::new(131, 131, &[], None);
pub static DEEPSLATE_IRON_ORE: BlockData = BlockData::new(132, 132, &[], None);
pub static COAL_ORE: BlockData = BlockData::new(133, 133, &[], None);
pub static DEEPSLATE_COAL_ORE: BlockData = BlockData::new(134, 134, &[], None);
pub static NETHER_GOLD_ORE: BlockData = BlockData::new(135, 135, &[], None);
pub static OAK_LOG: BlockData = BlockData::new(137, 136, &[&Properties::AXIS], None);
pub static SPRUCE_LOG: BlockData = BlockData::new(140, 139, &[&Properties::AXIS], None);
pub static BIRCH_LOG: BlockData = BlockData::new(143, 142, &[&Properties::AXIS], None);
pub static JUNGLE_LOG: BlockData = BlockData::new(146, 145, &[&Properties::AXIS], None);
pub static ACACIA_LOG: BlockData = BlockData::new(149, 148, &[&Properties::AXIS], None);
pub static CHERRY_LOG: BlockData = BlockData::new(152, 151, &[&Properties::AXIS], None);
pub static DARK_OAK_LOG: BlockData = BlockData::new(155, 154, &[&Properties::AXIS], None);
pub static PALE_OAK_LOG: BlockData = BlockData::new(158, 157, &[&Properties::AXIS], None);
pub static MANGROVE_LOG: BlockData = BlockData::new(161, 160, &[&Properties::AXIS], None);
pub static MANGROVE_ROOTS: BlockData = BlockData::new(164, 163, &[&Properties::WATERLOGGED], None);
pub static MUDDY_MANGROVE_ROOTS: BlockData = BlockData::new(166, 165, &[&Properties::AXIS], None);
pub static BAMBOO_BLOCK: BlockData = BlockData::new(169, 168, &[&Properties::AXIS], None);
pub static STRIPPED_SPRUCE_LOG: BlockData = BlockData::new(172, 171, &[&Properties::AXIS], None);
pub static STRIPPED_BIRCH_LOG: BlockData = BlockData::new(175, 174, &[&Properties::AXIS], None);
pub static STRIPPED_JUNGLE_LOG: BlockData = BlockData::new(178, 177, &[&Properties::AXIS], None);
pub static STRIPPED_ACACIA_LOG: BlockData = BlockData::new(181, 180, &[&Properties::AXIS], None);
pub static STRIPPED_CHERRY_LOG: BlockData = BlockData::new(184, 183, &[&Properties::AXIS], None);
pub static STRIPPED_DARK_OAK_LOG: BlockData = BlockData::new(187, 186, &[&Properties::AXIS], None);
pub static STRIPPED_PALE_OAK_LOG: BlockData = BlockData::new(190, 189, &[&Properties::AXIS], None);
pub static STRIPPED_OAK_LOG: BlockData = BlockData::new(193, 192, &[&Properties::AXIS], None);
pub static STRIPPED_MANGROVE_LOG: BlockData = BlockData::new(196, 195, &[&Properties::AXIS], None);
pub static STRIPPED_BAMBOO_BLOCK: BlockData = BlockData::new(199, 198, &[&Properties::AXIS], None);
pub static OAK_WOOD: BlockData = BlockData::new(202, 201, &[&Properties::AXIS], None);
pub static SPRUCE_WOOD: BlockData = BlockData::new(205, 204, &[&Properties::AXIS], None);
pub static BIRCH_WOOD: BlockData = BlockData::new(208, 207, &[&Properties::AXIS], None);
pub static JUNGLE_WOOD: BlockData = BlockData::new(211, 210, &[&Properties::AXIS], None);
pub static ACACIA_WOOD: BlockData = BlockData::new(214, 213, &[&Properties::AXIS], None);
pub static CHERRY_WOOD: BlockData = BlockData::new(217, 216, &[&Properties::AXIS], None);
pub static DARK_OAK_WOOD: BlockData = BlockData::new(220, 219, &[&Properties::AXIS], None);
pub static MANGROVE_WOOD: BlockData = BlockData::new(223, 222, &[&Properties::AXIS], None);
pub static STRIPPED_OAK_WOOD: BlockData = BlockData::new(226, 225, &[&Properties::AXIS], None);
pub static STRIPPED_SPRUCE_WOOD: BlockData = BlockData::new(229, 228, &[&Properties::AXIS], None);
pub static STRIPPED_BIRCH_WOOD: BlockData = BlockData::new(232, 231, &[&Properties::AXIS], None);
pub static STRIPPED_JUNGLE_WOOD: BlockData = BlockData::new(235, 234, &[&Properties::AXIS], None);
pub static STRIPPED_ACACIA_WOOD: BlockData = BlockData::new(238, 237, &[&Properties::AXIS], None);
pub static STRIPPED_CHERRY_WOOD: BlockData = BlockData::new(241, 240, &[&Properties::AXIS], None);
pub static STRIPPED_DARK_OAK_WOOD: BlockData = BlockData::new(244, 243, &[&Properties::AXIS], None);
pub static STRIPPED_PALE_OAK_WOOD: BlockData = BlockData::new(247, 246, &[&Properties::AXIS], None);
pub static STRIPPED_MANGROVE_WOOD: BlockData = BlockData::new(250, 249, &[&Properties::AXIS], None);
pub static OAK_LEAVES: BlockData = BlockData::new(
    279,
    252,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SPRUCE_LEAVES: BlockData = BlockData::new(
    307,
    280,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BIRCH_LEAVES: BlockData = BlockData::new(
    335,
    308,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static JUNGLE_LEAVES: BlockData = BlockData::new(
    363,
    336,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static ACACIA_LEAVES: BlockData = BlockData::new(
    391,
    364,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CHERRY_LEAVES: BlockData = BlockData::new(
    419,
    392,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DARK_OAK_LEAVES: BlockData = BlockData::new(
    447,
    420,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PALE_OAK_LEAVES: BlockData = BlockData::new(
    475,
    448,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MANGROVE_LEAVES: BlockData = BlockData::new(
    503,
    476,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static AZALEA_LEAVES: BlockData = BlockData::new(
    531,
    504,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static FLOWERING_AZALEA_LEAVES: BlockData = BlockData::new(
    559,
    532,
    &[
        &Properties::DISTANCE,
        &Properties::PERSISTENT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SPONGE: BlockData = BlockData::new(560, 560, &[], None);
pub static WET_SPONGE: BlockData = BlockData::new(561, 561, &[], None);
pub static GLASS: BlockData = BlockData::new(562, 562, &[], None);
pub static LAPIS_ORE: BlockData = BlockData::new(563, 563, &[], None);
pub static DEEPSLATE_LAPIS_ORE: BlockData = BlockData::new(564, 564, &[], None);
pub static LAPIS_BLOCK: BlockData = BlockData::new(565, 565, &[], None);
pub static DISPENSER: BlockData = BlockData::new(
    567,
    566,
    &[&Properties::FACING, &Properties::TRIGGERED],
    Some(BlockEntityType::Dispenser),
);
pub static SANDSTONE: BlockData = BlockData::new(578, 578, &[], None);
pub static CHISELED_SANDSTONE: BlockData = BlockData::new(579, 579, &[], None);
pub static CUT_SANDSTONE: BlockData = BlockData::new(580, 580, &[], None);
pub static NOTE_BLOCK: BlockData = BlockData::new(
    582,
    581,
    &[
        &Properties::NOTEBLOCK_INSTRUMENT,
        &Properties::NOTE,
        &Properties::POWERED,
    ],
    None,
);
pub static WHITE_BED: BlockData = BlockData::new(
    1934,
    1931,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static ORANGE_BED: BlockData = BlockData::new(
    1950,
    1947,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static MAGENTA_BED: BlockData = BlockData::new(
    1966,
    1963,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static LIGHT_BLUE_BED: BlockData = BlockData::new(
    1982,
    1979,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static YELLOW_BED: BlockData = BlockData::new(
    1998,
    1995,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static LIME_BED: BlockData = BlockData::new(
    2014,
    2011,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static PINK_BED: BlockData = BlockData::new(
    2030,
    2027,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static GRAY_BED: BlockData = BlockData::new(
    2046,
    2043,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static LIGHT_GRAY_BED: BlockData = BlockData::new(
    2062,
    2059,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static CYAN_BED: BlockData = BlockData::new(
    2078,
    2075,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static PURPLE_BED: BlockData = BlockData::new(
    2094,
    2091,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static BLUE_BED: BlockData = BlockData::new(
    2110,
    2107,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static BROWN_BED: BlockData = BlockData::new(
    2126,
    2123,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static GREEN_BED: BlockData = BlockData::new(
    2142,
    2139,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static RED_BED: BlockData = BlockData::new(
    2158,
    2155,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static BLACK_BED: BlockData = BlockData::new(
    2174,
    2171,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OCCUPIED,
        &Properties::BED_PART,
    ],
    None,
);
pub static POWERED_RAIL: BlockData = BlockData::new(
    2200,
    2187,
    &[
        &Properties::POWERED,
        &Properties::RAIL_SHAPE_STRAIGHT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DETECTOR_RAIL: BlockData = BlockData::new(
    2224,
    2211,
    &[
        &Properties::POWERED,
        &Properties::RAIL_SHAPE_STRAIGHT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static STICKY_PISTON: BlockData = BlockData::new(
    2241,
    2235,
    &[&Properties::EXTENDED, &Properties::FACING],
    None,
);
pub static COBWEB: BlockData = BlockData::new(2247, 2247, &[], None);
pub static SHORT_GRASS: BlockData = BlockData::new(2248, 2248, &[], None);
pub static FERN: BlockData = BlockData::new(2249, 2249, &[], None);
pub static DEAD_BUSH: BlockData = BlockData::new(2250, 2250, &[], None);
pub static BUSH: BlockData = BlockData::new(2251, 2251, &[], None);
pub static SHORT_DRY_GRASS: BlockData = BlockData::new(2252, 2252, &[], None);
pub static TALL_DRY_GRASS: BlockData = BlockData::new(2253, 2253, &[], None);
pub static SEAGRASS: BlockData = BlockData::new(2254, 2254, &[], None);
pub static TALL_SEAGRASS: BlockData =
    BlockData::new(2256, 2255, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static PISTON: BlockData = BlockData::new(
    2263,
    2257,
    &[&Properties::EXTENDED, &Properties::FACING],
    None,
);
pub static PISTON_HEAD: BlockData = BlockData::new(
    2271,
    2269,
    &[
        &Properties::FACING,
        &Properties::SHORT,
        &Properties::PISTON_TYPE,
    ],
    None,
);
pub static WHITE_WOOL: BlockData = BlockData::new(2293, 2293, &[], None);
pub static ORANGE_WOOL: BlockData = BlockData::new(2294, 2294, &[], None);
pub static MAGENTA_WOOL: BlockData = BlockData::new(2295, 2295, &[], None);
pub static LIGHT_BLUE_WOOL: BlockData = BlockData::new(2296, 2296, &[], None);
pub static YELLOW_WOOL: BlockData = BlockData::new(2297, 2297, &[], None);
pub static LIME_WOOL: BlockData = BlockData::new(2298, 2298, &[], None);
pub static PINK_WOOL: BlockData = BlockData::new(2299, 2299, &[], None);
pub static GRAY_WOOL: BlockData = BlockData::new(2300, 2300, &[], None);
pub static LIGHT_GRAY_WOOL: BlockData = BlockData::new(2301, 2301, &[], None);
pub static CYAN_WOOL: BlockData = BlockData::new(2302, 2302, &[], None);
pub static PURPLE_WOOL: BlockData = BlockData::new(2303, 2303, &[], None);
pub static BLUE_WOOL: BlockData = BlockData::new(2304, 2304, &[], None);
pub static BROWN_WOOL: BlockData = BlockData::new(2305, 2305, &[], None);
pub static GREEN_WOOL: BlockData = BlockData::new(2306, 2306, &[], None);
pub static RED_WOOL: BlockData = BlockData::new(2307, 2307, &[], None);
pub static BLACK_WOOL: BlockData = BlockData::new(2308, 2308, &[], None);
pub static MOVING_PISTON: BlockData = BlockData::new(
    2309,
    2309,
    &[&Properties::FACING, &Properties::PISTON_TYPE],
    Some(BlockEntityType::Piston),
);
pub static DANDELION: BlockData = BlockData::new(2321, 2321, &[], None);
pub static GOLDEN_DANDELION: BlockData = BlockData::new(2322, 2322, &[], None);
pub static TORCHFLOWER: BlockData = BlockData::new(2323, 2323, &[], None);
pub static POPPY: BlockData = BlockData::new(2324, 2324, &[], None);
pub static BLUE_ORCHID: BlockData = BlockData::new(2325, 2325, &[], None);
pub static ALLIUM: BlockData = BlockData::new(2326, 2326, &[], None);
pub static AZURE_BLUET: BlockData = BlockData::new(2327, 2327, &[], None);
pub static RED_TULIP: BlockData = BlockData::new(2328, 2328, &[], None);
pub static ORANGE_TULIP: BlockData = BlockData::new(2329, 2329, &[], None);
pub static WHITE_TULIP: BlockData = BlockData::new(2330, 2330, &[], None);
pub static PINK_TULIP: BlockData = BlockData::new(2331, 2331, &[], None);
pub static OXEYE_DAISY: BlockData = BlockData::new(2332, 2332, &[], None);
pub static CORNFLOWER: BlockData = BlockData::new(2333, 2333, &[], None);
pub static WITHER_ROSE: BlockData = BlockData::new(2334, 2334, &[], None);
pub static LILY_OF_THE_VALLEY: BlockData = BlockData::new(2335, 2335, &[], None);
pub static BROWN_MUSHROOM: BlockData = BlockData::new(2336, 2336, &[], None);
pub static RED_MUSHROOM: BlockData = BlockData::new(2337, 2337, &[], None);
pub static GOLD_BLOCK: BlockData = BlockData::new(2338, 2338, &[], None);
pub static IRON_BLOCK: BlockData = BlockData::new(2339, 2339, &[], None);
pub static BRICKS: BlockData = BlockData::new(2340, 2340, &[], None);
pub static TNT: BlockData = BlockData::new(2342, 2341, &[&Properties::UNSTABLE], None);
pub static BOOKSHELF: BlockData = BlockData::new(2343, 2343, &[], None);
pub static CHISELED_BOOKSHELF: BlockData = BlockData::new(
    2407,
    2344,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::SLOT_0_OCCUPIED,
        &Properties::SLOT_1_OCCUPIED,
        &Properties::SLOT_2_OCCUPIED,
        &Properties::SLOT_3_OCCUPIED,
        &Properties::SLOT_4_OCCUPIED,
        &Properties::SLOT_5_OCCUPIED,
    ],
    Some(BlockEntityType::ChiseledBookshelf),
);
pub static ACACIA_SHELF: BlockData = BlockData::new(
    2609,
    2600,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static BAMBOO_SHELF: BlockData = BlockData::new(
    2673,
    2664,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static BIRCH_SHELF: BlockData = BlockData::new(
    2737,
    2728,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static CHERRY_SHELF: BlockData = BlockData::new(
    2801,
    2792,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static CRIMSON_SHELF: BlockData = BlockData::new(
    2865,
    2856,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static DARK_OAK_SHELF: BlockData = BlockData::new(
    2929,
    2920,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static JUNGLE_SHELF: BlockData = BlockData::new(
    2993,
    2984,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static MANGROVE_SHELF: BlockData = BlockData::new(
    3057,
    3048,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static OAK_SHELF: BlockData = BlockData::new(
    3121,
    3112,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static PALE_OAK_SHELF: BlockData = BlockData::new(
    3185,
    3176,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static SPRUCE_SHELF: BlockData = BlockData::new(
    3249,
    3240,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static WARPED_SHELF: BlockData = BlockData::new(
    3313,
    3304,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
        &Properties::SIDE_CHAIN_PART,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Shelf),
);
pub static MOSSY_COBBLESTONE: BlockData = BlockData::new(3368, 3368, &[], None);
pub static OBSIDIAN: BlockData = BlockData::new(3369, 3369, &[], None);
pub static TORCH: BlockData = BlockData::new(3370, 3370, &[], None);
pub static WALL_TORCH: BlockData =
    BlockData::new(3371, 3371, &[&Properties::HORIZONTAL_FACING], None);
pub static FIRE: BlockData = BlockData::new(
    3406,
    3375,
    &[
        &Properties::AGE_15,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static SOUL_FIRE: BlockData = BlockData::new(3887, 3887, &[], None);
pub static SPAWNER: BlockData = BlockData::new(3888, 3888, &[], Some(BlockEntityType::MobSpawner));
pub static CREAKING_HEART: BlockData = BlockData::new(
    3896,
    3889,
    &[
        &Properties::AXIS,
        &Properties::CREAKING_HEART_STATE,
        &Properties::NATURAL,
    ],
    Some(BlockEntityType::CreakingHeart),
);
pub static OAK_STAIRS: BlockData = BlockData::new(
    3918,
    3907,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CHEST: BlockData = BlockData::new(
    3988,
    3987,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static REDSTONE_WIRE: BlockData = BlockData::new(
    5171,
    4011,
    &[
        &Properties::EAST_REDSTONE,
        &Properties::NORTH_REDSTONE,
        &Properties::POWER,
        &Properties::SOUTH_REDSTONE,
        &Properties::WEST_REDSTONE,
    ],
    None,
);
pub static DIAMOND_ORE: BlockData = BlockData::new(5307, 5307, &[], None);
pub static DEEPSLATE_DIAMOND_ORE: BlockData = BlockData::new(5308, 5308, &[], None);
pub static DIAMOND_BLOCK: BlockData = BlockData::new(5309, 5309, &[], None);
pub static CRAFTING_TABLE: BlockData = BlockData::new(5310, 5310, &[], None);
pub static WHEAT: BlockData = BlockData::new(5311, 5311, &[&Properties::AGE_7], None);
pub static FARMLAND: BlockData = BlockData::new(5319, 5319, &[&Properties::MOISTURE], None);
pub static FURNACE: BlockData = BlockData::new(
    5328,
    5327,
    &[&Properties::HORIZONTAL_FACING, &Properties::LIT],
    Some(BlockEntityType::Furnace),
);
pub static OAK_SIGN: BlockData = BlockData::new(
    5352,
    5335,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static SPRUCE_SIGN: BlockData = BlockData::new(
    5384,
    5367,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static BIRCH_SIGN: BlockData = BlockData::new(
    5416,
    5399,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static ACACIA_SIGN: BlockData = BlockData::new(
    5448,
    5431,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static CHERRY_SIGN: BlockData = BlockData::new(
    5480,
    5463,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static JUNGLE_SIGN: BlockData = BlockData::new(
    5512,
    5495,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static DARK_OAK_SIGN: BlockData = BlockData::new(
    5544,
    5527,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static PALE_OAK_SIGN: BlockData = BlockData::new(
    5576,
    5559,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static MANGROVE_SIGN: BlockData = BlockData::new(
    5608,
    5591,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static BAMBOO_SIGN: BlockData = BlockData::new(
    5640,
    5623,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static OAK_DOOR: BlockData = BlockData::new(
    5666,
    5655,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static LADDER: BlockData = BlockData::new(
    5720,
    5719,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static RAIL: BlockData = BlockData::new(
    5728,
    5727,
    &[&Properties::RAIL_SHAPE, &Properties::WATERLOGGED],
    None,
);
pub static COBBLESTONE_STAIRS: BlockData = BlockData::new(
    5758,
    5747,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static OAK_WALL_SIGN: BlockData = BlockData::new(
    5828,
    5827,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static SPRUCE_WALL_SIGN: BlockData = BlockData::new(
    5836,
    5835,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static BIRCH_WALL_SIGN: BlockData = BlockData::new(
    5844,
    5843,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static ACACIA_WALL_SIGN: BlockData = BlockData::new(
    5852,
    5851,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static CHERRY_WALL_SIGN: BlockData = BlockData::new(
    5860,
    5859,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static JUNGLE_WALL_SIGN: BlockData = BlockData::new(
    5868,
    5867,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static DARK_OAK_WALL_SIGN: BlockData = BlockData::new(
    5876,
    5875,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static PALE_OAK_WALL_SIGN: BlockData = BlockData::new(
    5884,
    5883,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static MANGROVE_WALL_SIGN: BlockData = BlockData::new(
    5892,
    5891,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static BAMBOO_WALL_SIGN: BlockData = BlockData::new(
    5900,
    5899,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static OAK_HANGING_SIGN: BlockData = BlockData::new(
    5956,
    5907,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static SPRUCE_HANGING_SIGN: BlockData = BlockData::new(
    6020,
    5971,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static BIRCH_HANGING_SIGN: BlockData = BlockData::new(
    6084,
    6035,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static ACACIA_HANGING_SIGN: BlockData = BlockData::new(
    6148,
    6099,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static CHERRY_HANGING_SIGN: BlockData = BlockData::new(
    6212,
    6163,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static JUNGLE_HANGING_SIGN: BlockData = BlockData::new(
    6276,
    6227,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static DARK_OAK_HANGING_SIGN: BlockData = BlockData::new(
    6340,
    6291,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static PALE_OAK_HANGING_SIGN: BlockData = BlockData::new(
    6404,
    6355,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static CRIMSON_HANGING_SIGN: BlockData = BlockData::new(
    6468,
    6419,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static WARPED_HANGING_SIGN: BlockData = BlockData::new(
    6532,
    6483,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static MANGROVE_HANGING_SIGN: BlockData = BlockData::new(
    6596,
    6547,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static BAMBOO_HANGING_SIGN: BlockData = BlockData::new(
    6660,
    6611,
    &[
        &Properties::ATTACHED,
        &Properties::ROTATION_16,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::HangingSign),
);
pub static OAK_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6676,
    6675,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static SPRUCE_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6684,
    6683,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static BIRCH_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6692,
    6691,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static ACACIA_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6700,
    6699,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static CHERRY_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6708,
    6707,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static JUNGLE_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6716,
    6715,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static DARK_OAK_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6724,
    6723,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static PALE_OAK_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6732,
    6731,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static MANGROVE_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6740,
    6739,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static CRIMSON_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6748,
    6747,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static WARPED_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6756,
    6755,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static BAMBOO_WALL_HANGING_SIGN: BlockData = BlockData::new(
    6764,
    6763,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::HangingSign),
);
pub static LEVER: BlockData = BlockData::new(
    6780,
    6771,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static STONE_PRESSURE_PLATE: BlockData =
    BlockData::new(6796, 6795, &[&Properties::POWERED], None);
pub static IRON_DOOR: BlockData = BlockData::new(
    6808,
    6797,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static OAK_PRESSURE_PLATE: BlockData =
    BlockData::new(6862, 6861, &[&Properties::POWERED], None);
pub static SPRUCE_PRESSURE_PLATE: BlockData =
    BlockData::new(6864, 6863, &[&Properties::POWERED], None);
pub static BIRCH_PRESSURE_PLATE: BlockData =
    BlockData::new(6866, 6865, &[&Properties::POWERED], None);
pub static JUNGLE_PRESSURE_PLATE: BlockData =
    BlockData::new(6868, 6867, &[&Properties::POWERED], None);
pub static ACACIA_PRESSURE_PLATE: BlockData =
    BlockData::new(6870, 6869, &[&Properties::POWERED], None);
pub static CHERRY_PRESSURE_PLATE: BlockData =
    BlockData::new(6872, 6871, &[&Properties::POWERED], None);
pub static DARK_OAK_PRESSURE_PLATE: BlockData =
    BlockData::new(6874, 6873, &[&Properties::POWERED], None);
pub static PALE_OAK_PRESSURE_PLATE: BlockData =
    BlockData::new(6876, 6875, &[&Properties::POWERED], None);
pub static MANGROVE_PRESSURE_PLATE: BlockData =
    BlockData::new(6878, 6877, &[&Properties::POWERED], None);
pub static BAMBOO_PRESSURE_PLATE: BlockData =
    BlockData::new(6880, 6879, &[&Properties::POWERED], None);
pub static REDSTONE_ORE: BlockData = BlockData::new(6882, 6881, &[&Properties::LIT], None);
pub static DEEPSLATE_REDSTONE_ORE: BlockData =
    BlockData::new(6884, 6883, &[&Properties::LIT], None);
pub static REDSTONE_TORCH: BlockData = BlockData::new(6885, 6885, &[&Properties::LIT], None);
pub static REDSTONE_WALL_TORCH: BlockData = BlockData::new(
    6887,
    6887,
    &[&Properties::HORIZONTAL_FACING, &Properties::LIT],
    None,
);
pub static STONE_BUTTON: BlockData = BlockData::new(
    6904,
    6895,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static SNOW: BlockData = BlockData::new(6919, 6919, &[&Properties::LAYERS], None);
pub static ICE: BlockData = BlockData::new(6927, 6927, &[], None);
pub static SNOW_BLOCK: BlockData = BlockData::new(6928, 6928, &[], None);
pub static CACTUS: BlockData = BlockData::new(6929, 6929, &[&Properties::AGE_15], None);
pub static CACTUS_FLOWER: BlockData = BlockData::new(6945, 6945, &[], None);
pub static CLAY: BlockData = BlockData::new(6946, 6946, &[], None);
pub static SUGAR_CANE: BlockData = BlockData::new(6947, 6947, &[&Properties::AGE_15], None);
pub static JUKEBOX: BlockData = BlockData::new(
    6964,
    6963,
    &[&Properties::HAS_RECORD],
    Some(BlockEntityType::Jukebox),
);
pub static OAK_FENCE: BlockData = BlockData::new(
    6996,
    6965,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static NETHERRACK: BlockData = BlockData::new(6997, 6997, &[], None);
pub static SOUL_SAND: BlockData = BlockData::new(6998, 6998, &[], None);
pub static SOUL_SOIL: BlockData = BlockData::new(6999, 6999, &[], None);
pub static BASALT: BlockData = BlockData::new(7001, 7000, &[&Properties::AXIS], None);
pub static POLISHED_BASALT: BlockData = BlockData::new(7004, 7003, &[&Properties::AXIS], None);
pub static SOUL_TORCH: BlockData = BlockData::new(7006, 7006, &[], None);
pub static SOUL_WALL_TORCH: BlockData =
    BlockData::new(7007, 7007, &[&Properties::HORIZONTAL_FACING], None);
pub static COPPER_TORCH: BlockData = BlockData::new(7011, 7011, &[], None);
pub static COPPER_WALL_TORCH: BlockData =
    BlockData::new(7012, 7012, &[&Properties::HORIZONTAL_FACING], None);
pub static GLOWSTONE: BlockData = BlockData::new(7016, 7016, &[], None);
pub static NETHER_PORTAL: BlockData =
    BlockData::new(7017, 7017, &[&Properties::HORIZONTAL_AXIS], None);
pub static CARVED_PUMPKIN: BlockData =
    BlockData::new(7019, 7019, &[&Properties::HORIZONTAL_FACING], None);
pub static JACK_O_LANTERN: BlockData =
    BlockData::new(7023, 7023, &[&Properties::HORIZONTAL_FACING], None);
pub static CAKE: BlockData = BlockData::new(7027, 7027, &[&Properties::BITES], None);
pub static REPEATER: BlockData = BlockData::new(
    7037,
    7034,
    &[
        &Properties::DELAY,
        &Properties::HORIZONTAL_FACING,
        &Properties::LOCKED,
        &Properties::POWERED,
    ],
    None,
);
pub static WHITE_STAINED_GLASS: BlockData = BlockData::new(7098, 7098, &[], None);
pub static ORANGE_STAINED_GLASS: BlockData = BlockData::new(7099, 7099, &[], None);
pub static MAGENTA_STAINED_GLASS: BlockData = BlockData::new(7100, 7100, &[], None);
pub static LIGHT_BLUE_STAINED_GLASS: BlockData = BlockData::new(7101, 7101, &[], None);
pub static YELLOW_STAINED_GLASS: BlockData = BlockData::new(7102, 7102, &[], None);
pub static LIME_STAINED_GLASS: BlockData = BlockData::new(7103, 7103, &[], None);
pub static PINK_STAINED_GLASS: BlockData = BlockData::new(7104, 7104, &[], None);
pub static GRAY_STAINED_GLASS: BlockData = BlockData::new(7105, 7105, &[], None);
pub static LIGHT_GRAY_STAINED_GLASS: BlockData = BlockData::new(7106, 7106, &[], None);
pub static CYAN_STAINED_GLASS: BlockData = BlockData::new(7107, 7107, &[], None);
pub static PURPLE_STAINED_GLASS: BlockData = BlockData::new(7108, 7108, &[], None);
pub static BLUE_STAINED_GLASS: BlockData = BlockData::new(7109, 7109, &[], None);
pub static BROWN_STAINED_GLASS: BlockData = BlockData::new(7110, 7110, &[], None);
pub static GREEN_STAINED_GLASS: BlockData = BlockData::new(7111, 7111, &[], None);
pub static RED_STAINED_GLASS: BlockData = BlockData::new(7112, 7112, &[], None);
pub static BLACK_STAINED_GLASS: BlockData = BlockData::new(7113, 7113, &[], None);
pub static OAK_TRAPDOOR: BlockData = BlockData::new(
    7129,
    7114,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SPRUCE_TRAPDOOR: BlockData = BlockData::new(
    7193,
    7178,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BIRCH_TRAPDOOR: BlockData = BlockData::new(
    7257,
    7242,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static JUNGLE_TRAPDOOR: BlockData = BlockData::new(
    7321,
    7306,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static ACACIA_TRAPDOOR: BlockData = BlockData::new(
    7385,
    7370,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CHERRY_TRAPDOOR: BlockData = BlockData::new(
    7449,
    7434,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DARK_OAK_TRAPDOOR: BlockData = BlockData::new(
    7513,
    7498,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PALE_OAK_TRAPDOOR: BlockData = BlockData::new(
    7577,
    7562,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MANGROVE_TRAPDOOR: BlockData = BlockData::new(
    7641,
    7626,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BAMBOO_TRAPDOOR: BlockData = BlockData::new(
    7705,
    7690,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static STONE_BRICKS: BlockData = BlockData::new(7754, 7754, &[], None);
pub static MOSSY_STONE_BRICKS: BlockData = BlockData::new(7755, 7755, &[], None);
pub static CRACKED_STONE_BRICKS: BlockData = BlockData::new(7756, 7756, &[], None);
pub static CHISELED_STONE_BRICKS: BlockData = BlockData::new(7757, 7757, &[], None);
pub static PACKED_MUD: BlockData = BlockData::new(7758, 7758, &[], None);
pub static MUD_BRICKS: BlockData = BlockData::new(7759, 7759, &[], None);
pub static INFESTED_STONE: BlockData = BlockData::new(7760, 7760, &[], None);
pub static INFESTED_COBBLESTONE: BlockData = BlockData::new(7761, 7761, &[], None);
pub static INFESTED_STONE_BRICKS: BlockData = BlockData::new(7762, 7762, &[], None);
pub static INFESTED_MOSSY_STONE_BRICKS: BlockData = BlockData::new(7763, 7763, &[], None);
pub static INFESTED_CRACKED_STONE_BRICKS: BlockData = BlockData::new(7764, 7764, &[], None);
pub static INFESTED_CHISELED_STONE_BRICKS: BlockData = BlockData::new(7765, 7765, &[], None);
pub static BROWN_MUSHROOM_BLOCK: BlockData = BlockData::new(
    7766,
    7766,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static RED_MUSHROOM_BLOCK: BlockData = BlockData::new(
    7830,
    7830,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static MUSHROOM_STEM: BlockData = BlockData::new(
    7894,
    7894,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static IRON_BARS: BlockData = BlockData::new(
    7989,
    7958,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static COPPER_BARS: BlockData = BlockData::new(
    8021,
    7990,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static EXPOSED_COPPER_BARS: BlockData = BlockData::new(
    8053,
    8022,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WEATHERED_COPPER_BARS: BlockData = BlockData::new(
    8085,
    8054,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static OXIDIZED_COPPER_BARS: BlockData = BlockData::new(
    8117,
    8086,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WAXED_COPPER_BARS: BlockData = BlockData::new(
    8149,
    8118,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WAXED_EXPOSED_COPPER_BARS: BlockData = BlockData::new(
    8181,
    8150,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WAXED_WEATHERED_COPPER_BARS: BlockData = BlockData::new(
    8213,
    8182,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WAXED_OXIDIZED_COPPER_BARS: BlockData = BlockData::new(
    8245,
    8214,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static IRON_CHAIN: BlockData = BlockData::new(
    8249,
    8246,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static COPPER_CHAIN: BlockData = BlockData::new(
    8255,
    8252,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static EXPOSED_COPPER_CHAIN: BlockData = BlockData::new(
    8261,
    8258,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static WEATHERED_COPPER_CHAIN: BlockData = BlockData::new(
    8267,
    8264,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static OXIDIZED_COPPER_CHAIN: BlockData = BlockData::new(
    8273,
    8270,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_COPPER_CHAIN: BlockData = BlockData::new(
    8279,
    8276,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_EXPOSED_COPPER_CHAIN: BlockData = BlockData::new(
    8285,
    8282,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_WEATHERED_COPPER_CHAIN: BlockData = BlockData::new(
    8291,
    8288,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_OXIDIZED_COPPER_CHAIN: BlockData = BlockData::new(
    8297,
    8294,
    &[&Properties::AXIS, &Properties::WATERLOGGED],
    None,
);
pub static GLASS_PANE: BlockData = BlockData::new(
    8331,
    8300,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static PUMPKIN: BlockData = BlockData::new(8332, 8332, &[], None);
pub static MELON: BlockData = BlockData::new(8333, 8333, &[], None);
pub static ATTACHED_PUMPKIN_STEM: BlockData =
    BlockData::new(8334, 8334, &[&Properties::HORIZONTAL_FACING], None);
pub static ATTACHED_MELON_STEM: BlockData =
    BlockData::new(8338, 8338, &[&Properties::HORIZONTAL_FACING], None);
pub static PUMPKIN_STEM: BlockData = BlockData::new(8342, 8342, &[&Properties::AGE_7], None);
pub static MELON_STEM: BlockData = BlockData::new(8350, 8350, &[&Properties::AGE_7], None);
pub static VINE: BlockData = BlockData::new(
    8389,
    8358,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static GLOW_LICHEN: BlockData = BlockData::new(
    8517,
    8390,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static RESIN_CLUMP: BlockData = BlockData::new(
    8645,
    8518,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static OAK_FENCE_GATE: BlockData = BlockData::new(
    8653,
    8646,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static BRICK_STAIRS: BlockData = BlockData::new(
    8689,
    8678,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static STONE_BRICK_STAIRS: BlockData = BlockData::new(
    8769,
    8758,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MUD_BRICK_STAIRS: BlockData = BlockData::new(
    8849,
    8838,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MYCELIUM: BlockData = BlockData::new(8919, 8918, &[&Properties::SNOWY], None);
pub static LILY_PAD: BlockData = BlockData::new(8920, 8920, &[], None);
pub static RESIN_BLOCK: BlockData = BlockData::new(8921, 8921, &[], None);
pub static RESIN_BRICKS: BlockData = BlockData::new(8922, 8922, &[], None);
pub static RESIN_BRICK_STAIRS: BlockData = BlockData::new(
    8934,
    8923,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static RESIN_BRICK_SLAB: BlockData = BlockData::new(
    9006,
    9003,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static RESIN_BRICK_WALL: BlockData = BlockData::new(
    9012,
    9009,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_RESIN_BRICKS: BlockData = BlockData::new(9333, 9333, &[], None);
pub static NETHER_BRICKS: BlockData = BlockData::new(9334, 9334, &[], None);
pub static NETHER_BRICK_FENCE: BlockData = BlockData::new(
    9366,
    9335,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static NETHER_BRICK_STAIRS: BlockData = BlockData::new(
    9378,
    9367,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static NETHER_WART: BlockData = BlockData::new(9447, 9447, &[&Properties::AGE_3], None);
pub static ENCHANTING_TABLE: BlockData =
    BlockData::new(9451, 9451, &[], Some(BlockEntityType::EnchantingTable));
pub static BREWING_STAND: BlockData = BlockData::new(
    9459,
    9452,
    &[
        &Properties::HAS_BOTTLE_0,
        &Properties::HAS_BOTTLE_1,
        &Properties::HAS_BOTTLE_2,
    ],
    Some(BlockEntityType::BrewingStand),
);
pub static CAULDRON: BlockData = BlockData::new(9460, 9460, &[], None);
pub static WATER_CAULDRON: BlockData =
    BlockData::new(9461, 9461, &[&Properties::LEVEL_CAULDRON], None);
pub static LAVA_CAULDRON: BlockData = BlockData::new(9464, 9464, &[], None);
pub static POWDER_SNOW_CAULDRON: BlockData =
    BlockData::new(9465, 9465, &[&Properties::LEVEL_CAULDRON], None);
pub static END_PORTAL: BlockData =
    BlockData::new(9468, 9468, &[], Some(BlockEntityType::EndPortal));
pub static END_PORTAL_FRAME: BlockData = BlockData::new(
    9473,
    9469,
    &[&Properties::EYE, &Properties::HORIZONTAL_FACING],
    None,
);
pub static END_STONE: BlockData = BlockData::new(9477, 9477, &[], None);
pub static DRAGON_EGG: BlockData = BlockData::new(9478, 9478, &[], None);
pub static REDSTONE_LAMP: BlockData = BlockData::new(9480, 9479, &[&Properties::LIT], None);
pub static COCOA: BlockData = BlockData::new(
    9481,
    9481,
    &[&Properties::AGE_2, &Properties::HORIZONTAL_FACING],
    None,
);
pub static SANDSTONE_STAIRS: BlockData = BlockData::new(
    9504,
    9493,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static EMERALD_ORE: BlockData = BlockData::new(9573, 9573, &[], None);
pub static DEEPSLATE_EMERALD_ORE: BlockData = BlockData::new(9574, 9574, &[], None);
pub static ENDER_CHEST: BlockData = BlockData::new(
    9576,
    9575,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::EnderChest),
);
pub static TRIPWIRE_HOOK: BlockData = BlockData::new(
    9592,
    9583,
    &[
        &Properties::ATTACHED,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static TRIPWIRE: BlockData = BlockData::new(
    9726,
    9599,
    &[
        &Properties::ATTACHED,
        &Properties::DISARMED,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::POWERED,
        &Properties::SOUTH,
        &Properties::WEST,
    ],
    None,
);
pub static EMERALD_BLOCK: BlockData = BlockData::new(9727, 9727, &[], None);
pub static SPRUCE_STAIRS: BlockData = BlockData::new(
    9739,
    9728,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BIRCH_STAIRS: BlockData = BlockData::new(
    9819,
    9808,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static JUNGLE_STAIRS: BlockData = BlockData::new(
    9899,
    9888,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static COMMAND_BLOCK: BlockData = BlockData::new(
    9974,
    9968,
    &[&Properties::CONDITIONAL, &Properties::FACING],
    Some(BlockEntityType::CommandBlock),
);
pub static BEACON: BlockData = BlockData::new(9980, 9980, &[], Some(BlockEntityType::Beacon));
pub static COBBLESTONE_WALL: BlockData = BlockData::new(
    9984,
    9981,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static MOSSY_COBBLESTONE_WALL: BlockData = BlockData::new(
    10308,
    10305,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static FLOWER_POT: BlockData = BlockData::new(10629, 10629, &[], None);
pub static POTTED_TORCHFLOWER: BlockData = BlockData::new(10630, 10630, &[], None);
pub static POTTED_OAK_SAPLING: BlockData = BlockData::new(10631, 10631, &[], None);
pub static POTTED_SPRUCE_SAPLING: BlockData = BlockData::new(10632, 10632, &[], None);
pub static POTTED_BIRCH_SAPLING: BlockData = BlockData::new(10633, 10633, &[], None);
pub static POTTED_JUNGLE_SAPLING: BlockData = BlockData::new(10634, 10634, &[], None);
pub static POTTED_ACACIA_SAPLING: BlockData = BlockData::new(10635, 10635, &[], None);
pub static POTTED_CHERRY_SAPLING: BlockData = BlockData::new(10636, 10636, &[], None);
pub static POTTED_DARK_OAK_SAPLING: BlockData = BlockData::new(10637, 10637, &[], None);
pub static POTTED_PALE_OAK_SAPLING: BlockData = BlockData::new(10638, 10638, &[], None);
pub static POTTED_MANGROVE_PROPAGULE: BlockData = BlockData::new(10639, 10639, &[], None);
pub static POTTED_FERN: BlockData = BlockData::new(10640, 10640, &[], None);
pub static POTTED_DANDELION: BlockData = BlockData::new(10641, 10641, &[], None);
pub static POTTED_GOLDEN_DANDELION: BlockData = BlockData::new(10642, 10642, &[], None);
pub static POTTED_POPPY: BlockData = BlockData::new(10643, 10643, &[], None);
pub static POTTED_BLUE_ORCHID: BlockData = BlockData::new(10644, 10644, &[], None);
pub static POTTED_ALLIUM: BlockData = BlockData::new(10645, 10645, &[], None);
pub static POTTED_AZURE_BLUET: BlockData = BlockData::new(10646, 10646, &[], None);
pub static POTTED_RED_TULIP: BlockData = BlockData::new(10647, 10647, &[], None);
pub static POTTED_ORANGE_TULIP: BlockData = BlockData::new(10648, 10648, &[], None);
pub static POTTED_WHITE_TULIP: BlockData = BlockData::new(10649, 10649, &[], None);
pub static POTTED_PINK_TULIP: BlockData = BlockData::new(10650, 10650, &[], None);
pub static POTTED_OXEYE_DAISY: BlockData = BlockData::new(10651, 10651, &[], None);
pub static POTTED_CORNFLOWER: BlockData = BlockData::new(10652, 10652, &[], None);
pub static POTTED_LILY_OF_THE_VALLEY: BlockData = BlockData::new(10653, 10653, &[], None);
pub static POTTED_WITHER_ROSE: BlockData = BlockData::new(10654, 10654, &[], None);
pub static POTTED_RED_MUSHROOM: BlockData = BlockData::new(10655, 10655, &[], None);
pub static POTTED_BROWN_MUSHROOM: BlockData = BlockData::new(10656, 10656, &[], None);
pub static POTTED_DEAD_BUSH: BlockData = BlockData::new(10657, 10657, &[], None);
pub static POTTED_CACTUS: BlockData = BlockData::new(10658, 10658, &[], None);
pub static CARROTS: BlockData = BlockData::new(10659, 10659, &[&Properties::AGE_7], None);
pub static POTATOES: BlockData = BlockData::new(10667, 10667, &[&Properties::AGE_7], None);
pub static OAK_BUTTON: BlockData = BlockData::new(
    10684,
    10675,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static SPRUCE_BUTTON: BlockData = BlockData::new(
    10708,
    10699,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static BIRCH_BUTTON: BlockData = BlockData::new(
    10732,
    10723,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static JUNGLE_BUTTON: BlockData = BlockData::new(
    10756,
    10747,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static ACACIA_BUTTON: BlockData = BlockData::new(
    10780,
    10771,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static CHERRY_BUTTON: BlockData = BlockData::new(
    10804,
    10795,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static DARK_OAK_BUTTON: BlockData = BlockData::new(
    10828,
    10819,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static PALE_OAK_BUTTON: BlockData = BlockData::new(
    10852,
    10843,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static MANGROVE_BUTTON: BlockData = BlockData::new(
    10876,
    10867,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static BAMBOO_BUTTON: BlockData = BlockData::new(
    10900,
    10891,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static SKELETON_SKULL: BlockData = BlockData::new(
    10931,
    10915,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static SKELETON_WALL_SKULL: BlockData = BlockData::new(
    10948,
    10947,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static WITHER_SKELETON_SKULL: BlockData = BlockData::new(
    10971,
    10955,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static WITHER_SKELETON_WALL_SKULL: BlockData = BlockData::new(
    10988,
    10987,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static ZOMBIE_HEAD: BlockData = BlockData::new(
    11011,
    10995,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static ZOMBIE_WALL_HEAD: BlockData = BlockData::new(
    11028,
    11027,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static PLAYER_HEAD: BlockData = BlockData::new(
    11051,
    11035,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static PLAYER_WALL_HEAD: BlockData = BlockData::new(
    11068,
    11067,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static CREEPER_HEAD: BlockData = BlockData::new(
    11091,
    11075,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static CREEPER_WALL_HEAD: BlockData = BlockData::new(
    11108,
    11107,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static DRAGON_HEAD: BlockData = BlockData::new(
    11131,
    11115,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static DRAGON_WALL_HEAD: BlockData = BlockData::new(
    11148,
    11147,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static PIGLIN_HEAD: BlockData = BlockData::new(
    11171,
    11155,
    &[&Properties::POWERED, &Properties::ROTATION_16],
    Some(BlockEntityType::Skull),
);
pub static PIGLIN_WALL_HEAD: BlockData = BlockData::new(
    11188,
    11187,
    &[&Properties::HORIZONTAL_FACING, &Properties::POWERED],
    Some(BlockEntityType::Skull),
);
pub static ANVIL: BlockData = BlockData::new(11195, 11195, &[&Properties::HORIZONTAL_FACING], None);
pub static CHIPPED_ANVIL: BlockData =
    BlockData::new(11199, 11199, &[&Properties::HORIZONTAL_FACING], None);
pub static DAMAGED_ANVIL: BlockData =
    BlockData::new(11203, 11203, &[&Properties::HORIZONTAL_FACING], None);
pub static TRAPPED_CHEST: BlockData = BlockData::new(
    11208,
    11207,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::TrappedChest),
);
pub static LIGHT_WEIGHTED_PRESSURE_PLATE: BlockData =
    BlockData::new(11231, 11231, &[&Properties::POWER], None);
pub static HEAVY_WEIGHTED_PRESSURE_PLATE: BlockData =
    BlockData::new(11247, 11247, &[&Properties::POWER], None);
pub static COMPARATOR: BlockData = BlockData::new(
    11264,
    11263,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::MODE_COMPARATOR,
        &Properties::POWERED,
    ],
    Some(BlockEntityType::Comparator),
);
pub static DAYLIGHT_DETECTOR: BlockData = BlockData::new(
    11295,
    11279,
    &[&Properties::INVERTED, &Properties::POWER],
    Some(BlockEntityType::DaylightDetector),
);
pub static REDSTONE_BLOCK: BlockData = BlockData::new(11311, 11311, &[], None);
pub static NETHER_QUARTZ_ORE: BlockData = BlockData::new(11312, 11312, &[], None);
pub static HOPPER: BlockData = BlockData::new(
    11313,
    11313,
    &[&Properties::ENABLED, &Properties::FACING_HOPPER],
    Some(BlockEntityType::Hopper),
);
pub static QUARTZ_BLOCK: BlockData = BlockData::new(11323, 11323, &[], None);
pub static CHISELED_QUARTZ_BLOCK: BlockData = BlockData::new(11324, 11324, &[], None);
pub static QUARTZ_PILLAR: BlockData = BlockData::new(11326, 11325, &[&Properties::AXIS], None);
pub static QUARTZ_STAIRS: BlockData = BlockData::new(
    11339,
    11328,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static ACTIVATOR_RAIL: BlockData = BlockData::new(
    11421,
    11408,
    &[
        &Properties::POWERED,
        &Properties::RAIL_SHAPE_STRAIGHT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DROPPER: BlockData = BlockData::new(
    11433,
    11432,
    &[&Properties::FACING, &Properties::TRIGGERED],
    Some(BlockEntityType::Dropper),
);
pub static WHITE_TERRACOTTA: BlockData = BlockData::new(11444, 11444, &[], None);
pub static ORANGE_TERRACOTTA: BlockData = BlockData::new(11445, 11445, &[], None);
pub static MAGENTA_TERRACOTTA: BlockData = BlockData::new(11446, 11446, &[], None);
pub static LIGHT_BLUE_TERRACOTTA: BlockData = BlockData::new(11447, 11447, &[], None);
pub static YELLOW_TERRACOTTA: BlockData = BlockData::new(11448, 11448, &[], None);
pub static LIME_TERRACOTTA: BlockData = BlockData::new(11449, 11449, &[], None);
pub static PINK_TERRACOTTA: BlockData = BlockData::new(11450, 11450, &[], None);
pub static GRAY_TERRACOTTA: BlockData = BlockData::new(11451, 11451, &[], None);
pub static LIGHT_GRAY_TERRACOTTA: BlockData = BlockData::new(11452, 11452, &[], None);
pub static CYAN_TERRACOTTA: BlockData = BlockData::new(11453, 11453, &[], None);
pub static PURPLE_TERRACOTTA: BlockData = BlockData::new(11454, 11454, &[], None);
pub static BLUE_TERRACOTTA: BlockData = BlockData::new(11455, 11455, &[], None);
pub static BROWN_TERRACOTTA: BlockData = BlockData::new(11456, 11456, &[], None);
pub static GREEN_TERRACOTTA: BlockData = BlockData::new(11457, 11457, &[], None);
pub static RED_TERRACOTTA: BlockData = BlockData::new(11458, 11458, &[], None);
pub static BLACK_TERRACOTTA: BlockData = BlockData::new(11459, 11459, &[], None);
pub static WHITE_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11491,
    11460,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static ORANGE_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11523,
    11492,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static MAGENTA_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11555,
    11524,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static LIGHT_BLUE_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11587,
    11556,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static YELLOW_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11619,
    11588,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static LIME_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11651,
    11620,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static PINK_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11683,
    11652,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static GRAY_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11715,
    11684,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static LIGHT_GRAY_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11747,
    11716,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static CYAN_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11779,
    11748,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static PURPLE_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11811,
    11780,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static BLUE_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11843,
    11812,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static BROWN_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11875,
    11844,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static GREEN_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11907,
    11876,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static RED_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11939,
    11908,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static BLACK_STAINED_GLASS_PANE: BlockData = BlockData::new(
    11971,
    11940,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static ACACIA_STAIRS: BlockData = BlockData::new(
    11983,
    11972,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CHERRY_STAIRS: BlockData = BlockData::new(
    12063,
    12052,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DARK_OAK_STAIRS: BlockData = BlockData::new(
    12143,
    12132,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PALE_OAK_STAIRS: BlockData = BlockData::new(
    12223,
    12212,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MANGROVE_STAIRS: BlockData = BlockData::new(
    12303,
    12292,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BAMBOO_STAIRS: BlockData = BlockData::new(
    12383,
    12372,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BAMBOO_MOSAIC_STAIRS: BlockData = BlockData::new(
    12463,
    12452,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SLIME_BLOCK: BlockData = BlockData::new(12532, 12532, &[], None);
pub static BARRIER: BlockData = BlockData::new(12534, 12533, &[&Properties::WATERLOGGED], None);
pub static LIGHT: BlockData = BlockData::new(
    12566,
    12535,
    &[&Properties::LEVEL, &Properties::WATERLOGGED],
    None,
);
pub static IRON_TRAPDOOR: BlockData = BlockData::new(
    12582,
    12567,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PRISMARINE: BlockData = BlockData::new(12631, 12631, &[], None);
pub static PRISMARINE_BRICKS: BlockData = BlockData::new(12632, 12632, &[], None);
pub static DARK_PRISMARINE: BlockData = BlockData::new(12633, 12633, &[], None);
pub static PRISMARINE_STAIRS: BlockData = BlockData::new(
    12645,
    12634,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PRISMARINE_BRICK_STAIRS: BlockData = BlockData::new(
    12725,
    12714,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DARK_PRISMARINE_STAIRS: BlockData = BlockData::new(
    12805,
    12794,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PRISMARINE_SLAB: BlockData = BlockData::new(
    12877,
    12874,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static PRISMARINE_BRICK_SLAB: BlockData = BlockData::new(
    12883,
    12880,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static DARK_PRISMARINE_SLAB: BlockData = BlockData::new(
    12889,
    12886,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SEA_LANTERN: BlockData = BlockData::new(12892, 12892, &[], None);
pub static HAY_BLOCK: BlockData = BlockData::new(12894, 12893, &[&Properties::AXIS], None);
pub static WHITE_CARPET: BlockData = BlockData::new(12896, 12896, &[], None);
pub static ORANGE_CARPET: BlockData = BlockData::new(12897, 12897, &[], None);
pub static MAGENTA_CARPET: BlockData = BlockData::new(12898, 12898, &[], None);
pub static LIGHT_BLUE_CARPET: BlockData = BlockData::new(12899, 12899, &[], None);
pub static YELLOW_CARPET: BlockData = BlockData::new(12900, 12900, &[], None);
pub static LIME_CARPET: BlockData = BlockData::new(12901, 12901, &[], None);
pub static PINK_CARPET: BlockData = BlockData::new(12902, 12902, &[], None);
pub static GRAY_CARPET: BlockData = BlockData::new(12903, 12903, &[], None);
pub static LIGHT_GRAY_CARPET: BlockData = BlockData::new(12904, 12904, &[], None);
pub static CYAN_CARPET: BlockData = BlockData::new(12905, 12905, &[], None);
pub static PURPLE_CARPET: BlockData = BlockData::new(12906, 12906, &[], None);
pub static BLUE_CARPET: BlockData = BlockData::new(12907, 12907, &[], None);
pub static BROWN_CARPET: BlockData = BlockData::new(12908, 12908, &[], None);
pub static GREEN_CARPET: BlockData = BlockData::new(12909, 12909, &[], None);
pub static RED_CARPET: BlockData = BlockData::new(12910, 12910, &[], None);
pub static BLACK_CARPET: BlockData = BlockData::new(12911, 12911, &[], None);
pub static TERRACOTTA: BlockData = BlockData::new(12912, 12912, &[], None);
pub static COAL_BLOCK: BlockData = BlockData::new(12913, 12913, &[], None);
pub static PACKED_ICE: BlockData = BlockData::new(12914, 12914, &[], None);
pub static SUNFLOWER: BlockData =
    BlockData::new(12916, 12915, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static LILAC: BlockData = BlockData::new(12918, 12917, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static ROSE_BUSH: BlockData =
    BlockData::new(12920, 12919, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static PEONY: BlockData = BlockData::new(12922, 12921, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static TALL_GRASS: BlockData =
    BlockData::new(12924, 12923, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static LARGE_FERN: BlockData =
    BlockData::new(12926, 12925, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static WHITE_BANNER: BlockData = BlockData::new(
    12935,
    12927,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static ORANGE_BANNER: BlockData = BlockData::new(
    12951,
    12943,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static MAGENTA_BANNER: BlockData = BlockData::new(
    12967,
    12959,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static LIGHT_BLUE_BANNER: BlockData = BlockData::new(
    12983,
    12975,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static YELLOW_BANNER: BlockData = BlockData::new(
    12999,
    12991,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static LIME_BANNER: BlockData = BlockData::new(
    13015,
    13007,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static PINK_BANNER: BlockData = BlockData::new(
    13031,
    13023,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static GRAY_BANNER: BlockData = BlockData::new(
    13047,
    13039,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static LIGHT_GRAY_BANNER: BlockData = BlockData::new(
    13063,
    13055,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static CYAN_BANNER: BlockData = BlockData::new(
    13079,
    13071,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static PURPLE_BANNER: BlockData = BlockData::new(
    13095,
    13087,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static BLUE_BANNER: BlockData = BlockData::new(
    13111,
    13103,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static BROWN_BANNER: BlockData = BlockData::new(
    13127,
    13119,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static GREEN_BANNER: BlockData = BlockData::new(
    13143,
    13135,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static RED_BANNER: BlockData = BlockData::new(
    13159,
    13151,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static BLACK_BANNER: BlockData = BlockData::new(
    13175,
    13167,
    &[&Properties::ROTATION_16],
    Some(BlockEntityType::Banner),
);
pub static WHITE_WALL_BANNER: BlockData = BlockData::new(
    13183,
    13183,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static ORANGE_WALL_BANNER: BlockData = BlockData::new(
    13187,
    13187,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static MAGENTA_WALL_BANNER: BlockData = BlockData::new(
    13191,
    13191,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static LIGHT_BLUE_WALL_BANNER: BlockData = BlockData::new(
    13195,
    13195,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static YELLOW_WALL_BANNER: BlockData = BlockData::new(
    13199,
    13199,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static LIME_WALL_BANNER: BlockData = BlockData::new(
    13203,
    13203,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static PINK_WALL_BANNER: BlockData = BlockData::new(
    13207,
    13207,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static GRAY_WALL_BANNER: BlockData = BlockData::new(
    13211,
    13211,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static LIGHT_GRAY_WALL_BANNER: BlockData = BlockData::new(
    13215,
    13215,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static CYAN_WALL_BANNER: BlockData = BlockData::new(
    13219,
    13219,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static PURPLE_WALL_BANNER: BlockData = BlockData::new(
    13223,
    13223,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static BLUE_WALL_BANNER: BlockData = BlockData::new(
    13227,
    13227,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static BROWN_WALL_BANNER: BlockData = BlockData::new(
    13231,
    13231,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static GREEN_WALL_BANNER: BlockData = BlockData::new(
    13235,
    13235,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static RED_WALL_BANNER: BlockData = BlockData::new(
    13239,
    13239,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static BLACK_WALL_BANNER: BlockData = BlockData::new(
    13243,
    13243,
    &[&Properties::HORIZONTAL_FACING],
    Some(BlockEntityType::Banner),
);
pub static RED_SANDSTONE: BlockData = BlockData::new(13247, 13247, &[], None);
pub static CHISELED_RED_SANDSTONE: BlockData = BlockData::new(13248, 13248, &[], None);
pub static CUT_RED_SANDSTONE: BlockData = BlockData::new(13249, 13249, &[], None);
pub static RED_SANDSTONE_STAIRS: BlockData = BlockData::new(
    13261,
    13250,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static OAK_SLAB: BlockData = BlockData::new(
    13333,
    13330,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SPRUCE_SLAB: BlockData = BlockData::new(
    13339,
    13336,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static BIRCH_SLAB: BlockData = BlockData::new(
    13345,
    13342,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static JUNGLE_SLAB: BlockData = BlockData::new(
    13351,
    13348,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static ACACIA_SLAB: BlockData = BlockData::new(
    13357,
    13354,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CHERRY_SLAB: BlockData = BlockData::new(
    13363,
    13360,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static DARK_OAK_SLAB: BlockData = BlockData::new(
    13369,
    13366,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static PALE_OAK_SLAB: BlockData = BlockData::new(
    13375,
    13372,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static MANGROVE_SLAB: BlockData = BlockData::new(
    13381,
    13378,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static BAMBOO_SLAB: BlockData = BlockData::new(
    13387,
    13384,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static BAMBOO_MOSAIC_SLAB: BlockData = BlockData::new(
    13393,
    13390,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static STONE_SLAB: BlockData = BlockData::new(
    13399,
    13396,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SMOOTH_STONE_SLAB: BlockData = BlockData::new(
    13405,
    13402,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SANDSTONE_SLAB: BlockData = BlockData::new(
    13411,
    13408,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CUT_SANDSTONE_SLAB: BlockData = BlockData::new(
    13417,
    13414,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static PETRIFIED_OAK_SLAB: BlockData = BlockData::new(
    13423,
    13420,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static COBBLESTONE_SLAB: BlockData = BlockData::new(
    13429,
    13426,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static BRICK_SLAB: BlockData = BlockData::new(
    13435,
    13432,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static STONE_BRICK_SLAB: BlockData = BlockData::new(
    13441,
    13438,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static MUD_BRICK_SLAB: BlockData = BlockData::new(
    13447,
    13444,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static NETHER_BRICK_SLAB: BlockData = BlockData::new(
    13453,
    13450,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static QUARTZ_SLAB: BlockData = BlockData::new(
    13459,
    13456,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static RED_SANDSTONE_SLAB: BlockData = BlockData::new(
    13465,
    13462,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CUT_RED_SANDSTONE_SLAB: BlockData = BlockData::new(
    13471,
    13468,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static PURPUR_SLAB: BlockData = BlockData::new(
    13477,
    13474,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SMOOTH_STONE: BlockData = BlockData::new(13480, 13480, &[], None);
pub static SMOOTH_SANDSTONE: BlockData = BlockData::new(13481, 13481, &[], None);
pub static SMOOTH_QUARTZ: BlockData = BlockData::new(13482, 13482, &[], None);
pub static SMOOTH_RED_SANDSTONE: BlockData = BlockData::new(13483, 13483, &[], None);
pub static SPRUCE_FENCE_GATE: BlockData = BlockData::new(
    13491,
    13484,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static BIRCH_FENCE_GATE: BlockData = BlockData::new(
    13523,
    13516,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static JUNGLE_FENCE_GATE: BlockData = BlockData::new(
    13555,
    13548,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static ACACIA_FENCE_GATE: BlockData = BlockData::new(
    13587,
    13580,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static CHERRY_FENCE_GATE: BlockData = BlockData::new(
    13619,
    13612,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static DARK_OAK_FENCE_GATE: BlockData = BlockData::new(
    13651,
    13644,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static PALE_OAK_FENCE_GATE: BlockData = BlockData::new(
    13683,
    13676,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static MANGROVE_FENCE_GATE: BlockData = BlockData::new(
    13715,
    13708,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static BAMBOO_FENCE_GATE: BlockData = BlockData::new(
    13747,
    13740,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static SPRUCE_FENCE: BlockData = BlockData::new(
    13803,
    13772,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static BIRCH_FENCE: BlockData = BlockData::new(
    13835,
    13804,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static JUNGLE_FENCE: BlockData = BlockData::new(
    13867,
    13836,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static ACACIA_FENCE: BlockData = BlockData::new(
    13899,
    13868,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static CHERRY_FENCE: BlockData = BlockData::new(
    13931,
    13900,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static DARK_OAK_FENCE: BlockData = BlockData::new(
    13963,
    13932,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static PALE_OAK_FENCE: BlockData = BlockData::new(
    13995,
    13964,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static MANGROVE_FENCE: BlockData = BlockData::new(
    14027,
    13996,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static BAMBOO_FENCE: BlockData = BlockData::new(
    14059,
    14028,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static SPRUCE_DOOR: BlockData = BlockData::new(
    14071,
    14060,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static BIRCH_DOOR: BlockData = BlockData::new(
    14135,
    14124,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static JUNGLE_DOOR: BlockData = BlockData::new(
    14199,
    14188,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static ACACIA_DOOR: BlockData = BlockData::new(
    14263,
    14252,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static CHERRY_DOOR: BlockData = BlockData::new(
    14327,
    14316,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static DARK_OAK_DOOR: BlockData = BlockData::new(
    14391,
    14380,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static PALE_OAK_DOOR: BlockData = BlockData::new(
    14455,
    14444,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static MANGROVE_DOOR: BlockData = BlockData::new(
    14519,
    14508,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static BAMBOO_DOOR: BlockData = BlockData::new(
    14583,
    14572,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static END_ROD: BlockData = BlockData::new(14640, 14636, &[&Properties::FACING], None);
pub static CHORUS_PLANT: BlockData = BlockData::new(
    14705,
    14642,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WEST,
    ],
    None,
);
pub static CHORUS_FLOWER: BlockData = BlockData::new(14706, 14706, &[&Properties::AGE_5], None);
pub static PURPUR_BLOCK: BlockData = BlockData::new(14712, 14712, &[], None);
pub static PURPUR_PILLAR: BlockData = BlockData::new(14714, 14713, &[&Properties::AXIS], None);
pub static PURPUR_STAIRS: BlockData = BlockData::new(
    14727,
    14716,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static END_STONE_BRICKS: BlockData = BlockData::new(14796, 14796, &[], None);
pub static TORCHFLOWER_CROP: BlockData = BlockData::new(14797, 14797, &[&Properties::AGE_1], None);
pub static PITCHER_CROP: BlockData = BlockData::new(
    14800,
    14799,
    &[&Properties::AGE_4, &Properties::DOUBLE_BLOCK_HALF],
    None,
);
pub static PITCHER_PLANT: BlockData =
    BlockData::new(14810, 14809, &[&Properties::DOUBLE_BLOCK_HALF], None);
pub static BEETROOTS: BlockData = BlockData::new(14811, 14811, &[&Properties::AGE_3], None);
pub static DIRT_PATH: BlockData = BlockData::new(14815, 14815, &[], None);
pub static END_GATEWAY: BlockData =
    BlockData::new(14816, 14816, &[], Some(BlockEntityType::EndGateway));
pub static REPEATING_COMMAND_BLOCK: BlockData = BlockData::new(
    14823,
    14817,
    &[&Properties::CONDITIONAL, &Properties::FACING],
    Some(BlockEntityType::CommandBlock),
);
pub static CHAIN_COMMAND_BLOCK: BlockData = BlockData::new(
    14835,
    14829,
    &[&Properties::CONDITIONAL, &Properties::FACING],
    Some(BlockEntityType::CommandBlock),
);
pub static FROSTED_ICE: BlockData = BlockData::new(14841, 14841, &[&Properties::AGE_3], None);
pub static MAGMA_BLOCK: BlockData = BlockData::new(14845, 14845, &[], None);
pub static NETHER_WART_BLOCK: BlockData = BlockData::new(14846, 14846, &[], None);
pub static RED_NETHER_BRICKS: BlockData = BlockData::new(14847, 14847, &[], None);
pub static BONE_BLOCK: BlockData = BlockData::new(14849, 14848, &[&Properties::AXIS], None);
pub static STRUCTURE_VOID: BlockData = BlockData::new(14851, 14851, &[], None);
pub static OBSERVER: BlockData = BlockData::new(
    14857,
    14852,
    &[&Properties::FACING, &Properties::POWERED],
    None,
);
pub static SHULKER_BOX: BlockData = BlockData::new(
    14868,
    14864,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static WHITE_SHULKER_BOX: BlockData = BlockData::new(
    14874,
    14870,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static ORANGE_SHULKER_BOX: BlockData = BlockData::new(
    14880,
    14876,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static MAGENTA_SHULKER_BOX: BlockData = BlockData::new(
    14886,
    14882,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static LIGHT_BLUE_SHULKER_BOX: BlockData = BlockData::new(
    14892,
    14888,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static YELLOW_SHULKER_BOX: BlockData = BlockData::new(
    14898,
    14894,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static LIME_SHULKER_BOX: BlockData = BlockData::new(
    14904,
    14900,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static PINK_SHULKER_BOX: BlockData = BlockData::new(
    14910,
    14906,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static GRAY_SHULKER_BOX: BlockData = BlockData::new(
    14916,
    14912,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static LIGHT_GRAY_SHULKER_BOX: BlockData = BlockData::new(
    14922,
    14918,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static CYAN_SHULKER_BOX: BlockData = BlockData::new(
    14928,
    14924,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static PURPLE_SHULKER_BOX: BlockData = BlockData::new(
    14934,
    14930,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static BLUE_SHULKER_BOX: BlockData = BlockData::new(
    14940,
    14936,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static BROWN_SHULKER_BOX: BlockData = BlockData::new(
    14946,
    14942,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static GREEN_SHULKER_BOX: BlockData = BlockData::new(
    14952,
    14948,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static RED_SHULKER_BOX: BlockData = BlockData::new(
    14958,
    14954,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static BLACK_SHULKER_BOX: BlockData = BlockData::new(
    14964,
    14960,
    &[&Properties::FACING],
    Some(BlockEntityType::ShulkerBox),
);
pub static WHITE_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14966, 14966, &[&Properties::HORIZONTAL_FACING], None);
pub static ORANGE_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14970, 14970, &[&Properties::HORIZONTAL_FACING], None);
pub static MAGENTA_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14974, 14974, &[&Properties::HORIZONTAL_FACING], None);
pub static LIGHT_BLUE_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14978, 14978, &[&Properties::HORIZONTAL_FACING], None);
pub static YELLOW_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14982, 14982, &[&Properties::HORIZONTAL_FACING], None);
pub static LIME_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14986, 14986, &[&Properties::HORIZONTAL_FACING], None);
pub static PINK_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14990, 14990, &[&Properties::HORIZONTAL_FACING], None);
pub static GRAY_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14994, 14994, &[&Properties::HORIZONTAL_FACING], None);
pub static LIGHT_GRAY_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(14998, 14998, &[&Properties::HORIZONTAL_FACING], None);
pub static CYAN_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15002, 15002, &[&Properties::HORIZONTAL_FACING], None);
pub static PURPLE_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15006, 15006, &[&Properties::HORIZONTAL_FACING], None);
pub static BLUE_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15010, 15010, &[&Properties::HORIZONTAL_FACING], None);
pub static BROWN_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15014, 15014, &[&Properties::HORIZONTAL_FACING], None);
pub static GREEN_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15018, 15018, &[&Properties::HORIZONTAL_FACING], None);
pub static RED_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15022, 15022, &[&Properties::HORIZONTAL_FACING], None);
pub static BLACK_GLAZED_TERRACOTTA: BlockData =
    BlockData::new(15026, 15026, &[&Properties::HORIZONTAL_FACING], None);
pub static WHITE_CONCRETE: BlockData = BlockData::new(15030, 15030, &[], None);
pub static ORANGE_CONCRETE: BlockData = BlockData::new(15031, 15031, &[], None);
pub static MAGENTA_CONCRETE: BlockData = BlockData::new(15032, 15032, &[], None);
pub static LIGHT_BLUE_CONCRETE: BlockData = BlockData::new(15033, 15033, &[], None);
pub static YELLOW_CONCRETE: BlockData = BlockData::new(15034, 15034, &[], None);
pub static LIME_CONCRETE: BlockData = BlockData::new(15035, 15035, &[], None);
pub static PINK_CONCRETE: BlockData = BlockData::new(15036, 15036, &[], None);
pub static GRAY_CONCRETE: BlockData = BlockData::new(15037, 15037, &[], None);
pub static LIGHT_GRAY_CONCRETE: BlockData = BlockData::new(15038, 15038, &[], None);
pub static CYAN_CONCRETE: BlockData = BlockData::new(15039, 15039, &[], None);
pub static PURPLE_CONCRETE: BlockData = BlockData::new(15040, 15040, &[], None);
pub static BLUE_CONCRETE: BlockData = BlockData::new(15041, 15041, &[], None);
pub static BROWN_CONCRETE: BlockData = BlockData::new(15042, 15042, &[], None);
pub static GREEN_CONCRETE: BlockData = BlockData::new(15043, 15043, &[], None);
pub static RED_CONCRETE: BlockData = BlockData::new(15044, 15044, &[], None);
pub static BLACK_CONCRETE: BlockData = BlockData::new(15045, 15045, &[], None);
pub static WHITE_CONCRETE_POWDER: BlockData = BlockData::new(15046, 15046, &[], None);
pub static ORANGE_CONCRETE_POWDER: BlockData = BlockData::new(15047, 15047, &[], None);
pub static MAGENTA_CONCRETE_POWDER: BlockData = BlockData::new(15048, 15048, &[], None);
pub static LIGHT_BLUE_CONCRETE_POWDER: BlockData = BlockData::new(15049, 15049, &[], None);
pub static YELLOW_CONCRETE_POWDER: BlockData = BlockData::new(15050, 15050, &[], None);
pub static LIME_CONCRETE_POWDER: BlockData = BlockData::new(15051, 15051, &[], None);
pub static PINK_CONCRETE_POWDER: BlockData = BlockData::new(15052, 15052, &[], None);
pub static GRAY_CONCRETE_POWDER: BlockData = BlockData::new(15053, 15053, &[], None);
pub static LIGHT_GRAY_CONCRETE_POWDER: BlockData = BlockData::new(15054, 15054, &[], None);
pub static CYAN_CONCRETE_POWDER: BlockData = BlockData::new(15055, 15055, &[], None);
pub static PURPLE_CONCRETE_POWDER: BlockData = BlockData::new(15056, 15056, &[], None);
pub static BLUE_CONCRETE_POWDER: BlockData = BlockData::new(15057, 15057, &[], None);
pub static BROWN_CONCRETE_POWDER: BlockData = BlockData::new(15058, 15058, &[], None);
pub static GREEN_CONCRETE_POWDER: BlockData = BlockData::new(15059, 15059, &[], None);
pub static RED_CONCRETE_POWDER: BlockData = BlockData::new(15060, 15060, &[], None);
pub static BLACK_CONCRETE_POWDER: BlockData = BlockData::new(15061, 15061, &[], None);
pub static KELP: BlockData = BlockData::new(15062, 15062, &[&Properties::AGE_25], None);
pub static KELP_PLANT: BlockData = BlockData::new(15088, 15088, &[], None);
pub static DRIED_KELP_BLOCK: BlockData = BlockData::new(15089, 15089, &[], None);
pub static TURTLE_EGG: BlockData =
    BlockData::new(15090, 15090, &[&Properties::EGGS, &Properties::HATCH], None);
pub static SNIFFER_EGG: BlockData = BlockData::new(15102, 15102, &[&Properties::HATCH], None);
pub static DRIED_GHAST: BlockData = BlockData::new(
    15106,
    15105,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DRIED_GHAST_HYDRATION_LEVELS,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DEAD_TUBE_CORAL_BLOCK: BlockData = BlockData::new(15137, 15137, &[], None);
pub static DEAD_BRAIN_CORAL_BLOCK: BlockData = BlockData::new(15138, 15138, &[], None);
pub static DEAD_BUBBLE_CORAL_BLOCK: BlockData = BlockData::new(15139, 15139, &[], None);
pub static DEAD_FIRE_CORAL_BLOCK: BlockData = BlockData::new(15140, 15140, &[], None);
pub static DEAD_HORN_CORAL_BLOCK: BlockData = BlockData::new(15141, 15141, &[], None);
pub static TUBE_CORAL_BLOCK: BlockData = BlockData::new(15142, 15142, &[], None);
pub static BRAIN_CORAL_BLOCK: BlockData = BlockData::new(15143, 15143, &[], None);
pub static BUBBLE_CORAL_BLOCK: BlockData = BlockData::new(15144, 15144, &[], None);
pub static FIRE_CORAL_BLOCK: BlockData = BlockData::new(15145, 15145, &[], None);
pub static HORN_CORAL_BLOCK: BlockData = BlockData::new(15146, 15146, &[], None);
pub static DEAD_TUBE_CORAL: BlockData =
    BlockData::new(15147, 15147, &[&Properties::WATERLOGGED], None);
pub static DEAD_BRAIN_CORAL: BlockData =
    BlockData::new(15149, 15149, &[&Properties::WATERLOGGED], None);
pub static DEAD_BUBBLE_CORAL: BlockData =
    BlockData::new(15151, 15151, &[&Properties::WATERLOGGED], None);
pub static DEAD_FIRE_CORAL: BlockData =
    BlockData::new(15153, 15153, &[&Properties::WATERLOGGED], None);
pub static DEAD_HORN_CORAL: BlockData =
    BlockData::new(15155, 15155, &[&Properties::WATERLOGGED], None);
pub static TUBE_CORAL: BlockData = BlockData::new(15157, 15157, &[&Properties::WATERLOGGED], None);
pub static BRAIN_CORAL: BlockData = BlockData::new(15159, 15159, &[&Properties::WATERLOGGED], None);
pub static BUBBLE_CORAL: BlockData =
    BlockData::new(15161, 15161, &[&Properties::WATERLOGGED], None);
pub static FIRE_CORAL: BlockData = BlockData::new(15163, 15163, &[&Properties::WATERLOGGED], None);
pub static HORN_CORAL: BlockData = BlockData::new(15165, 15165, &[&Properties::WATERLOGGED], None);
pub static DEAD_TUBE_CORAL_FAN: BlockData =
    BlockData::new(15167, 15167, &[&Properties::WATERLOGGED], None);
pub static DEAD_BRAIN_CORAL_FAN: BlockData =
    BlockData::new(15169, 15169, &[&Properties::WATERLOGGED], None);
pub static DEAD_BUBBLE_CORAL_FAN: BlockData =
    BlockData::new(15171, 15171, &[&Properties::WATERLOGGED], None);
pub static DEAD_FIRE_CORAL_FAN: BlockData =
    BlockData::new(15173, 15173, &[&Properties::WATERLOGGED], None);
pub static DEAD_HORN_CORAL_FAN: BlockData =
    BlockData::new(15175, 15175, &[&Properties::WATERLOGGED], None);
pub static TUBE_CORAL_FAN: BlockData =
    BlockData::new(15177, 15177, &[&Properties::WATERLOGGED], None);
pub static BRAIN_CORAL_FAN: BlockData =
    BlockData::new(15179, 15179, &[&Properties::WATERLOGGED], None);
pub static BUBBLE_CORAL_FAN: BlockData =
    BlockData::new(15181, 15181, &[&Properties::WATERLOGGED], None);
pub static FIRE_CORAL_FAN: BlockData =
    BlockData::new(15183, 15183, &[&Properties::WATERLOGGED], None);
pub static HORN_CORAL_FAN: BlockData =
    BlockData::new(15185, 15185, &[&Properties::WATERLOGGED], None);
pub static DEAD_TUBE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15187,
    15187,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static DEAD_BRAIN_CORAL_WALL_FAN: BlockData = BlockData::new(
    15195,
    15195,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static DEAD_BUBBLE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15203,
    15203,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static DEAD_FIRE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15211,
    15211,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static DEAD_HORN_CORAL_WALL_FAN: BlockData = BlockData::new(
    15219,
    15219,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static TUBE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15227,
    15227,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static BRAIN_CORAL_WALL_FAN: BlockData = BlockData::new(
    15235,
    15235,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static BUBBLE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15243,
    15243,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static FIRE_CORAL_WALL_FAN: BlockData = BlockData::new(
    15251,
    15251,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static HORN_CORAL_WALL_FAN: BlockData = BlockData::new(
    15259,
    15259,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static SEA_PICKLE: BlockData = BlockData::new(
    15267,
    15267,
    &[&Properties::PICKLES, &Properties::WATERLOGGED],
    None,
);
pub static BLUE_ICE: BlockData = BlockData::new(15275, 15275, &[], None);
pub static CONDUIT: BlockData = BlockData::new(
    15276,
    15276,
    &[&Properties::WATERLOGGED],
    Some(BlockEntityType::Conduit),
);
pub static BAMBOO_SAPLING: BlockData = BlockData::new(15278, 15278, &[], None);
pub static BAMBOO: BlockData = BlockData::new(
    15279,
    15279,
    &[
        &Properties::AGE_1,
        &Properties::BAMBOO_LEAVES,
        &Properties::STAGE,
    ],
    None,
);
pub static POTTED_BAMBOO: BlockData = BlockData::new(15291, 15291, &[], None);
pub static VOID_AIR: BlockData = BlockData::new(15292, 15292, &[], None);
pub static CAVE_AIR: BlockData = BlockData::new(15293, 15293, &[], None);
pub static BUBBLE_COLUMN: BlockData = BlockData::new(15294, 15294, &[&Properties::DRAG], None);
pub static POLISHED_GRANITE_STAIRS: BlockData = BlockData::new(
    15307,
    15296,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SMOOTH_RED_SANDSTONE_STAIRS: BlockData = BlockData::new(
    15387,
    15376,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MOSSY_STONE_BRICK_STAIRS: BlockData = BlockData::new(
    15467,
    15456,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_DIORITE_STAIRS: BlockData = BlockData::new(
    15547,
    15536,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MOSSY_COBBLESTONE_STAIRS: BlockData = BlockData::new(
    15627,
    15616,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static END_STONE_BRICK_STAIRS: BlockData = BlockData::new(
    15707,
    15696,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static STONE_STAIRS: BlockData = BlockData::new(
    15787,
    15776,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SMOOTH_SANDSTONE_STAIRS: BlockData = BlockData::new(
    15867,
    15856,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SMOOTH_QUARTZ_STAIRS: BlockData = BlockData::new(
    15947,
    15936,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static GRANITE_STAIRS: BlockData = BlockData::new(
    16027,
    16016,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static ANDESITE_STAIRS: BlockData = BlockData::new(
    16107,
    16096,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static RED_NETHER_BRICK_STAIRS: BlockData = BlockData::new(
    16187,
    16176,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_ANDESITE_STAIRS: BlockData = BlockData::new(
    16267,
    16256,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DIORITE_STAIRS: BlockData = BlockData::new(
    16347,
    16336,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_GRANITE_SLAB: BlockData = BlockData::new(
    16419,
    16416,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SMOOTH_RED_SANDSTONE_SLAB: BlockData = BlockData::new(
    16425,
    16422,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static MOSSY_STONE_BRICK_SLAB: BlockData = BlockData::new(
    16431,
    16428,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_DIORITE_SLAB: BlockData = BlockData::new(
    16437,
    16434,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static MOSSY_COBBLESTONE_SLAB: BlockData = BlockData::new(
    16443,
    16440,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static END_STONE_BRICK_SLAB: BlockData = BlockData::new(
    16449,
    16446,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SMOOTH_SANDSTONE_SLAB: BlockData = BlockData::new(
    16455,
    16452,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SMOOTH_QUARTZ_SLAB: BlockData = BlockData::new(
    16461,
    16458,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static GRANITE_SLAB: BlockData = BlockData::new(
    16467,
    16464,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static ANDESITE_SLAB: BlockData = BlockData::new(
    16473,
    16470,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static RED_NETHER_BRICK_SLAB: BlockData = BlockData::new(
    16479,
    16476,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_ANDESITE_SLAB: BlockData = BlockData::new(
    16485,
    16482,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static DIORITE_SLAB: BlockData = BlockData::new(
    16491,
    16488,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static BRICK_WALL: BlockData = BlockData::new(
    16497,
    16494,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static PRISMARINE_WALL: BlockData = BlockData::new(
    16821,
    16818,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static RED_SANDSTONE_WALL: BlockData = BlockData::new(
    17145,
    17142,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static MOSSY_STONE_BRICK_WALL: BlockData = BlockData::new(
    17469,
    17466,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static GRANITE_WALL: BlockData = BlockData::new(
    17793,
    17790,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static STONE_BRICK_WALL: BlockData = BlockData::new(
    18117,
    18114,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static MUD_BRICK_WALL: BlockData = BlockData::new(
    18441,
    18438,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static NETHER_BRICK_WALL: BlockData = BlockData::new(
    18765,
    18762,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static ANDESITE_WALL: BlockData = BlockData::new(
    19089,
    19086,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static RED_NETHER_BRICK_WALL: BlockData = BlockData::new(
    19413,
    19410,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static SANDSTONE_WALL: BlockData = BlockData::new(
    19737,
    19734,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static END_STONE_BRICK_WALL: BlockData = BlockData::new(
    20061,
    20058,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static DIORITE_WALL: BlockData = BlockData::new(
    20385,
    20382,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static SCAFFOLDING: BlockData = BlockData::new(
    20737,
    20706,
    &[
        &Properties::BOTTOM,
        &Properties::STABILITY_DISTANCE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static LOOM: BlockData = BlockData::new(20738, 20738, &[&Properties::HORIZONTAL_FACING], None);
pub static BARREL: BlockData = BlockData::new(
    20743,
    20742,
    &[&Properties::FACING, &Properties::OPEN],
    Some(BlockEntityType::Barrel),
);
pub static SMOKER: BlockData = BlockData::new(
    20755,
    20754,
    &[&Properties::HORIZONTAL_FACING, &Properties::LIT],
    Some(BlockEntityType::Smoker),
);
pub static BLAST_FURNACE: BlockData = BlockData::new(
    20763,
    20762,
    &[&Properties::HORIZONTAL_FACING, &Properties::LIT],
    Some(BlockEntityType::BlastFurnace),
);
pub static CARTOGRAPHY_TABLE: BlockData = BlockData::new(20770, 20770, &[], None);
pub static FLETCHING_TABLE: BlockData = BlockData::new(20771, 20771, &[], None);
pub static GRINDSTONE: BlockData = BlockData::new(
    20776,
    20772,
    &[&Properties::ATTACH_FACE, &Properties::HORIZONTAL_FACING],
    None,
);
pub static LECTERN: BlockData = BlockData::new(
    20787,
    20784,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HAS_BOOK,
        &Properties::POWERED,
    ],
    Some(BlockEntityType::Lectern),
);
pub static SMITHING_TABLE: BlockData = BlockData::new(20800, 20800, &[], None);
pub static STONECUTTER: BlockData =
    BlockData::new(20801, 20801, &[&Properties::HORIZONTAL_FACING], None);
pub static BELL: BlockData = BlockData::new(
    20806,
    20805,
    &[
        &Properties::BELL_ATTACHMENT,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    Some(BlockEntityType::Bell),
);
pub static LANTERN: BlockData = BlockData::new(
    20840,
    20837,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static SOUL_LANTERN: BlockData = BlockData::new(
    20844,
    20841,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static COPPER_LANTERN: BlockData = BlockData::new(
    20848,
    20845,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static EXPOSED_COPPER_LANTERN: BlockData = BlockData::new(
    20852,
    20849,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static WEATHERED_COPPER_LANTERN: BlockData = BlockData::new(
    20856,
    20853,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static OXIDIZED_COPPER_LANTERN: BlockData = BlockData::new(
    20860,
    20857,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_COPPER_LANTERN: BlockData = BlockData::new(
    20864,
    20861,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_EXPOSED_COPPER_LANTERN: BlockData = BlockData::new(
    20868,
    20865,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_WEATHERED_COPPER_LANTERN: BlockData = BlockData::new(
    20872,
    20869,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_OXIDIZED_COPPER_LANTERN: BlockData = BlockData::new(
    20876,
    20873,
    &[&Properties::HANGING, &Properties::WATERLOGGED],
    None,
);
pub static CAMPFIRE: BlockData = BlockData::new(
    20880,
    20877,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::LIT,
        &Properties::SIGNAL_FIRE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Campfire),
);
pub static SOUL_CAMPFIRE: BlockData = BlockData::new(
    20912,
    20909,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::LIT,
        &Properties::SIGNAL_FIRE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Campfire),
);
pub static SWEET_BERRY_BUSH: BlockData = BlockData::new(20941, 20941, &[&Properties::AGE_3], None);
pub static WARPED_STEM: BlockData = BlockData::new(20946, 20945, &[&Properties::AXIS], None);
pub static STRIPPED_WARPED_STEM: BlockData =
    BlockData::new(20949, 20948, &[&Properties::AXIS], None);
pub static WARPED_HYPHAE: BlockData = BlockData::new(20952, 20951, &[&Properties::AXIS], None);
pub static STRIPPED_WARPED_HYPHAE: BlockData =
    BlockData::new(20955, 20954, &[&Properties::AXIS], None);
pub static WARPED_NYLIUM: BlockData = BlockData::new(20957, 20957, &[], None);
pub static WARPED_FUNGUS: BlockData = BlockData::new(20958, 20958, &[], None);
pub static WARPED_WART_BLOCK: BlockData = BlockData::new(20959, 20959, &[], None);
pub static WARPED_ROOTS: BlockData = BlockData::new(20960, 20960, &[], None);
pub static NETHER_SPROUTS: BlockData = BlockData::new(20961, 20961, &[], None);
pub static CRIMSON_STEM: BlockData = BlockData::new(20963, 20962, &[&Properties::AXIS], None);
pub static STRIPPED_CRIMSON_STEM: BlockData =
    BlockData::new(20966, 20965, &[&Properties::AXIS], None);
pub static CRIMSON_HYPHAE: BlockData = BlockData::new(20969, 20968, &[&Properties::AXIS], None);
pub static STRIPPED_CRIMSON_HYPHAE: BlockData =
    BlockData::new(20972, 20971, &[&Properties::AXIS], None);
pub static CRIMSON_NYLIUM: BlockData = BlockData::new(20974, 20974, &[], None);
pub static CRIMSON_FUNGUS: BlockData = BlockData::new(20975, 20975, &[], None);
pub static SHROOMLIGHT: BlockData = BlockData::new(20976, 20976, &[], None);
pub static WEEPING_VINES: BlockData = BlockData::new(20977, 20977, &[&Properties::AGE_25], None);
pub static WEEPING_VINES_PLANT: BlockData = BlockData::new(21003, 21003, &[], None);
pub static TWISTING_VINES: BlockData = BlockData::new(21004, 21004, &[&Properties::AGE_25], None);
pub static TWISTING_VINES_PLANT: BlockData = BlockData::new(21030, 21030, &[], None);
pub static CRIMSON_ROOTS: BlockData = BlockData::new(21031, 21031, &[], None);
pub static CRIMSON_PLANKS: BlockData = BlockData::new(21032, 21032, &[], None);
pub static WARPED_PLANKS: BlockData = BlockData::new(21033, 21033, &[], None);
pub static CRIMSON_SLAB: BlockData = BlockData::new(
    21037,
    21034,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WARPED_SLAB: BlockData = BlockData::new(
    21043,
    21040,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CRIMSON_PRESSURE_PLATE: BlockData =
    BlockData::new(21047, 21046, &[&Properties::POWERED], None);
pub static WARPED_PRESSURE_PLATE: BlockData =
    BlockData::new(21049, 21048, &[&Properties::POWERED], None);
pub static CRIMSON_FENCE: BlockData = BlockData::new(
    21081,
    21050,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static WARPED_FENCE: BlockData = BlockData::new(
    21113,
    21082,
    &[
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static CRIMSON_TRAPDOOR: BlockData = BlockData::new(
    21129,
    21114,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WARPED_TRAPDOOR: BlockData = BlockData::new(
    21193,
    21178,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CRIMSON_FENCE_GATE: BlockData = BlockData::new(
    21249,
    21242,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WARPED_FENCE_GATE: BlockData = BlockData::new(
    21281,
    21274,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::IN_WALL,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static CRIMSON_STAIRS: BlockData = BlockData::new(
    21317,
    21306,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WARPED_STAIRS: BlockData = BlockData::new(
    21397,
    21386,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CRIMSON_BUTTON: BlockData = BlockData::new(
    21475,
    21466,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static WARPED_BUTTON: BlockData = BlockData::new(
    21499,
    21490,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static CRIMSON_DOOR: BlockData = BlockData::new(
    21525,
    21514,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WARPED_DOOR: BlockData = BlockData::new(
    21589,
    21578,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static CRIMSON_SIGN: BlockData = BlockData::new(
    21659,
    21642,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static WARPED_SIGN: BlockData = BlockData::new(
    21691,
    21674,
    &[&Properties::ROTATION_16, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static CRIMSON_WALL_SIGN: BlockData = BlockData::new(
    21707,
    21706,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static WARPED_WALL_SIGN: BlockData = BlockData::new(
    21715,
    21714,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    Some(BlockEntityType::Sign),
);
pub static STRUCTURE_BLOCK: BlockData = BlockData::new(
    21723,
    21722,
    &[&Properties::STRUCTUREBLOCK_MODE],
    Some(BlockEntityType::StructureBlock),
);
pub static JIGSAW: BlockData = BlockData::new(
    21736,
    21726,
    &[&Properties::ORIENTATION],
    Some(BlockEntityType::Jigsaw),
);
pub static TEST_BLOCK: BlockData = BlockData::new(
    21738,
    21738,
    &[&Properties::TEST_BLOCK_MODE],
    Some(BlockEntityType::TestBlock),
);
pub static TEST_INSTANCE_BLOCK: BlockData =
    BlockData::new(21742, 21742, &[], Some(BlockEntityType::TestInstanceBlock));
pub static COMPOSTER: BlockData =
    BlockData::new(21743, 21743, &[&Properties::LEVEL_COMPOSTER], None);
pub static TARGET: BlockData = BlockData::new(21752, 21752, &[&Properties::POWER], None);
pub static BEE_NEST: BlockData = BlockData::new(
    21768,
    21768,
    &[&Properties::HORIZONTAL_FACING, &Properties::LEVEL_HONEY],
    Some(BlockEntityType::Beehive),
);
pub static BEEHIVE: BlockData = BlockData::new(
    21792,
    21792,
    &[&Properties::HORIZONTAL_FACING, &Properties::LEVEL_HONEY],
    Some(BlockEntityType::Beehive),
);
pub static HONEY_BLOCK: BlockData = BlockData::new(21816, 21816, &[], None);
pub static HONEYCOMB_BLOCK: BlockData = BlockData::new(21817, 21817, &[], None);
pub static NETHERITE_BLOCK: BlockData = BlockData::new(21818, 21818, &[], None);
pub static ANCIENT_DEBRIS: BlockData = BlockData::new(21819, 21819, &[], None);
pub static CRYING_OBSIDIAN: BlockData = BlockData::new(21820, 21820, &[], None);
pub static RESPAWN_ANCHOR: BlockData =
    BlockData::new(21821, 21821, &[&Properties::RESPAWN_ANCHOR_CHARGES], None);
pub static POTTED_CRIMSON_FUNGUS: BlockData = BlockData::new(21826, 21826, &[], None);
pub static POTTED_WARPED_FUNGUS: BlockData = BlockData::new(21827, 21827, &[], None);
pub static POTTED_CRIMSON_ROOTS: BlockData = BlockData::new(21828, 21828, &[], None);
pub static POTTED_WARPED_ROOTS: BlockData = BlockData::new(21829, 21829, &[], None);
pub static LODESTONE: BlockData = BlockData::new(21830, 21830, &[], None);
pub static BLACKSTONE: BlockData = BlockData::new(21831, 21831, &[], None);
pub static BLACKSTONE_STAIRS: BlockData = BlockData::new(
    21843,
    21832,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BLACKSTONE_WALL: BlockData = BlockData::new(
    21915,
    21912,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static BLACKSTONE_SLAB: BlockData = BlockData::new(
    22239,
    22236,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_BLACKSTONE: BlockData = BlockData::new(22242, 22242, &[], None);
pub static POLISHED_BLACKSTONE_BRICKS: BlockData = BlockData::new(22243, 22243, &[], None);
pub static CRACKED_POLISHED_BLACKSTONE_BRICKS: BlockData = BlockData::new(22244, 22244, &[], None);
pub static CHISELED_POLISHED_BLACKSTONE: BlockData = BlockData::new(22245, 22245, &[], None);
pub static POLISHED_BLACKSTONE_BRICK_SLAB: BlockData = BlockData::new(
    22249,
    22246,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_BLACKSTONE_BRICK_STAIRS: BlockData = BlockData::new(
    22263,
    22252,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_BLACKSTONE_BRICK_WALL: BlockData = BlockData::new(
    22335,
    22332,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static GILDED_BLACKSTONE: BlockData = BlockData::new(22656, 22656, &[], None);
pub static POLISHED_BLACKSTONE_STAIRS: BlockData = BlockData::new(
    22668,
    22657,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_BLACKSTONE_SLAB: BlockData = BlockData::new(
    22740,
    22737,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_BLACKSTONE_PRESSURE_PLATE: BlockData =
    BlockData::new(22744, 22743, &[&Properties::POWERED], None);
pub static POLISHED_BLACKSTONE_BUTTON: BlockData = BlockData::new(
    22754,
    22745,
    &[
        &Properties::ATTACH_FACE,
        &Properties::HORIZONTAL_FACING,
        &Properties::POWERED,
    ],
    None,
);
pub static POLISHED_BLACKSTONE_WALL: BlockData = BlockData::new(
    22772,
    22769,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_NETHER_BRICKS: BlockData = BlockData::new(23093, 23093, &[], None);
pub static CRACKED_NETHER_BRICKS: BlockData = BlockData::new(23094, 23094, &[], None);
pub static QUARTZ_BRICKS: BlockData = BlockData::new(23095, 23095, &[], None);
pub static CANDLE: BlockData = BlockData::new(
    23099,
    23096,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WHITE_CANDLE: BlockData = BlockData::new(
    23115,
    23112,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static ORANGE_CANDLE: BlockData = BlockData::new(
    23131,
    23128,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static MAGENTA_CANDLE: BlockData = BlockData::new(
    23147,
    23144,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static LIGHT_BLUE_CANDLE: BlockData = BlockData::new(
    23163,
    23160,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static YELLOW_CANDLE: BlockData = BlockData::new(
    23179,
    23176,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static LIME_CANDLE: BlockData = BlockData::new(
    23195,
    23192,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PINK_CANDLE: BlockData = BlockData::new(
    23211,
    23208,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static GRAY_CANDLE: BlockData = BlockData::new(
    23227,
    23224,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static LIGHT_GRAY_CANDLE: BlockData = BlockData::new(
    23243,
    23240,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CYAN_CANDLE: BlockData = BlockData::new(
    23259,
    23256,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static PURPLE_CANDLE: BlockData = BlockData::new(
    23275,
    23272,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BLUE_CANDLE: BlockData = BlockData::new(
    23291,
    23288,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BROWN_CANDLE: BlockData = BlockData::new(
    23307,
    23304,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static GREEN_CANDLE: BlockData = BlockData::new(
    23323,
    23320,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static RED_CANDLE: BlockData = BlockData::new(
    23339,
    23336,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BLACK_CANDLE: BlockData = BlockData::new(
    23355,
    23352,
    &[
        &Properties::CANDLES,
        &Properties::LIT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CANDLE_CAKE: BlockData = BlockData::new(23369, 23368, &[&Properties::LIT], None);
pub static WHITE_CANDLE_CAKE: BlockData = BlockData::new(23371, 23370, &[&Properties::LIT], None);
pub static ORANGE_CANDLE_CAKE: BlockData = BlockData::new(23373, 23372, &[&Properties::LIT], None);
pub static MAGENTA_CANDLE_CAKE: BlockData = BlockData::new(23375, 23374, &[&Properties::LIT], None);
pub static LIGHT_BLUE_CANDLE_CAKE: BlockData =
    BlockData::new(23377, 23376, &[&Properties::LIT], None);
pub static YELLOW_CANDLE_CAKE: BlockData = BlockData::new(23379, 23378, &[&Properties::LIT], None);
pub static LIME_CANDLE_CAKE: BlockData = BlockData::new(23381, 23380, &[&Properties::LIT], None);
pub static PINK_CANDLE_CAKE: BlockData = BlockData::new(23383, 23382, &[&Properties::LIT], None);
pub static GRAY_CANDLE_CAKE: BlockData = BlockData::new(23385, 23384, &[&Properties::LIT], None);
pub static LIGHT_GRAY_CANDLE_CAKE: BlockData =
    BlockData::new(23387, 23386, &[&Properties::LIT], None);
pub static CYAN_CANDLE_CAKE: BlockData = BlockData::new(23389, 23388, &[&Properties::LIT], None);
pub static PURPLE_CANDLE_CAKE: BlockData = BlockData::new(23391, 23390, &[&Properties::LIT], None);
pub static BLUE_CANDLE_CAKE: BlockData = BlockData::new(23393, 23392, &[&Properties::LIT], None);
pub static BROWN_CANDLE_CAKE: BlockData = BlockData::new(23395, 23394, &[&Properties::LIT], None);
pub static GREEN_CANDLE_CAKE: BlockData = BlockData::new(23397, 23396, &[&Properties::LIT], None);
pub static RED_CANDLE_CAKE: BlockData = BlockData::new(23399, 23398, &[&Properties::LIT], None);
pub static BLACK_CANDLE_CAKE: BlockData = BlockData::new(23401, 23400, &[&Properties::LIT], None);
pub static AMETHYST_BLOCK: BlockData = BlockData::new(23402, 23402, &[], None);
pub static BUDDING_AMETHYST: BlockData = BlockData::new(23403, 23403, &[], None);
pub static AMETHYST_CLUSTER: BlockData = BlockData::new(
    23413,
    23404,
    &[&Properties::FACING, &Properties::WATERLOGGED],
    None,
);
pub static LARGE_AMETHYST_BUD: BlockData = BlockData::new(
    23425,
    23416,
    &[&Properties::FACING, &Properties::WATERLOGGED],
    None,
);
pub static MEDIUM_AMETHYST_BUD: BlockData = BlockData::new(
    23437,
    23428,
    &[&Properties::FACING, &Properties::WATERLOGGED],
    None,
);
pub static SMALL_AMETHYST_BUD: BlockData = BlockData::new(
    23449,
    23440,
    &[&Properties::FACING, &Properties::WATERLOGGED],
    None,
);
pub static TUFF: BlockData = BlockData::new(23452, 23452, &[], None);
pub static TUFF_SLAB: BlockData = BlockData::new(
    23456,
    23453,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static TUFF_STAIRS: BlockData = BlockData::new(
    23470,
    23459,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static TUFF_WALL: BlockData = BlockData::new(
    23542,
    23539,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static POLISHED_TUFF: BlockData = BlockData::new(23863, 23863, &[], None);
pub static POLISHED_TUFF_SLAB: BlockData = BlockData::new(
    23867,
    23864,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_TUFF_STAIRS: BlockData = BlockData::new(
    23881,
    23870,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_TUFF_WALL: BlockData = BlockData::new(
    23953,
    23950,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_TUFF: BlockData = BlockData::new(24274, 24274, &[], None);
pub static TUFF_BRICKS: BlockData = BlockData::new(24275, 24275, &[], None);
pub static TUFF_BRICK_SLAB: BlockData = BlockData::new(
    24279,
    24276,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static TUFF_BRICK_STAIRS: BlockData = BlockData::new(
    24293,
    24282,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static TUFF_BRICK_WALL: BlockData = BlockData::new(
    24365,
    24362,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_TUFF_BRICKS: BlockData = BlockData::new(24686, 24686, &[], None);
pub static SULFUR: BlockData = BlockData::new(24687, 24687, &[], None);
pub static POTENT_SULFUR: BlockData = BlockData::new(
    24688,
    24688,
    &[&Properties::POTENT_SULFUR_STATE],
    Some(BlockEntityType::PotentSulfur),
);
pub static SULFUR_SLAB: BlockData = BlockData::new(
    24696,
    24693,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SULFUR_STAIRS: BlockData = BlockData::new(
    24710,
    24699,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SULFUR_WALL: BlockData = BlockData::new(
    24782,
    24779,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static POLISHED_SULFUR: BlockData = BlockData::new(25103, 25103, &[], None);
pub static POLISHED_SULFUR_SLAB: BlockData = BlockData::new(
    25107,
    25104,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_SULFUR_STAIRS: BlockData = BlockData::new(
    25121,
    25110,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_SULFUR_WALL: BlockData = BlockData::new(
    25193,
    25190,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static SULFUR_BRICKS: BlockData = BlockData::new(25514, 25514, &[], None);
pub static SULFUR_BRICK_SLAB: BlockData = BlockData::new(
    25518,
    25515,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static SULFUR_BRICK_STAIRS: BlockData = BlockData::new(
    25532,
    25521,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SULFUR_BRICK_WALL: BlockData = BlockData::new(
    25604,
    25601,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_SULFUR: BlockData = BlockData::new(25925, 25925, &[], None);
pub static CINNABAR: BlockData = BlockData::new(25926, 25926, &[], None);
pub static CINNABAR_SLAB: BlockData = BlockData::new(
    25930,
    25927,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CINNABAR_STAIRS: BlockData = BlockData::new(
    25944,
    25933,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CINNABAR_WALL: BlockData = BlockData::new(
    26016,
    26013,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static POLISHED_CINNABAR: BlockData = BlockData::new(26337, 26337, &[], None);
pub static POLISHED_CINNABAR_SLAB: BlockData = BlockData::new(
    26341,
    26338,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_CINNABAR_STAIRS: BlockData = BlockData::new(
    26355,
    26344,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_CINNABAR_WALL: BlockData = BlockData::new(
    26427,
    26424,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CINNABAR_BRICKS: BlockData = BlockData::new(26748, 26748, &[], None);
pub static CINNABAR_BRICK_SLAB: BlockData = BlockData::new(
    26752,
    26749,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static CINNABAR_BRICK_STAIRS: BlockData = BlockData::new(
    26766,
    26755,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CINNABAR_BRICK_WALL: BlockData = BlockData::new(
    26838,
    26835,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_CINNABAR: BlockData = BlockData::new(27159, 27159, &[], None);
pub static CALCITE: BlockData = BlockData::new(27160, 27160, &[], None);
pub static TINTED_GLASS: BlockData = BlockData::new(27161, 27161, &[], None);
pub static POWDER_SNOW: BlockData = BlockData::new(27162, 27162, &[], None);
pub static SCULK_SENSOR: BlockData = BlockData::new(
    27164,
    27163,
    &[
        &Properties::POWER,
        &Properties::SCULK_SENSOR_PHASE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::SculkSensor),
);
pub static CALIBRATED_SCULK_SENSOR: BlockData = BlockData::new(
    27260,
    27259,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::POWER,
        &Properties::SCULK_SENSOR_PHASE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CalibratedSculkSensor),
);
pub static SCULK: BlockData = BlockData::new(27643, 27643, &[], None);
pub static SCULK_VEIN: BlockData = BlockData::new(
    27771,
    27644,
    &[
        &Properties::DOWN,
        &Properties::EAST,
        &Properties::NORTH,
        &Properties::SOUTH,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST,
    ],
    None,
);
pub static SCULK_CATALYST: BlockData = BlockData::new(
    27773,
    27772,
    &[&Properties::BLOOM],
    Some(BlockEntityType::SculkCatalyst),
);
pub static SCULK_SHRIEKER: BlockData = BlockData::new(
    27781,
    27774,
    &[
        &Properties::CAN_SUMMON,
        &Properties::SHRIEKING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::SculkShrieker),
);
pub static COPPER_BLOCK: BlockData = BlockData::new(27782, 27782, &[], None);
pub static EXPOSED_COPPER: BlockData = BlockData::new(27783, 27783, &[], None);
pub static WEATHERED_COPPER: BlockData = BlockData::new(27784, 27784, &[], None);
pub static OXIDIZED_COPPER: BlockData = BlockData::new(27785, 27785, &[], None);
pub static WAXED_COPPER_BLOCK: BlockData = BlockData::new(27786, 27786, &[], None);
pub static WAXED_EXPOSED_COPPER: BlockData = BlockData::new(27787, 27787, &[], None);
pub static WAXED_WEATHERED_COPPER: BlockData = BlockData::new(27788, 27788, &[], None);
pub static WAXED_OXIDIZED_COPPER: BlockData = BlockData::new(27789, 27789, &[], None);
pub static COPPER_ORE: BlockData = BlockData::new(27790, 27790, &[], None);
pub static DEEPSLATE_COPPER_ORE: BlockData = BlockData::new(27791, 27791, &[], None);
pub static CUT_COPPER: BlockData = BlockData::new(27792, 27792, &[], None);
pub static EXPOSED_CUT_COPPER: BlockData = BlockData::new(27793, 27793, &[], None);
pub static WEATHERED_CUT_COPPER: BlockData = BlockData::new(27794, 27794, &[], None);
pub static OXIDIZED_CUT_COPPER: BlockData = BlockData::new(27795, 27795, &[], None);
pub static WAXED_CUT_COPPER: BlockData = BlockData::new(27796, 27796, &[], None);
pub static WAXED_EXPOSED_CUT_COPPER: BlockData = BlockData::new(27797, 27797, &[], None);
pub static WAXED_WEATHERED_CUT_COPPER: BlockData = BlockData::new(27798, 27798, &[], None);
pub static WAXED_OXIDIZED_CUT_COPPER: BlockData = BlockData::new(27799, 27799, &[], None);
pub static CHISELED_COPPER: BlockData = BlockData::new(27800, 27800, &[], None);
pub static EXPOSED_CHISELED_COPPER: BlockData = BlockData::new(27801, 27801, &[], None);
pub static WEATHERED_CHISELED_COPPER: BlockData = BlockData::new(27802, 27802, &[], None);
pub static OXIDIZED_CHISELED_COPPER: BlockData = BlockData::new(27803, 27803, &[], None);
pub static WAXED_CHISELED_COPPER: BlockData = BlockData::new(27804, 27804, &[], None);
pub static WAXED_EXPOSED_CHISELED_COPPER: BlockData = BlockData::new(27805, 27805, &[], None);
pub static WAXED_WEATHERED_CHISELED_COPPER: BlockData = BlockData::new(27806, 27806, &[], None);
pub static WAXED_OXIDIZED_CHISELED_COPPER: BlockData = BlockData::new(27807, 27807, &[], None);
pub static CUT_COPPER_STAIRS: BlockData = BlockData::new(
    27819,
    27808,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static EXPOSED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    27899,
    27888,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WEATHERED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    27979,
    27968,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static OXIDIZED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    28059,
    28048,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    28139,
    28128,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_EXPOSED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    28219,
    28208,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_WEATHERED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    28299,
    28288,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_OXIDIZED_CUT_COPPER_STAIRS: BlockData = BlockData::new(
    28379,
    28368,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CUT_COPPER_SLAB: BlockData = BlockData::new(
    28451,
    28448,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static EXPOSED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28457,
    28454,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WEATHERED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28463,
    28460,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static OXIDIZED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28469,
    28466,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28475,
    28472,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_EXPOSED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28481,
    28478,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_WEATHERED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28487,
    28484,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static WAXED_OXIDIZED_CUT_COPPER_SLAB: BlockData = BlockData::new(
    28493,
    28490,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static COPPER_DOOR: BlockData = BlockData::new(
    28507,
    28496,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static EXPOSED_COPPER_DOOR: BlockData = BlockData::new(
    28571,
    28560,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WEATHERED_COPPER_DOOR: BlockData = BlockData::new(
    28635,
    28624,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static OXIDIZED_COPPER_DOOR: BlockData = BlockData::new(
    28699,
    28688,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WAXED_COPPER_DOOR: BlockData = BlockData::new(
    28763,
    28752,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WAXED_EXPOSED_COPPER_DOOR: BlockData = BlockData::new(
    28827,
    28816,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WAXED_WEATHERED_COPPER_DOOR: BlockData = BlockData::new(
    28891,
    28880,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static WAXED_OXIDIZED_COPPER_DOOR: BlockData = BlockData::new(
    28955,
    28944,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::DOOR_HINGE,
        &Properties::OPEN,
        &Properties::POWERED,
    ],
    None,
);
pub static COPPER_TRAPDOOR: BlockData = BlockData::new(
    29023,
    29008,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static EXPOSED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29087,
    29072,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WEATHERED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29151,
    29136,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static OXIDIZED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29215,
    29200,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29279,
    29264,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_EXPOSED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29343,
    29328,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_WEATHERED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29407,
    29392,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_OXIDIZED_COPPER_TRAPDOOR: BlockData = BlockData::new(
    29471,
    29456,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::OPEN,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static COPPER_GRATE: BlockData =
    BlockData::new(29521, 29520, &[&Properties::WATERLOGGED], None);
pub static EXPOSED_COPPER_GRATE: BlockData =
    BlockData::new(29523, 29522, &[&Properties::WATERLOGGED], None);
pub static WEATHERED_COPPER_GRATE: BlockData =
    BlockData::new(29525, 29524, &[&Properties::WATERLOGGED], None);
pub static OXIDIZED_COPPER_GRATE: BlockData =
    BlockData::new(29527, 29526, &[&Properties::WATERLOGGED], None);
pub static WAXED_COPPER_GRATE: BlockData =
    BlockData::new(29529, 29528, &[&Properties::WATERLOGGED], None);
pub static WAXED_EXPOSED_COPPER_GRATE: BlockData =
    BlockData::new(29531, 29530, &[&Properties::WATERLOGGED], None);
pub static WAXED_WEATHERED_COPPER_GRATE: BlockData =
    BlockData::new(29533, 29532, &[&Properties::WATERLOGGED], None);
pub static WAXED_OXIDIZED_COPPER_GRATE: BlockData =
    BlockData::new(29535, 29534, &[&Properties::WATERLOGGED], None);
pub static COPPER_BULB: BlockData = BlockData::new(
    29539,
    29536,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static EXPOSED_COPPER_BULB: BlockData = BlockData::new(
    29543,
    29540,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static WEATHERED_COPPER_BULB: BlockData = BlockData::new(
    29547,
    29544,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static OXIDIZED_COPPER_BULB: BlockData = BlockData::new(
    29551,
    29548,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static WAXED_COPPER_BULB: BlockData = BlockData::new(
    29555,
    29552,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static WAXED_EXPOSED_COPPER_BULB: BlockData = BlockData::new(
    29559,
    29556,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static WAXED_WEATHERED_COPPER_BULB: BlockData = BlockData::new(
    29563,
    29560,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static WAXED_OXIDIZED_COPPER_BULB: BlockData = BlockData::new(
    29567,
    29564,
    &[&Properties::LIT, &Properties::POWERED],
    None,
);
pub static COPPER_CHEST: BlockData = BlockData::new(
    29569,
    29568,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static EXPOSED_COPPER_CHEST: BlockData = BlockData::new(
    29593,
    29592,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static WEATHERED_COPPER_CHEST: BlockData = BlockData::new(
    29617,
    29616,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static OXIDIZED_COPPER_CHEST: BlockData = BlockData::new(
    29641,
    29640,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static WAXED_COPPER_CHEST: BlockData = BlockData::new(
    29665,
    29664,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static WAXED_EXPOSED_COPPER_CHEST: BlockData = BlockData::new(
    29689,
    29688,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static WAXED_WEATHERED_COPPER_CHEST: BlockData = BlockData::new(
    29713,
    29712,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static WAXED_OXIDIZED_COPPER_CHEST: BlockData = BlockData::new(
    29737,
    29736,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::CHEST_TYPE,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::Chest),
);
pub static COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29761,
    29760,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static EXPOSED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29793,
    29792,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static WEATHERED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29825,
    29824,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static OXIDIZED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29857,
    29856,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static WAXED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29889,
    29888,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static WAXED_EXPOSED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29921,
    29920,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static WAXED_WEATHERED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29953,
    29952,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static WAXED_OXIDIZED_COPPER_GOLEM_STATUE: BlockData = BlockData::new(
    29985,
    29984,
    &[
        &Properties::COPPER_GOLEM_POSE,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::CopperGolemStatue),
);
pub static LIGHTNING_ROD: BlockData = BlockData::new(
    30035,
    30016,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static EXPOSED_LIGHTNING_ROD: BlockData = BlockData::new(
    30059,
    30040,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WEATHERED_LIGHTNING_ROD: BlockData = BlockData::new(
    30083,
    30064,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static OXIDIZED_LIGHTNING_ROD: BlockData = BlockData::new(
    30107,
    30088,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_LIGHTNING_ROD: BlockData = BlockData::new(
    30131,
    30112,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_EXPOSED_LIGHTNING_ROD: BlockData = BlockData::new(
    30155,
    30136,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_WEATHERED_LIGHTNING_ROD: BlockData = BlockData::new(
    30179,
    30160,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static WAXED_OXIDIZED_LIGHTNING_ROD: BlockData = BlockData::new(
    30203,
    30184,
    &[
        &Properties::FACING,
        &Properties::POWERED,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DRIPSTONE_BLOCK: BlockData = BlockData::new(30208, 30208, &[], None);
pub static POINTED_DRIPSTONE: BlockData = BlockData::new(
    30214,
    30209,
    &[
        &Properties::SPELEOTHEM_THICKNESS,
        &Properties::VERTICAL_DIRECTION,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static SULFUR_SPIKE: BlockData = BlockData::new(
    30234,
    30229,
    &[
        &Properties::SPELEOTHEM_THICKNESS,
        &Properties::VERTICAL_DIRECTION,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static CAVE_VINES: BlockData = BlockData::new(
    30250,
    30249,
    &[&Properties::AGE_25, &Properties::BERRIES],
    None,
);
pub static CAVE_VINES_PLANT: BlockData =
    BlockData::new(30302, 30301, &[&Properties::BERRIES], None);
pub static SPORE_BLOSSOM: BlockData = BlockData::new(30303, 30303, &[], None);
pub static AZALEA: BlockData = BlockData::new(30304, 30304, &[], None);
pub static FLOWERING_AZALEA: BlockData = BlockData::new(30305, 30305, &[], None);
pub static MOSS_CARPET: BlockData = BlockData::new(30306, 30306, &[], None);
pub static PINK_PETALS: BlockData = BlockData::new(
    30307,
    30307,
    &[&Properties::HORIZONTAL_FACING, &Properties::FLOWER_AMOUNT],
    None,
);
pub static WILDFLOWERS: BlockData = BlockData::new(
    30323,
    30323,
    &[&Properties::HORIZONTAL_FACING, &Properties::FLOWER_AMOUNT],
    None,
);
pub static LEAF_LITTER: BlockData = BlockData::new(
    30339,
    30339,
    &[&Properties::HORIZONTAL_FACING, &Properties::SEGMENT_AMOUNT],
    None,
);
pub static MOSS_BLOCK: BlockData = BlockData::new(30355, 30355, &[], None);
pub static BIG_DRIPLEAF: BlockData = BlockData::new(
    30357,
    30356,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::TILT,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static BIG_DRIPLEAF_STEM: BlockData = BlockData::new(
    30389,
    30388,
    &[&Properties::HORIZONTAL_FACING, &Properties::WATERLOGGED],
    None,
);
pub static SMALL_DRIPLEAF: BlockData = BlockData::new(
    30399,
    30396,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::DOUBLE_BLOCK_HALF,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static HANGING_ROOTS: BlockData =
    BlockData::new(30413, 30412, &[&Properties::WATERLOGGED], None);
pub static ROOTED_DIRT: BlockData = BlockData::new(30414, 30414, &[], None);
pub static MUD: BlockData = BlockData::new(30415, 30415, &[], None);
pub static DEEPSLATE: BlockData = BlockData::new(30417, 30416, &[&Properties::AXIS], None);
pub static COBBLED_DEEPSLATE: BlockData = BlockData::new(30419, 30419, &[], None);
pub static COBBLED_DEEPSLATE_STAIRS: BlockData = BlockData::new(
    30431,
    30420,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static COBBLED_DEEPSLATE_SLAB: BlockData = BlockData::new(
    30503,
    30500,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static COBBLED_DEEPSLATE_WALL: BlockData = BlockData::new(
    30509,
    30506,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static POLISHED_DEEPSLATE: BlockData = BlockData::new(30830, 30830, &[], None);
pub static POLISHED_DEEPSLATE_STAIRS: BlockData = BlockData::new(
    30842,
    30831,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static POLISHED_DEEPSLATE_SLAB: BlockData = BlockData::new(
    30914,
    30911,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static POLISHED_DEEPSLATE_WALL: BlockData = BlockData::new(
    30920,
    30917,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static DEEPSLATE_TILES: BlockData = BlockData::new(31241, 31241, &[], None);
pub static DEEPSLATE_TILE_STAIRS: BlockData = BlockData::new(
    31253,
    31242,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DEEPSLATE_TILE_SLAB: BlockData = BlockData::new(
    31325,
    31322,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static DEEPSLATE_TILE_WALL: BlockData = BlockData::new(
    31331,
    31328,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static DEEPSLATE_BRICKS: BlockData = BlockData::new(31652, 31652, &[], None);
pub static DEEPSLATE_BRICK_STAIRS: BlockData = BlockData::new(
    31664,
    31653,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::HALF,
        &Properties::STAIRS_SHAPE,
        &Properties::WATERLOGGED,
    ],
    None,
);
pub static DEEPSLATE_BRICK_SLAB: BlockData = BlockData::new(
    31736,
    31733,
    &[&Properties::SLAB_TYPE, &Properties::WATERLOGGED],
    None,
);
pub static DEEPSLATE_BRICK_WALL: BlockData = BlockData::new(
    31742,
    31739,
    &[
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::UP,
        &Properties::WATERLOGGED,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static CHISELED_DEEPSLATE: BlockData = BlockData::new(32063, 32063, &[], None);
pub static CRACKED_DEEPSLATE_BRICKS: BlockData = BlockData::new(32064, 32064, &[], None);
pub static CRACKED_DEEPSLATE_TILES: BlockData = BlockData::new(32065, 32065, &[], None);
pub static INFESTED_DEEPSLATE: BlockData = BlockData::new(32067, 32066, &[&Properties::AXIS], None);
pub static SMOOTH_BASALT: BlockData = BlockData::new(32069, 32069, &[], None);
pub static RAW_IRON_BLOCK: BlockData = BlockData::new(32070, 32070, &[], None);
pub static RAW_COPPER_BLOCK: BlockData = BlockData::new(32071, 32071, &[], None);
pub static RAW_GOLD_BLOCK: BlockData = BlockData::new(32072, 32072, &[], None);
pub static POTTED_AZALEA_BUSH: BlockData = BlockData::new(32073, 32073, &[], None);
pub static POTTED_FLOWERING_AZALEA_BUSH: BlockData = BlockData::new(32074, 32074, &[], None);
pub static OCHRE_FROGLIGHT: BlockData = BlockData::new(32076, 32075, &[&Properties::AXIS], None);
pub static VERDANT_FROGLIGHT: BlockData = BlockData::new(32079, 32078, &[&Properties::AXIS], None);
pub static PEARLESCENT_FROGLIGHT: BlockData =
    BlockData::new(32082, 32081, &[&Properties::AXIS], None);
pub static FROGSPAWN: BlockData = BlockData::new(32084, 32084, &[], None);
pub static REINFORCED_DEEPSLATE: BlockData = BlockData::new(32085, 32085, &[], None);
pub static DECORATED_POT: BlockData = BlockData::new(
    32095,
    32086,
    &[
        &Properties::CRACKED,
        &Properties::HORIZONTAL_FACING,
        &Properties::WATERLOGGED,
    ],
    Some(BlockEntityType::DecoratedPot),
);
pub static CRAFTER: BlockData = BlockData::new(
    32147,
    32102,
    &[
        &Properties::CRAFTING,
        &Properties::ORIENTATION,
        &Properties::TRIGGERED,
    ],
    Some(BlockEntityType::Crafter),
);
pub static TRIAL_SPAWNER: BlockData = BlockData::new(
    32156,
    32150,
    &[&Properties::OMINOUS, &Properties::TRIAL_SPAWNER_STATE],
    Some(BlockEntityType::TrialSpawner),
);
pub static VAULT: BlockData = BlockData::new(
    32166,
    32162,
    &[
        &Properties::HORIZONTAL_FACING,
        &Properties::OMINOUS,
        &Properties::VAULT_STATE,
    ],
    Some(BlockEntityType::Vault),
);
pub static HEAVY_CORE: BlockData = BlockData::new(32195, 32194, &[&Properties::WATERLOGGED], None);
pub static PALE_MOSS_BLOCK: BlockData = BlockData::new(32196, 32196, &[], None);
pub static PALE_MOSS_CARPET: BlockData = BlockData::new(
    32197,
    32197,
    &[
        &Properties::BOTTOM,
        &Properties::EAST_WALL,
        &Properties::NORTH_WALL,
        &Properties::SOUTH_WALL,
        &Properties::WEST_WALL,
    ],
    None,
);
pub static PALE_HANGING_MOSS: BlockData = BlockData::new(32359, 32359, &[&Properties::TIP], None);
pub static OPEN_EYEBLOSSOM: BlockData = BlockData::new(32361, 32361, &[], None);
pub static CLOSED_EYEBLOSSOM: BlockData = BlockData::new(32362, 32362, &[], None);
pub static POTTED_OPEN_EYEBLOSSOM: BlockData = BlockData::new(32363, 32363, &[], None);
pub static POTTED_CLOSED_EYEBLOSSOM: BlockData = BlockData::new(32364, 32364, &[], None);
pub static FIREFLY_BUSH: BlockData = BlockData::new(32365, 32365, &[], None);
pub(crate) fn register_all(registry: &mut Registry<Block>) {
    let mut register = |key: &'static str, value: Block| {
        Registry::register(registry, key.into(), value);
    };
    register("minecraft:air", Block::Air);
    register("minecraft:stone", Block::Stone);
    register("minecraft:granite", Block::Granite);
    register("minecraft:polished_granite", Block::PolishedGranite);
    register("minecraft:diorite", Block::Diorite);
    register("minecraft:polished_diorite", Block::PolishedDiorite);
    register("minecraft:andesite", Block::Andesite);
    register("minecraft:polished_andesite", Block::PolishedAndesite);
    register("minecraft:grass_block", Block::GrassBlock);
    register("minecraft:dirt", Block::Dirt);
    register("minecraft:coarse_dirt", Block::CoarseDirt);
    register("minecraft:podzol", Block::Podzol);
    register("minecraft:cobblestone", Block::Cobblestone);
    register("minecraft:oak_planks", Block::OakPlanks);
    register("minecraft:spruce_planks", Block::SprucePlanks);
    register("minecraft:birch_planks", Block::BirchPlanks);
    register("minecraft:jungle_planks", Block::JunglePlanks);
    register("minecraft:acacia_planks", Block::AcaciaPlanks);
    register("minecraft:cherry_planks", Block::CherryPlanks);
    register("minecraft:dark_oak_planks", Block::DarkOakPlanks);
    register("minecraft:pale_oak_wood", Block::PaleOakWood);
    register("minecraft:pale_oak_planks", Block::PaleOakPlanks);
    register("minecraft:mangrove_planks", Block::MangrovePlanks);
    register("minecraft:bamboo_planks", Block::BambooPlanks);
    register("minecraft:bamboo_mosaic", Block::BambooMosaic);
    register("minecraft:oak_sapling", Block::OakSapling);
    register("minecraft:spruce_sapling", Block::SpruceSapling);
    register("minecraft:birch_sapling", Block::BirchSapling);
    register("minecraft:jungle_sapling", Block::JungleSapling);
    register("minecraft:acacia_sapling", Block::AcaciaSapling);
    register("minecraft:cherry_sapling", Block::CherrySapling);
    register("minecraft:dark_oak_sapling", Block::DarkOakSapling);
    register("minecraft:pale_oak_sapling", Block::PaleOakSapling);
    register("minecraft:mangrove_propagule", Block::MangrovePropagule);
    register("minecraft:bedrock", Block::Bedrock);
    register("minecraft:water", Block::Water);
    register("minecraft:lava", Block::Lava);
    register("minecraft:sand", Block::Sand);
    register("minecraft:suspicious_sand", Block::SuspiciousSand);
    register("minecraft:red_sand", Block::RedSand);
    register("minecraft:gravel", Block::Gravel);
    register("minecraft:suspicious_gravel", Block::SuspiciousGravel);
    register("minecraft:gold_ore", Block::GoldOre);
    register("minecraft:deepslate_gold_ore", Block::DeepslateGoldOre);
    register("minecraft:iron_ore", Block::IronOre);
    register("minecraft:deepslate_iron_ore", Block::DeepslateIronOre);
    register("minecraft:coal_ore", Block::CoalOre);
    register("minecraft:deepslate_coal_ore", Block::DeepslateCoalOre);
    register("minecraft:nether_gold_ore", Block::NetherGoldOre);
    register("minecraft:oak_log", Block::OakLog);
    register("minecraft:spruce_log", Block::SpruceLog);
    register("minecraft:birch_log", Block::BirchLog);
    register("minecraft:jungle_log", Block::JungleLog);
    register("minecraft:acacia_log", Block::AcaciaLog);
    register("minecraft:cherry_log", Block::CherryLog);
    register("minecraft:dark_oak_log", Block::DarkOakLog);
    register("minecraft:pale_oak_log", Block::PaleOakLog);
    register("minecraft:mangrove_log", Block::MangroveLog);
    register("minecraft:mangrove_roots", Block::MangroveRoots);
    register("minecraft:muddy_mangrove_roots", Block::MuddyMangroveRoots);
    register("minecraft:bamboo_block", Block::BambooBlock);
    register("minecraft:stripped_spruce_log", Block::StrippedSpruceLog);
    register("minecraft:stripped_birch_log", Block::StrippedBirchLog);
    register("minecraft:stripped_jungle_log", Block::StrippedJungleLog);
    register("minecraft:stripped_acacia_log", Block::StrippedAcaciaLog);
    register("minecraft:stripped_cherry_log", Block::StrippedCherryLog);
    register("minecraft:stripped_dark_oak_log", Block::StrippedDarkOakLog);
    register("minecraft:stripped_pale_oak_log", Block::StrippedPaleOakLog);
    register("minecraft:stripped_oak_log", Block::StrippedOakLog);
    register(
        "minecraft:stripped_mangrove_log",
        Block::StrippedMangroveLog,
    );
    register(
        "minecraft:stripped_bamboo_block",
        Block::StrippedBambooBlock,
    );
    register("minecraft:oak_wood", Block::OakWood);
    register("minecraft:spruce_wood", Block::SpruceWood);
    register("minecraft:birch_wood", Block::BirchWood);
    register("minecraft:jungle_wood", Block::JungleWood);
    register("minecraft:acacia_wood", Block::AcaciaWood);
    register("minecraft:cherry_wood", Block::CherryWood);
    register("minecraft:dark_oak_wood", Block::DarkOakWood);
    register("minecraft:mangrove_wood", Block::MangroveWood);
    register("minecraft:stripped_oak_wood", Block::StrippedOakWood);
    register("minecraft:stripped_spruce_wood", Block::StrippedSpruceWood);
    register("minecraft:stripped_birch_wood", Block::StrippedBirchWood);
    register("minecraft:stripped_jungle_wood", Block::StrippedJungleWood);
    register("minecraft:stripped_acacia_wood", Block::StrippedAcaciaWood);
    register("minecraft:stripped_cherry_wood", Block::StrippedCherryWood);
    register(
        "minecraft:stripped_dark_oak_wood",
        Block::StrippedDarkOakWood,
    );
    register(
        "minecraft:stripped_pale_oak_wood",
        Block::StrippedPaleOakWood,
    );
    register(
        "minecraft:stripped_mangrove_wood",
        Block::StrippedMangroveWood,
    );
    register("minecraft:oak_leaves", Block::OakLeaves);
    register("minecraft:spruce_leaves", Block::SpruceLeaves);
    register("minecraft:birch_leaves", Block::BirchLeaves);
    register("minecraft:jungle_leaves", Block::JungleLeaves);
    register("minecraft:acacia_leaves", Block::AcaciaLeaves);
    register("minecraft:cherry_leaves", Block::CherryLeaves);
    register("minecraft:dark_oak_leaves", Block::DarkOakLeaves);
    register("minecraft:pale_oak_leaves", Block::PaleOakLeaves);
    register("minecraft:mangrove_leaves", Block::MangroveLeaves);
    register("minecraft:azalea_leaves", Block::AzaleaLeaves);
    register(
        "minecraft:flowering_azalea_leaves",
        Block::FloweringAzaleaLeaves,
    );
    register("minecraft:sponge", Block::Sponge);
    register("minecraft:wet_sponge", Block::WetSponge);
    register("minecraft:glass", Block::Glass);
    register("minecraft:lapis_ore", Block::LapisOre);
    register("minecraft:deepslate_lapis_ore", Block::DeepslateLapisOre);
    register("minecraft:lapis_block", Block::LapisBlock);
    register("minecraft:dispenser", Block::Dispenser);
    register("minecraft:sandstone", Block::Sandstone);
    register("minecraft:chiseled_sandstone", Block::ChiseledSandstone);
    register("minecraft:cut_sandstone", Block::CutSandstone);
    register("minecraft:note_block", Block::NoteBlock);
    register("minecraft:white_bed", Block::WhiteBed);
    register("minecraft:orange_bed", Block::OrangeBed);
    register("minecraft:magenta_bed", Block::MagentaBed);
    register("minecraft:light_blue_bed", Block::LightBlueBed);
    register("minecraft:yellow_bed", Block::YellowBed);
    register("minecraft:lime_bed", Block::LimeBed);
    register("minecraft:pink_bed", Block::PinkBed);
    register("minecraft:gray_bed", Block::GrayBed);
    register("minecraft:light_gray_bed", Block::LightGrayBed);
    register("minecraft:cyan_bed", Block::CyanBed);
    register("minecraft:purple_bed", Block::PurpleBed);
    register("minecraft:blue_bed", Block::BlueBed);
    register("minecraft:brown_bed", Block::BrownBed);
    register("minecraft:green_bed", Block::GreenBed);
    register("minecraft:red_bed", Block::RedBed);
    register("minecraft:black_bed", Block::BlackBed);
    register("minecraft:powered_rail", Block::PoweredRail);
    register("minecraft:detector_rail", Block::DetectorRail);
    register("minecraft:sticky_piston", Block::StickyPiston);
    register("minecraft:cobweb", Block::Cobweb);
    register("minecraft:short_grass", Block::ShortGrass);
    register("minecraft:fern", Block::Fern);
    register("minecraft:dead_bush", Block::DeadBush);
    register("minecraft:bush", Block::Bush);
    register("minecraft:short_dry_grass", Block::ShortDryGrass);
    register("minecraft:tall_dry_grass", Block::TallDryGrass);
    register("minecraft:seagrass", Block::Seagrass);
    register("minecraft:tall_seagrass", Block::TallSeagrass);
    register("minecraft:piston", Block::Piston);
    register("minecraft:piston_head", Block::PistonHead);
    register("minecraft:white_wool", Block::WhiteWool);
    register("minecraft:orange_wool", Block::OrangeWool);
    register("minecraft:magenta_wool", Block::MagentaWool);
    register("minecraft:light_blue_wool", Block::LightBlueWool);
    register("minecraft:yellow_wool", Block::YellowWool);
    register("minecraft:lime_wool", Block::LimeWool);
    register("minecraft:pink_wool", Block::PinkWool);
    register("minecraft:gray_wool", Block::GrayWool);
    register("minecraft:light_gray_wool", Block::LightGrayWool);
    register("minecraft:cyan_wool", Block::CyanWool);
    register("minecraft:purple_wool", Block::PurpleWool);
    register("minecraft:blue_wool", Block::BlueWool);
    register("minecraft:brown_wool", Block::BrownWool);
    register("minecraft:green_wool", Block::GreenWool);
    register("minecraft:red_wool", Block::RedWool);
    register("minecraft:black_wool", Block::BlackWool);
    register("minecraft:moving_piston", Block::MovingPiston);
    register("minecraft:dandelion", Block::Dandelion);
    register("minecraft:golden_dandelion", Block::GoldenDandelion);
    register("minecraft:torchflower", Block::Torchflower);
    register("minecraft:poppy", Block::Poppy);
    register("minecraft:blue_orchid", Block::BlueOrchid);
    register("minecraft:allium", Block::Allium);
    register("minecraft:azure_bluet", Block::AzureBluet);
    register("minecraft:red_tulip", Block::RedTulip);
    register("minecraft:orange_tulip", Block::OrangeTulip);
    register("minecraft:white_tulip", Block::WhiteTulip);
    register("minecraft:pink_tulip", Block::PinkTulip);
    register("minecraft:oxeye_daisy", Block::OxeyeDaisy);
    register("minecraft:cornflower", Block::Cornflower);
    register("minecraft:wither_rose", Block::WitherRose);
    register("minecraft:lily_of_the_valley", Block::LilyOfTheValley);
    register("minecraft:brown_mushroom", Block::BrownMushroom);
    register("minecraft:red_mushroom", Block::RedMushroom);
    register("minecraft:gold_block", Block::GoldBlock);
    register("minecraft:iron_block", Block::IronBlock);
    register("minecraft:bricks", Block::Bricks);
    register("minecraft:tnt", Block::Tnt);
    register("minecraft:bookshelf", Block::Bookshelf);
    register("minecraft:chiseled_bookshelf", Block::ChiseledBookshelf);
    register("minecraft:acacia_shelf", Block::AcaciaShelf);
    register("minecraft:bamboo_shelf", Block::BambooShelf);
    register("minecraft:birch_shelf", Block::BirchShelf);
    register("minecraft:cherry_shelf", Block::CherryShelf);
    register("minecraft:crimson_shelf", Block::CrimsonShelf);
    register("minecraft:dark_oak_shelf", Block::DarkOakShelf);
    register("minecraft:jungle_shelf", Block::JungleShelf);
    register("minecraft:mangrove_shelf", Block::MangroveShelf);
    register("minecraft:oak_shelf", Block::OakShelf);
    register("minecraft:pale_oak_shelf", Block::PaleOakShelf);
    register("minecraft:spruce_shelf", Block::SpruceShelf);
    register("minecraft:warped_shelf", Block::WarpedShelf);
    register("minecraft:mossy_cobblestone", Block::MossyCobblestone);
    register("minecraft:obsidian", Block::Obsidian);
    register("minecraft:torch", Block::Torch);
    register("minecraft:wall_torch", Block::WallTorch);
    register("minecraft:fire", Block::Fire);
    register("minecraft:soul_fire", Block::SoulFire);
    register("minecraft:spawner", Block::Spawner);
    register("minecraft:creaking_heart", Block::CreakingHeart);
    register("minecraft:oak_stairs", Block::OakStairs);
    register("minecraft:chest", Block::Chest);
    register("minecraft:redstone_wire", Block::RedstoneWire);
    register("minecraft:diamond_ore", Block::DiamondOre);
    register(
        "minecraft:deepslate_diamond_ore",
        Block::DeepslateDiamondOre,
    );
    register("minecraft:diamond_block", Block::DiamondBlock);
    register("minecraft:crafting_table", Block::CraftingTable);
    register("minecraft:wheat", Block::Wheat);
    register("minecraft:farmland", Block::Farmland);
    register("minecraft:furnace", Block::Furnace);
    register("minecraft:oak_sign", Block::OakSign);
    register("minecraft:spruce_sign", Block::SpruceSign);
    register("minecraft:birch_sign", Block::BirchSign);
    register("minecraft:acacia_sign", Block::AcaciaSign);
    register("minecraft:cherry_sign", Block::CherrySign);
    register("minecraft:jungle_sign", Block::JungleSign);
    register("minecraft:dark_oak_sign", Block::DarkOakSign);
    register("minecraft:pale_oak_sign", Block::PaleOakSign);
    register("minecraft:mangrove_sign", Block::MangroveSign);
    register("minecraft:bamboo_sign", Block::BambooSign);
    register("minecraft:oak_door", Block::OakDoor);
    register("minecraft:ladder", Block::Ladder);
    register("minecraft:rail", Block::Rail);
    register("minecraft:cobblestone_stairs", Block::CobblestoneStairs);
    register("minecraft:oak_wall_sign", Block::OakWallSign);
    register("minecraft:spruce_wall_sign", Block::SpruceWallSign);
    register("minecraft:birch_wall_sign", Block::BirchWallSign);
    register("minecraft:acacia_wall_sign", Block::AcaciaWallSign);
    register("minecraft:cherry_wall_sign", Block::CherryWallSign);
    register("minecraft:jungle_wall_sign", Block::JungleWallSign);
    register("minecraft:dark_oak_wall_sign", Block::DarkOakWallSign);
    register("minecraft:pale_oak_wall_sign", Block::PaleOakWallSign);
    register("minecraft:mangrove_wall_sign", Block::MangroveWallSign);
    register("minecraft:bamboo_wall_sign", Block::BambooWallSign);
    register("minecraft:oak_hanging_sign", Block::OakHangingSign);
    register("minecraft:spruce_hanging_sign", Block::SpruceHangingSign);
    register("minecraft:birch_hanging_sign", Block::BirchHangingSign);
    register("minecraft:acacia_hanging_sign", Block::AcaciaHangingSign);
    register("minecraft:cherry_hanging_sign", Block::CherryHangingSign);
    register("minecraft:jungle_hanging_sign", Block::JungleHangingSign);
    register("minecraft:dark_oak_hanging_sign", Block::DarkOakHangingSign);
    register("minecraft:pale_oak_hanging_sign", Block::PaleOakHangingSign);
    register("minecraft:crimson_hanging_sign", Block::CrimsonHangingSign);
    register("minecraft:warped_hanging_sign", Block::WarpedHangingSign);
    register(
        "minecraft:mangrove_hanging_sign",
        Block::MangroveHangingSign,
    );
    register("minecraft:bamboo_hanging_sign", Block::BambooHangingSign);
    register("minecraft:oak_wall_hanging_sign", Block::OakWallHangingSign);
    register(
        "minecraft:spruce_wall_hanging_sign",
        Block::SpruceWallHangingSign,
    );
    register(
        "minecraft:birch_wall_hanging_sign",
        Block::BirchWallHangingSign,
    );
    register(
        "minecraft:acacia_wall_hanging_sign",
        Block::AcaciaWallHangingSign,
    );
    register(
        "minecraft:cherry_wall_hanging_sign",
        Block::CherryWallHangingSign,
    );
    register(
        "minecraft:jungle_wall_hanging_sign",
        Block::JungleWallHangingSign,
    );
    register(
        "minecraft:dark_oak_wall_hanging_sign",
        Block::DarkOakWallHangingSign,
    );
    register(
        "minecraft:pale_oak_wall_hanging_sign",
        Block::PaleOakWallHangingSign,
    );
    register(
        "minecraft:mangrove_wall_hanging_sign",
        Block::MangroveWallHangingSign,
    );
    register(
        "minecraft:crimson_wall_hanging_sign",
        Block::CrimsonWallHangingSign,
    );
    register(
        "minecraft:warped_wall_hanging_sign",
        Block::WarpedWallHangingSign,
    );
    register(
        "minecraft:bamboo_wall_hanging_sign",
        Block::BambooWallHangingSign,
    );
    register("minecraft:lever", Block::Lever);
    register("minecraft:stone_pressure_plate", Block::StonePressurePlate);
    register("minecraft:iron_door", Block::IronDoor);
    register("minecraft:oak_pressure_plate", Block::OakPressurePlate);
    register(
        "minecraft:spruce_pressure_plate",
        Block::SprucePressurePlate,
    );
    register("minecraft:birch_pressure_plate", Block::BirchPressurePlate);
    register(
        "minecraft:jungle_pressure_plate",
        Block::JunglePressurePlate,
    );
    register(
        "minecraft:acacia_pressure_plate",
        Block::AcaciaPressurePlate,
    );
    register(
        "minecraft:cherry_pressure_plate",
        Block::CherryPressurePlate,
    );
    register(
        "minecraft:dark_oak_pressure_plate",
        Block::DarkOakPressurePlate,
    );
    register(
        "minecraft:pale_oak_pressure_plate",
        Block::PaleOakPressurePlate,
    );
    register(
        "minecraft:mangrove_pressure_plate",
        Block::MangrovePressurePlate,
    );
    register(
        "minecraft:bamboo_pressure_plate",
        Block::BambooPressurePlate,
    );
    register("minecraft:redstone_ore", Block::RedstoneOre);
    register(
        "minecraft:deepslate_redstone_ore",
        Block::DeepslateRedstoneOre,
    );
    register("minecraft:redstone_torch", Block::RedstoneTorch);
    register("minecraft:redstone_wall_torch", Block::RedstoneWallTorch);
    register("minecraft:stone_button", Block::StoneButton);
    register("minecraft:snow", Block::Snow);
    register("minecraft:ice", Block::Ice);
    register("minecraft:snow_block", Block::SnowBlock);
    register("minecraft:cactus", Block::Cactus);
    register("minecraft:cactus_flower", Block::CactusFlower);
    register("minecraft:clay", Block::Clay);
    register("minecraft:sugar_cane", Block::SugarCane);
    register("minecraft:jukebox", Block::Jukebox);
    register("minecraft:oak_fence", Block::OakFence);
    register("minecraft:netherrack", Block::Netherrack);
    register("minecraft:soul_sand", Block::SoulSand);
    register("minecraft:soul_soil", Block::SoulSoil);
    register("minecraft:basalt", Block::Basalt);
    register("minecraft:polished_basalt", Block::PolishedBasalt);
    register("minecraft:soul_torch", Block::SoulTorch);
    register("minecraft:soul_wall_torch", Block::SoulWallTorch);
    register("minecraft:copper_torch", Block::CopperTorch);
    register("minecraft:copper_wall_torch", Block::CopperWallTorch);
    register("minecraft:glowstone", Block::Glowstone);
    register("minecraft:nether_portal", Block::NetherPortal);
    register("minecraft:carved_pumpkin", Block::CarvedPumpkin);
    register("minecraft:jack_o_lantern", Block::JackOLantern);
    register("minecraft:cake", Block::Cake);
    register("minecraft:repeater", Block::Repeater);
    register("minecraft:white_stained_glass", Block::WhiteStainedGlass);
    register("minecraft:orange_stained_glass", Block::OrangeStainedGlass);
    register(
        "minecraft:magenta_stained_glass",
        Block::MagentaStainedGlass,
    );
    register(
        "minecraft:light_blue_stained_glass",
        Block::LightBlueStainedGlass,
    );
    register("minecraft:yellow_stained_glass", Block::YellowStainedGlass);
    register("minecraft:lime_stained_glass", Block::LimeStainedGlass);
    register("minecraft:pink_stained_glass", Block::PinkStainedGlass);
    register("minecraft:gray_stained_glass", Block::GrayStainedGlass);
    register(
        "minecraft:light_gray_stained_glass",
        Block::LightGrayStainedGlass,
    );
    register("minecraft:cyan_stained_glass", Block::CyanStainedGlass);
    register("minecraft:purple_stained_glass", Block::PurpleStainedGlass);
    register("minecraft:blue_stained_glass", Block::BlueStainedGlass);
    register("minecraft:brown_stained_glass", Block::BrownStainedGlass);
    register("minecraft:green_stained_glass", Block::GreenStainedGlass);
    register("minecraft:red_stained_glass", Block::RedStainedGlass);
    register("minecraft:black_stained_glass", Block::BlackStainedGlass);
    register("minecraft:oak_trapdoor", Block::OakTrapdoor);
    register("minecraft:spruce_trapdoor", Block::SpruceTrapdoor);
    register("minecraft:birch_trapdoor", Block::BirchTrapdoor);
    register("minecraft:jungle_trapdoor", Block::JungleTrapdoor);
    register("minecraft:acacia_trapdoor", Block::AcaciaTrapdoor);
    register("minecraft:cherry_trapdoor", Block::CherryTrapdoor);
    register("minecraft:dark_oak_trapdoor", Block::DarkOakTrapdoor);
    register("minecraft:pale_oak_trapdoor", Block::PaleOakTrapdoor);
    register("minecraft:mangrove_trapdoor", Block::MangroveTrapdoor);
    register("minecraft:bamboo_trapdoor", Block::BambooTrapdoor);
    register("minecraft:stone_bricks", Block::StoneBricks);
    register("minecraft:mossy_stone_bricks", Block::MossyStoneBricks);
    register("minecraft:cracked_stone_bricks", Block::CrackedStoneBricks);
    register(
        "minecraft:chiseled_stone_bricks",
        Block::ChiseledStoneBricks,
    );
    register("minecraft:packed_mud", Block::PackedMud);
    register("minecraft:mud_bricks", Block::MudBricks);
    register("minecraft:infested_stone", Block::InfestedStone);
    register("minecraft:infested_cobblestone", Block::InfestedCobblestone);
    register(
        "minecraft:infested_stone_bricks",
        Block::InfestedStoneBricks,
    );
    register(
        "minecraft:infested_mossy_stone_bricks",
        Block::InfestedMossyStoneBricks,
    );
    register(
        "minecraft:infested_cracked_stone_bricks",
        Block::InfestedCrackedStoneBricks,
    );
    register(
        "minecraft:infested_chiseled_stone_bricks",
        Block::InfestedChiseledStoneBricks,
    );
    register("minecraft:brown_mushroom_block", Block::BrownMushroomBlock);
    register("minecraft:red_mushroom_block", Block::RedMushroomBlock);
    register("minecraft:mushroom_stem", Block::MushroomStem);
    register("minecraft:iron_bars", Block::IronBars);
    register("minecraft:copper_bars", Block::CopperBars);
    register("minecraft:exposed_copper_bars", Block::ExposedCopperBars);
    register(
        "minecraft:weathered_copper_bars",
        Block::WeatheredCopperBars,
    );
    register("minecraft:oxidized_copper_bars", Block::OxidizedCopperBars);
    register("minecraft:waxed_copper_bars", Block::WaxedCopperBars);
    register(
        "minecraft:waxed_exposed_copper_bars",
        Block::WaxedExposedCopperBars,
    );
    register(
        "minecraft:waxed_weathered_copper_bars",
        Block::WaxedWeatheredCopperBars,
    );
    register(
        "minecraft:waxed_oxidized_copper_bars",
        Block::WaxedOxidizedCopperBars,
    );
    register("minecraft:iron_chain", Block::IronChain);
    register("minecraft:copper_chain", Block::CopperChain);
    register("minecraft:exposed_copper_chain", Block::ExposedCopperChain);
    register(
        "minecraft:weathered_copper_chain",
        Block::WeatheredCopperChain,
    );
    register(
        "minecraft:oxidized_copper_chain",
        Block::OxidizedCopperChain,
    );
    register("minecraft:waxed_copper_chain", Block::WaxedCopperChain);
    register(
        "minecraft:waxed_exposed_copper_chain",
        Block::WaxedExposedCopperChain,
    );
    register(
        "minecraft:waxed_weathered_copper_chain",
        Block::WaxedWeatheredCopperChain,
    );
    register(
        "minecraft:waxed_oxidized_copper_chain",
        Block::WaxedOxidizedCopperChain,
    );
    register("minecraft:glass_pane", Block::GlassPane);
    register("minecraft:pumpkin", Block::Pumpkin);
    register("minecraft:melon", Block::Melon);
    register(
        "minecraft:attached_pumpkin_stem",
        Block::AttachedPumpkinStem,
    );
    register("minecraft:attached_melon_stem", Block::AttachedMelonStem);
    register("minecraft:pumpkin_stem", Block::PumpkinStem);
    register("minecraft:melon_stem", Block::MelonStem);
    register("minecraft:vine", Block::Vine);
    register("minecraft:glow_lichen", Block::GlowLichen);
    register("minecraft:resin_clump", Block::ResinClump);
    register("minecraft:oak_fence_gate", Block::OakFenceGate);
    register("minecraft:brick_stairs", Block::BrickStairs);
    register("minecraft:stone_brick_stairs", Block::StoneBrickStairs);
    register("minecraft:mud_brick_stairs", Block::MudBrickStairs);
    register("minecraft:mycelium", Block::Mycelium);
    register("minecraft:lily_pad", Block::LilyPad);
    register("minecraft:resin_block", Block::ResinBlock);
    register("minecraft:resin_bricks", Block::ResinBricks);
    register("minecraft:resin_brick_stairs", Block::ResinBrickStairs);
    register("minecraft:resin_brick_slab", Block::ResinBrickSlab);
    register("minecraft:resin_brick_wall", Block::ResinBrickWall);
    register(
        "minecraft:chiseled_resin_bricks",
        Block::ChiseledResinBricks,
    );
    register("minecraft:nether_bricks", Block::NetherBricks);
    register("minecraft:nether_brick_fence", Block::NetherBrickFence);
    register("minecraft:nether_brick_stairs", Block::NetherBrickStairs);
    register("minecraft:nether_wart", Block::NetherWart);
    register("minecraft:enchanting_table", Block::EnchantingTable);
    register("minecraft:brewing_stand", Block::BrewingStand);
    register("minecraft:cauldron", Block::Cauldron);
    register("minecraft:water_cauldron", Block::WaterCauldron);
    register("minecraft:lava_cauldron", Block::LavaCauldron);
    register("minecraft:powder_snow_cauldron", Block::PowderSnowCauldron);
    register("minecraft:end_portal", Block::EndPortal);
    register("minecraft:end_portal_frame", Block::EndPortalFrame);
    register("minecraft:end_stone", Block::EndStone);
    register("minecraft:dragon_egg", Block::DragonEgg);
    register("minecraft:redstone_lamp", Block::RedstoneLamp);
    register("minecraft:cocoa", Block::Cocoa);
    register("minecraft:sandstone_stairs", Block::SandstoneStairs);
    register("minecraft:emerald_ore", Block::EmeraldOre);
    register(
        "minecraft:deepslate_emerald_ore",
        Block::DeepslateEmeraldOre,
    );
    register("minecraft:ender_chest", Block::EnderChest);
    register("minecraft:tripwire_hook", Block::TripwireHook);
    register("minecraft:tripwire", Block::Tripwire);
    register("minecraft:emerald_block", Block::EmeraldBlock);
    register("minecraft:spruce_stairs", Block::SpruceStairs);
    register("minecraft:birch_stairs", Block::BirchStairs);
    register("minecraft:jungle_stairs", Block::JungleStairs);
    register("minecraft:command_block", Block::CommandBlock);
    register("minecraft:beacon", Block::Beacon);
    register("minecraft:cobblestone_wall", Block::CobblestoneWall);
    register(
        "minecraft:mossy_cobblestone_wall",
        Block::MossyCobblestoneWall,
    );
    register("minecraft:flower_pot", Block::FlowerPot);
    register("minecraft:potted_torchflower", Block::PottedTorchflower);
    register("minecraft:potted_oak_sapling", Block::PottedOakSapling);
    register(
        "minecraft:potted_spruce_sapling",
        Block::PottedSpruceSapling,
    );
    register("minecraft:potted_birch_sapling", Block::PottedBirchSapling);
    register(
        "minecraft:potted_jungle_sapling",
        Block::PottedJungleSapling,
    );
    register(
        "minecraft:potted_acacia_sapling",
        Block::PottedAcaciaSapling,
    );
    register(
        "minecraft:potted_cherry_sapling",
        Block::PottedCherrySapling,
    );
    register(
        "minecraft:potted_dark_oak_sapling",
        Block::PottedDarkOakSapling,
    );
    register(
        "minecraft:potted_pale_oak_sapling",
        Block::PottedPaleOakSapling,
    );
    register(
        "minecraft:potted_mangrove_propagule",
        Block::PottedMangrovePropagule,
    );
    register("minecraft:potted_fern", Block::PottedFern);
    register("minecraft:potted_dandelion", Block::PottedDandelion);
    register(
        "minecraft:potted_golden_dandelion",
        Block::PottedGoldenDandelion,
    );
    register("minecraft:potted_poppy", Block::PottedPoppy);
    register("minecraft:potted_blue_orchid", Block::PottedBlueOrchid);
    register("minecraft:potted_allium", Block::PottedAllium);
    register("minecraft:potted_azure_bluet", Block::PottedAzureBluet);
    register("minecraft:potted_red_tulip", Block::PottedRedTulip);
    register("minecraft:potted_orange_tulip", Block::PottedOrangeTulip);
    register("minecraft:potted_white_tulip", Block::PottedWhiteTulip);
    register("minecraft:potted_pink_tulip", Block::PottedPinkTulip);
    register("minecraft:potted_oxeye_daisy", Block::PottedOxeyeDaisy);
    register("minecraft:potted_cornflower", Block::PottedCornflower);
    register(
        "minecraft:potted_lily_of_the_valley",
        Block::PottedLilyOfTheValley,
    );
    register("minecraft:potted_wither_rose", Block::PottedWitherRose);
    register("minecraft:potted_red_mushroom", Block::PottedRedMushroom);
    register(
        "minecraft:potted_brown_mushroom",
        Block::PottedBrownMushroom,
    );
    register("minecraft:potted_dead_bush", Block::PottedDeadBush);
    register("minecraft:potted_cactus", Block::PottedCactus);
    register("minecraft:carrots", Block::Carrots);
    register("minecraft:potatoes", Block::Potatoes);
    register("minecraft:oak_button", Block::OakButton);
    register("minecraft:spruce_button", Block::SpruceButton);
    register("minecraft:birch_button", Block::BirchButton);
    register("minecraft:jungle_button", Block::JungleButton);
    register("minecraft:acacia_button", Block::AcaciaButton);
    register("minecraft:cherry_button", Block::CherryButton);
    register("minecraft:dark_oak_button", Block::DarkOakButton);
    register("minecraft:pale_oak_button", Block::PaleOakButton);
    register("minecraft:mangrove_button", Block::MangroveButton);
    register("minecraft:bamboo_button", Block::BambooButton);
    register("minecraft:skeleton_skull", Block::SkeletonSkull);
    register("minecraft:skeleton_wall_skull", Block::SkeletonWallSkull);
    register(
        "minecraft:wither_skeleton_skull",
        Block::WitherSkeletonSkull,
    );
    register(
        "minecraft:wither_skeleton_wall_skull",
        Block::WitherSkeletonWallSkull,
    );
    register("minecraft:zombie_head", Block::ZombieHead);
    register("minecraft:zombie_wall_head", Block::ZombieWallHead);
    register("minecraft:player_head", Block::PlayerHead);
    register("minecraft:player_wall_head", Block::PlayerWallHead);
    register("minecraft:creeper_head", Block::CreeperHead);
    register("minecraft:creeper_wall_head", Block::CreeperWallHead);
    register("minecraft:dragon_head", Block::DragonHead);
    register("minecraft:dragon_wall_head", Block::DragonWallHead);
    register("minecraft:piglin_head", Block::PiglinHead);
    register("minecraft:piglin_wall_head", Block::PiglinWallHead);
    register("minecraft:anvil", Block::Anvil);
    register("minecraft:chipped_anvil", Block::ChippedAnvil);
    register("minecraft:damaged_anvil", Block::DamagedAnvil);
    register("minecraft:trapped_chest", Block::TrappedChest);
    register(
        "minecraft:light_weighted_pressure_plate",
        Block::LightWeightedPressurePlate,
    );
    register(
        "minecraft:heavy_weighted_pressure_plate",
        Block::HeavyWeightedPressurePlate,
    );
    register("minecraft:comparator", Block::Comparator);
    register("minecraft:daylight_detector", Block::DaylightDetector);
    register("minecraft:redstone_block", Block::RedstoneBlock);
    register("minecraft:nether_quartz_ore", Block::NetherQuartzOre);
    register("minecraft:hopper", Block::Hopper);
    register("minecraft:quartz_block", Block::QuartzBlock);
    register(
        "minecraft:chiseled_quartz_block",
        Block::ChiseledQuartzBlock,
    );
    register("minecraft:quartz_pillar", Block::QuartzPillar);
    register("minecraft:quartz_stairs", Block::QuartzStairs);
    register("minecraft:activator_rail", Block::ActivatorRail);
    register("minecraft:dropper", Block::Dropper);
    register("minecraft:white_terracotta", Block::WhiteTerracotta);
    register("minecraft:orange_terracotta", Block::OrangeTerracotta);
    register("minecraft:magenta_terracotta", Block::MagentaTerracotta);
    register(
        "minecraft:light_blue_terracotta",
        Block::LightBlueTerracotta,
    );
    register("minecraft:yellow_terracotta", Block::YellowTerracotta);
    register("minecraft:lime_terracotta", Block::LimeTerracotta);
    register("minecraft:pink_terracotta", Block::PinkTerracotta);
    register("minecraft:gray_terracotta", Block::GrayTerracotta);
    register(
        "minecraft:light_gray_terracotta",
        Block::LightGrayTerracotta,
    );
    register("minecraft:cyan_terracotta", Block::CyanTerracotta);
    register("minecraft:purple_terracotta", Block::PurpleTerracotta);
    register("minecraft:blue_terracotta", Block::BlueTerracotta);
    register("minecraft:brown_terracotta", Block::BrownTerracotta);
    register("minecraft:green_terracotta", Block::GreenTerracotta);
    register("minecraft:red_terracotta", Block::RedTerracotta);
    register("minecraft:black_terracotta", Block::BlackTerracotta);
    register(
        "minecraft:white_stained_glass_pane",
        Block::WhiteStainedGlassPane,
    );
    register(
        "minecraft:orange_stained_glass_pane",
        Block::OrangeStainedGlassPane,
    );
    register(
        "minecraft:magenta_stained_glass_pane",
        Block::MagentaStainedGlassPane,
    );
    register(
        "minecraft:light_blue_stained_glass_pane",
        Block::LightBlueStainedGlassPane,
    );
    register(
        "minecraft:yellow_stained_glass_pane",
        Block::YellowStainedGlassPane,
    );
    register(
        "minecraft:lime_stained_glass_pane",
        Block::LimeStainedGlassPane,
    );
    register(
        "minecraft:pink_stained_glass_pane",
        Block::PinkStainedGlassPane,
    );
    register(
        "minecraft:gray_stained_glass_pane",
        Block::GrayStainedGlassPane,
    );
    register(
        "minecraft:light_gray_stained_glass_pane",
        Block::LightGrayStainedGlassPane,
    );
    register(
        "minecraft:cyan_stained_glass_pane",
        Block::CyanStainedGlassPane,
    );
    register(
        "minecraft:purple_stained_glass_pane",
        Block::PurpleStainedGlassPane,
    );
    register(
        "minecraft:blue_stained_glass_pane",
        Block::BlueStainedGlassPane,
    );
    register(
        "minecraft:brown_stained_glass_pane",
        Block::BrownStainedGlassPane,
    );
    register(
        "minecraft:green_stained_glass_pane",
        Block::GreenStainedGlassPane,
    );
    register(
        "minecraft:red_stained_glass_pane",
        Block::RedStainedGlassPane,
    );
    register(
        "minecraft:black_stained_glass_pane",
        Block::BlackStainedGlassPane,
    );
    register("minecraft:acacia_stairs", Block::AcaciaStairs);
    register("minecraft:cherry_stairs", Block::CherryStairs);
    register("minecraft:dark_oak_stairs", Block::DarkOakStairs);
    register("minecraft:pale_oak_stairs", Block::PaleOakStairs);
    register("minecraft:mangrove_stairs", Block::MangroveStairs);
    register("minecraft:bamboo_stairs", Block::BambooStairs);
    register("minecraft:bamboo_mosaic_stairs", Block::BambooMosaicStairs);
    register("minecraft:slime_block", Block::SlimeBlock);
    register("minecraft:barrier", Block::Barrier);
    register("minecraft:light", Block::Light);
    register("minecraft:iron_trapdoor", Block::IronTrapdoor);
    register("minecraft:prismarine", Block::Prismarine);
    register("minecraft:prismarine_bricks", Block::PrismarineBricks);
    register("minecraft:dark_prismarine", Block::DarkPrismarine);
    register("minecraft:prismarine_stairs", Block::PrismarineStairs);
    register(
        "minecraft:prismarine_brick_stairs",
        Block::PrismarineBrickStairs,
    );
    register(
        "minecraft:dark_prismarine_stairs",
        Block::DarkPrismarineStairs,
    );
    register("minecraft:prismarine_slab", Block::PrismarineSlab);
    register(
        "minecraft:prismarine_brick_slab",
        Block::PrismarineBrickSlab,
    );
    register("minecraft:dark_prismarine_slab", Block::DarkPrismarineSlab);
    register("minecraft:sea_lantern", Block::SeaLantern);
    register("minecraft:hay_block", Block::HayBlock);
    register("minecraft:white_carpet", Block::WhiteCarpet);
    register("minecraft:orange_carpet", Block::OrangeCarpet);
    register("minecraft:magenta_carpet", Block::MagentaCarpet);
    register("minecraft:light_blue_carpet", Block::LightBlueCarpet);
    register("minecraft:yellow_carpet", Block::YellowCarpet);
    register("minecraft:lime_carpet", Block::LimeCarpet);
    register("minecraft:pink_carpet", Block::PinkCarpet);
    register("minecraft:gray_carpet", Block::GrayCarpet);
    register("minecraft:light_gray_carpet", Block::LightGrayCarpet);
    register("minecraft:cyan_carpet", Block::CyanCarpet);
    register("minecraft:purple_carpet", Block::PurpleCarpet);
    register("minecraft:blue_carpet", Block::BlueCarpet);
    register("minecraft:brown_carpet", Block::BrownCarpet);
    register("minecraft:green_carpet", Block::GreenCarpet);
    register("minecraft:red_carpet", Block::RedCarpet);
    register("minecraft:black_carpet", Block::BlackCarpet);
    register("minecraft:terracotta", Block::Terracotta);
    register("minecraft:coal_block", Block::CoalBlock);
    register("minecraft:packed_ice", Block::PackedIce);
    register("minecraft:sunflower", Block::Sunflower);
    register("minecraft:lilac", Block::Lilac);
    register("minecraft:rose_bush", Block::RoseBush);
    register("minecraft:peony", Block::Peony);
    register("minecraft:tall_grass", Block::TallGrass);
    register("minecraft:large_fern", Block::LargeFern);
    register("minecraft:white_banner", Block::WhiteBanner);
    register("minecraft:orange_banner", Block::OrangeBanner);
    register("minecraft:magenta_banner", Block::MagentaBanner);
    register("minecraft:light_blue_banner", Block::LightBlueBanner);
    register("minecraft:yellow_banner", Block::YellowBanner);
    register("minecraft:lime_banner", Block::LimeBanner);
    register("minecraft:pink_banner", Block::PinkBanner);
    register("minecraft:gray_banner", Block::GrayBanner);
    register("minecraft:light_gray_banner", Block::LightGrayBanner);
    register("minecraft:cyan_banner", Block::CyanBanner);
    register("minecraft:purple_banner", Block::PurpleBanner);
    register("minecraft:blue_banner", Block::BlueBanner);
    register("minecraft:brown_banner", Block::BrownBanner);
    register("minecraft:green_banner", Block::GreenBanner);
    register("minecraft:red_banner", Block::RedBanner);
    register("minecraft:black_banner", Block::BlackBanner);
    register("minecraft:white_wall_banner", Block::WhiteWallBanner);
    register("minecraft:orange_wall_banner", Block::OrangeWallBanner);
    register("minecraft:magenta_wall_banner", Block::MagentaWallBanner);
    register(
        "minecraft:light_blue_wall_banner",
        Block::LightBlueWallBanner,
    );
    register("minecraft:yellow_wall_banner", Block::YellowWallBanner);
    register("minecraft:lime_wall_banner", Block::LimeWallBanner);
    register("minecraft:pink_wall_banner", Block::PinkWallBanner);
    register("minecraft:gray_wall_banner", Block::GrayWallBanner);
    register(
        "minecraft:light_gray_wall_banner",
        Block::LightGrayWallBanner,
    );
    register("minecraft:cyan_wall_banner", Block::CyanWallBanner);
    register("minecraft:purple_wall_banner", Block::PurpleWallBanner);
    register("minecraft:blue_wall_banner", Block::BlueWallBanner);
    register("minecraft:brown_wall_banner", Block::BrownWallBanner);
    register("minecraft:green_wall_banner", Block::GreenWallBanner);
    register("minecraft:red_wall_banner", Block::RedWallBanner);
    register("minecraft:black_wall_banner", Block::BlackWallBanner);
    register("minecraft:red_sandstone", Block::RedSandstone);
    register(
        "minecraft:chiseled_red_sandstone",
        Block::ChiseledRedSandstone,
    );
    register("minecraft:cut_red_sandstone", Block::CutRedSandstone);
    register("minecraft:red_sandstone_stairs", Block::RedSandstoneStairs);
    register("minecraft:oak_slab", Block::OakSlab);
    register("minecraft:spruce_slab", Block::SpruceSlab);
    register("minecraft:birch_slab", Block::BirchSlab);
    register("minecraft:jungle_slab", Block::JungleSlab);
    register("minecraft:acacia_slab", Block::AcaciaSlab);
    register("minecraft:cherry_slab", Block::CherrySlab);
    register("minecraft:dark_oak_slab", Block::DarkOakSlab);
    register("minecraft:pale_oak_slab", Block::PaleOakSlab);
    register("minecraft:mangrove_slab", Block::MangroveSlab);
    register("minecraft:bamboo_slab", Block::BambooSlab);
    register("minecraft:bamboo_mosaic_slab", Block::BambooMosaicSlab);
    register("minecraft:stone_slab", Block::StoneSlab);
    register("minecraft:smooth_stone_slab", Block::SmoothStoneSlab);
    register("minecraft:sandstone_slab", Block::SandstoneSlab);
    register("minecraft:cut_sandstone_slab", Block::CutSandstoneSlab);
    register("minecraft:petrified_oak_slab", Block::PetrifiedOakSlab);
    register("minecraft:cobblestone_slab", Block::CobblestoneSlab);
    register("minecraft:brick_slab", Block::BrickSlab);
    register("minecraft:stone_brick_slab", Block::StoneBrickSlab);
    register("minecraft:mud_brick_slab", Block::MudBrickSlab);
    register("minecraft:nether_brick_slab", Block::NetherBrickSlab);
    register("minecraft:quartz_slab", Block::QuartzSlab);
    register("minecraft:red_sandstone_slab", Block::RedSandstoneSlab);
    register(
        "minecraft:cut_red_sandstone_slab",
        Block::CutRedSandstoneSlab,
    );
    register("minecraft:purpur_slab", Block::PurpurSlab);
    register("minecraft:smooth_stone", Block::SmoothStone);
    register("minecraft:smooth_sandstone", Block::SmoothSandstone);
    register("minecraft:smooth_quartz", Block::SmoothQuartz);
    register("minecraft:smooth_red_sandstone", Block::SmoothRedSandstone);
    register("minecraft:spruce_fence_gate", Block::SpruceFenceGate);
    register("minecraft:birch_fence_gate", Block::BirchFenceGate);
    register("minecraft:jungle_fence_gate", Block::JungleFenceGate);
    register("minecraft:acacia_fence_gate", Block::AcaciaFenceGate);
    register("minecraft:cherry_fence_gate", Block::CherryFenceGate);
    register("minecraft:dark_oak_fence_gate", Block::DarkOakFenceGate);
    register("minecraft:pale_oak_fence_gate", Block::PaleOakFenceGate);
    register("minecraft:mangrove_fence_gate", Block::MangroveFenceGate);
    register("minecraft:bamboo_fence_gate", Block::BambooFenceGate);
    register("minecraft:spruce_fence", Block::SpruceFence);
    register("minecraft:birch_fence", Block::BirchFence);
    register("minecraft:jungle_fence", Block::JungleFence);
    register("minecraft:acacia_fence", Block::AcaciaFence);
    register("minecraft:cherry_fence", Block::CherryFence);
    register("minecraft:dark_oak_fence", Block::DarkOakFence);
    register("minecraft:pale_oak_fence", Block::PaleOakFence);
    register("minecraft:mangrove_fence", Block::MangroveFence);
    register("minecraft:bamboo_fence", Block::BambooFence);
    register("minecraft:spruce_door", Block::SpruceDoor);
    register("minecraft:birch_door", Block::BirchDoor);
    register("minecraft:jungle_door", Block::JungleDoor);
    register("minecraft:acacia_door", Block::AcaciaDoor);
    register("minecraft:cherry_door", Block::CherryDoor);
    register("minecraft:dark_oak_door", Block::DarkOakDoor);
    register("minecraft:pale_oak_door", Block::PaleOakDoor);
    register("minecraft:mangrove_door", Block::MangroveDoor);
    register("minecraft:bamboo_door", Block::BambooDoor);
    register("minecraft:end_rod", Block::EndRod);
    register("minecraft:chorus_plant", Block::ChorusPlant);
    register("minecraft:chorus_flower", Block::ChorusFlower);
    register("minecraft:purpur_block", Block::PurpurBlock);
    register("minecraft:purpur_pillar", Block::PurpurPillar);
    register("minecraft:purpur_stairs", Block::PurpurStairs);
    register("minecraft:end_stone_bricks", Block::EndStoneBricks);
    register("minecraft:torchflower_crop", Block::TorchflowerCrop);
    register("minecraft:pitcher_crop", Block::PitcherCrop);
    register("minecraft:pitcher_plant", Block::PitcherPlant);
    register("minecraft:beetroots", Block::Beetroots);
    register("minecraft:dirt_path", Block::DirtPath);
    register("minecraft:end_gateway", Block::EndGateway);
    register(
        "minecraft:repeating_command_block",
        Block::RepeatingCommandBlock,
    );
    register("minecraft:chain_command_block", Block::ChainCommandBlock);
    register("minecraft:frosted_ice", Block::FrostedIce);
    register("minecraft:magma_block", Block::MagmaBlock);
    register("minecraft:nether_wart_block", Block::NetherWartBlock);
    register("minecraft:red_nether_bricks", Block::RedNetherBricks);
    register("minecraft:bone_block", Block::BoneBlock);
    register("minecraft:structure_void", Block::StructureVoid);
    register("minecraft:observer", Block::Observer);
    register("minecraft:shulker_box", Block::ShulkerBox);
    register("minecraft:white_shulker_box", Block::WhiteShulkerBox);
    register("minecraft:orange_shulker_box", Block::OrangeShulkerBox);
    register("minecraft:magenta_shulker_box", Block::MagentaShulkerBox);
    register(
        "minecraft:light_blue_shulker_box",
        Block::LightBlueShulkerBox,
    );
    register("minecraft:yellow_shulker_box", Block::YellowShulkerBox);
    register("minecraft:lime_shulker_box", Block::LimeShulkerBox);
    register("minecraft:pink_shulker_box", Block::PinkShulkerBox);
    register("minecraft:gray_shulker_box", Block::GrayShulkerBox);
    register(
        "minecraft:light_gray_shulker_box",
        Block::LightGrayShulkerBox,
    );
    register("minecraft:cyan_shulker_box", Block::CyanShulkerBox);
    register("minecraft:purple_shulker_box", Block::PurpleShulkerBox);
    register("minecraft:blue_shulker_box", Block::BlueShulkerBox);
    register("minecraft:brown_shulker_box", Block::BrownShulkerBox);
    register("minecraft:green_shulker_box", Block::GreenShulkerBox);
    register("minecraft:red_shulker_box", Block::RedShulkerBox);
    register("minecraft:black_shulker_box", Block::BlackShulkerBox);
    register(
        "minecraft:white_glazed_terracotta",
        Block::WhiteGlazedTerracotta,
    );
    register(
        "minecraft:orange_glazed_terracotta",
        Block::OrangeGlazedTerracotta,
    );
    register(
        "minecraft:magenta_glazed_terracotta",
        Block::MagentaGlazedTerracotta,
    );
    register(
        "minecraft:light_blue_glazed_terracotta",
        Block::LightBlueGlazedTerracotta,
    );
    register(
        "minecraft:yellow_glazed_terracotta",
        Block::YellowGlazedTerracotta,
    );
    register(
        "minecraft:lime_glazed_terracotta",
        Block::LimeGlazedTerracotta,
    );
    register(
        "minecraft:pink_glazed_terracotta",
        Block::PinkGlazedTerracotta,
    );
    register(
        "minecraft:gray_glazed_terracotta",
        Block::GrayGlazedTerracotta,
    );
    register(
        "minecraft:light_gray_glazed_terracotta",
        Block::LightGrayGlazedTerracotta,
    );
    register(
        "minecraft:cyan_glazed_terracotta",
        Block::CyanGlazedTerracotta,
    );
    register(
        "minecraft:purple_glazed_terracotta",
        Block::PurpleGlazedTerracotta,
    );
    register(
        "minecraft:blue_glazed_terracotta",
        Block::BlueGlazedTerracotta,
    );
    register(
        "minecraft:brown_glazed_terracotta",
        Block::BrownGlazedTerracotta,
    );
    register(
        "minecraft:green_glazed_terracotta",
        Block::GreenGlazedTerracotta,
    );
    register(
        "minecraft:red_glazed_terracotta",
        Block::RedGlazedTerracotta,
    );
    register(
        "minecraft:black_glazed_terracotta",
        Block::BlackGlazedTerracotta,
    );
    register("minecraft:white_concrete", Block::WhiteConcrete);
    register("minecraft:orange_concrete", Block::OrangeConcrete);
    register("minecraft:magenta_concrete", Block::MagentaConcrete);
    register("minecraft:light_blue_concrete", Block::LightBlueConcrete);
    register("minecraft:yellow_concrete", Block::YellowConcrete);
    register("minecraft:lime_concrete", Block::LimeConcrete);
    register("minecraft:pink_concrete", Block::PinkConcrete);
    register("minecraft:gray_concrete", Block::GrayConcrete);
    register("minecraft:light_gray_concrete", Block::LightGrayConcrete);
    register("minecraft:cyan_concrete", Block::CyanConcrete);
    register("minecraft:purple_concrete", Block::PurpleConcrete);
    register("minecraft:blue_concrete", Block::BlueConcrete);
    register("minecraft:brown_concrete", Block::BrownConcrete);
    register("minecraft:green_concrete", Block::GreenConcrete);
    register("minecraft:red_concrete", Block::RedConcrete);
    register("minecraft:black_concrete", Block::BlackConcrete);
    register(
        "minecraft:white_concrete_powder",
        Block::WhiteConcretePowder,
    );
    register(
        "minecraft:orange_concrete_powder",
        Block::OrangeConcretePowder,
    );
    register(
        "minecraft:magenta_concrete_powder",
        Block::MagentaConcretePowder,
    );
    register(
        "minecraft:light_blue_concrete_powder",
        Block::LightBlueConcretePowder,
    );
    register(
        "minecraft:yellow_concrete_powder",
        Block::YellowConcretePowder,
    );
    register("minecraft:lime_concrete_powder", Block::LimeConcretePowder);
    register("minecraft:pink_concrete_powder", Block::PinkConcretePowder);
    register("minecraft:gray_concrete_powder", Block::GrayConcretePowder);
    register(
        "minecraft:light_gray_concrete_powder",
        Block::LightGrayConcretePowder,
    );
    register("minecraft:cyan_concrete_powder", Block::CyanConcretePowder);
    register(
        "minecraft:purple_concrete_powder",
        Block::PurpleConcretePowder,
    );
    register("minecraft:blue_concrete_powder", Block::BlueConcretePowder);
    register(
        "minecraft:brown_concrete_powder",
        Block::BrownConcretePowder,
    );
    register(
        "minecraft:green_concrete_powder",
        Block::GreenConcretePowder,
    );
    register("minecraft:red_concrete_powder", Block::RedConcretePowder);
    register(
        "minecraft:black_concrete_powder",
        Block::BlackConcretePowder,
    );
    register("minecraft:kelp", Block::Kelp);
    register("minecraft:kelp_plant", Block::KelpPlant);
    register("minecraft:dried_kelp_block", Block::DriedKelpBlock);
    register("minecraft:turtle_egg", Block::TurtleEgg);
    register("minecraft:sniffer_egg", Block::SnifferEgg);
    register("minecraft:dried_ghast", Block::DriedGhast);
    register("minecraft:dead_tube_coral_block", Block::DeadTubeCoralBlock);
    register(
        "minecraft:dead_brain_coral_block",
        Block::DeadBrainCoralBlock,
    );
    register(
        "minecraft:dead_bubble_coral_block",
        Block::DeadBubbleCoralBlock,
    );
    register("minecraft:dead_fire_coral_block", Block::DeadFireCoralBlock);
    register("minecraft:dead_horn_coral_block", Block::DeadHornCoralBlock);
    register("minecraft:tube_coral_block", Block::TubeCoralBlock);
    register("minecraft:brain_coral_block", Block::BrainCoralBlock);
    register("minecraft:bubble_coral_block", Block::BubbleCoralBlock);
    register("minecraft:fire_coral_block", Block::FireCoralBlock);
    register("minecraft:horn_coral_block", Block::HornCoralBlock);
    register("minecraft:dead_tube_coral", Block::DeadTubeCoral);
    register("minecraft:dead_brain_coral", Block::DeadBrainCoral);
    register("minecraft:dead_bubble_coral", Block::DeadBubbleCoral);
    register("minecraft:dead_fire_coral", Block::DeadFireCoral);
    register("minecraft:dead_horn_coral", Block::DeadHornCoral);
    register("minecraft:tube_coral", Block::TubeCoral);
    register("minecraft:brain_coral", Block::BrainCoral);
    register("minecraft:bubble_coral", Block::BubbleCoral);
    register("minecraft:fire_coral", Block::FireCoral);
    register("minecraft:horn_coral", Block::HornCoral);
    register("minecraft:dead_tube_coral_fan", Block::DeadTubeCoralFan);
    register("minecraft:dead_brain_coral_fan", Block::DeadBrainCoralFan);
    register("minecraft:dead_bubble_coral_fan", Block::DeadBubbleCoralFan);
    register("minecraft:dead_fire_coral_fan", Block::DeadFireCoralFan);
    register("minecraft:dead_horn_coral_fan", Block::DeadHornCoralFan);
    register("minecraft:tube_coral_fan", Block::TubeCoralFan);
    register("minecraft:brain_coral_fan", Block::BrainCoralFan);
    register("minecraft:bubble_coral_fan", Block::BubbleCoralFan);
    register("minecraft:fire_coral_fan", Block::FireCoralFan);
    register("minecraft:horn_coral_fan", Block::HornCoralFan);
    register(
        "minecraft:dead_tube_coral_wall_fan",
        Block::DeadTubeCoralWallFan,
    );
    register(
        "minecraft:dead_brain_coral_wall_fan",
        Block::DeadBrainCoralWallFan,
    );
    register(
        "minecraft:dead_bubble_coral_wall_fan",
        Block::DeadBubbleCoralWallFan,
    );
    register(
        "minecraft:dead_fire_coral_wall_fan",
        Block::DeadFireCoralWallFan,
    );
    register(
        "minecraft:dead_horn_coral_wall_fan",
        Block::DeadHornCoralWallFan,
    );
    register("minecraft:tube_coral_wall_fan", Block::TubeCoralWallFan);
    register("minecraft:brain_coral_wall_fan", Block::BrainCoralWallFan);
    register("minecraft:bubble_coral_wall_fan", Block::BubbleCoralWallFan);
    register("minecraft:fire_coral_wall_fan", Block::FireCoralWallFan);
    register("minecraft:horn_coral_wall_fan", Block::HornCoralWallFan);
    register("minecraft:sea_pickle", Block::SeaPickle);
    register("minecraft:blue_ice", Block::BlueIce);
    register("minecraft:conduit", Block::Conduit);
    register("minecraft:bamboo_sapling", Block::BambooSapling);
    register("minecraft:bamboo", Block::Bamboo);
    register("minecraft:potted_bamboo", Block::PottedBamboo);
    register("minecraft:void_air", Block::VoidAir);
    register("minecraft:cave_air", Block::CaveAir);
    register("minecraft:bubble_column", Block::BubbleColumn);
    register(
        "minecraft:polished_granite_stairs",
        Block::PolishedGraniteStairs,
    );
    register(
        "minecraft:smooth_red_sandstone_stairs",
        Block::SmoothRedSandstoneStairs,
    );
    register(
        "minecraft:mossy_stone_brick_stairs",
        Block::MossyStoneBrickStairs,
    );
    register(
        "minecraft:polished_diorite_stairs",
        Block::PolishedDioriteStairs,
    );
    register(
        "minecraft:mossy_cobblestone_stairs",
        Block::MossyCobblestoneStairs,
    );
    register(
        "minecraft:end_stone_brick_stairs",
        Block::EndStoneBrickStairs,
    );
    register("minecraft:stone_stairs", Block::StoneStairs);
    register(
        "minecraft:smooth_sandstone_stairs",
        Block::SmoothSandstoneStairs,
    );
    register("minecraft:smooth_quartz_stairs", Block::SmoothQuartzStairs);
    register("minecraft:granite_stairs", Block::GraniteStairs);
    register("minecraft:andesite_stairs", Block::AndesiteStairs);
    register(
        "minecraft:red_nether_brick_stairs",
        Block::RedNetherBrickStairs,
    );
    register(
        "minecraft:polished_andesite_stairs",
        Block::PolishedAndesiteStairs,
    );
    register("minecraft:diorite_stairs", Block::DioriteStairs);
    register(
        "minecraft:polished_granite_slab",
        Block::PolishedGraniteSlab,
    );
    register(
        "minecraft:smooth_red_sandstone_slab",
        Block::SmoothRedSandstoneSlab,
    );
    register(
        "minecraft:mossy_stone_brick_slab",
        Block::MossyStoneBrickSlab,
    );
    register(
        "minecraft:polished_diorite_slab",
        Block::PolishedDioriteSlab,
    );
    register(
        "minecraft:mossy_cobblestone_slab",
        Block::MossyCobblestoneSlab,
    );
    register("minecraft:end_stone_brick_slab", Block::EndStoneBrickSlab);
    register(
        "minecraft:smooth_sandstone_slab",
        Block::SmoothSandstoneSlab,
    );
    register("minecraft:smooth_quartz_slab", Block::SmoothQuartzSlab);
    register("minecraft:granite_slab", Block::GraniteSlab);
    register("minecraft:andesite_slab", Block::AndesiteSlab);
    register("minecraft:red_nether_brick_slab", Block::RedNetherBrickSlab);
    register(
        "minecraft:polished_andesite_slab",
        Block::PolishedAndesiteSlab,
    );
    register("minecraft:diorite_slab", Block::DioriteSlab);
    register("minecraft:brick_wall", Block::BrickWall);
    register("minecraft:prismarine_wall", Block::PrismarineWall);
    register("minecraft:red_sandstone_wall", Block::RedSandstoneWall);
    register(
        "minecraft:mossy_stone_brick_wall",
        Block::MossyStoneBrickWall,
    );
    register("minecraft:granite_wall", Block::GraniteWall);
    register("minecraft:stone_brick_wall", Block::StoneBrickWall);
    register("minecraft:mud_brick_wall", Block::MudBrickWall);
    register("minecraft:nether_brick_wall", Block::NetherBrickWall);
    register("minecraft:andesite_wall", Block::AndesiteWall);
    register("minecraft:red_nether_brick_wall", Block::RedNetherBrickWall);
    register("minecraft:sandstone_wall", Block::SandstoneWall);
    register("minecraft:end_stone_brick_wall", Block::EndStoneBrickWall);
    register("minecraft:diorite_wall", Block::DioriteWall);
    register("minecraft:scaffolding", Block::Scaffolding);
    register("minecraft:loom", Block::Loom);
    register("minecraft:barrel", Block::Barrel);
    register("minecraft:smoker", Block::Smoker);
    register("minecraft:blast_furnace", Block::BlastFurnace);
    register("minecraft:cartography_table", Block::CartographyTable);
    register("minecraft:fletching_table", Block::FletchingTable);
    register("minecraft:grindstone", Block::Grindstone);
    register("minecraft:lectern", Block::Lectern);
    register("minecraft:smithing_table", Block::SmithingTable);
    register("minecraft:stonecutter", Block::Stonecutter);
    register("minecraft:bell", Block::Bell);
    register("minecraft:lantern", Block::Lantern);
    register("minecraft:soul_lantern", Block::SoulLantern);
    register("minecraft:copper_lantern", Block::CopperLantern);
    register(
        "minecraft:exposed_copper_lantern",
        Block::ExposedCopperLantern,
    );
    register(
        "minecraft:weathered_copper_lantern",
        Block::WeatheredCopperLantern,
    );
    register(
        "minecraft:oxidized_copper_lantern",
        Block::OxidizedCopperLantern,
    );
    register("minecraft:waxed_copper_lantern", Block::WaxedCopperLantern);
    register(
        "minecraft:waxed_exposed_copper_lantern",
        Block::WaxedExposedCopperLantern,
    );
    register(
        "minecraft:waxed_weathered_copper_lantern",
        Block::WaxedWeatheredCopperLantern,
    );
    register(
        "minecraft:waxed_oxidized_copper_lantern",
        Block::WaxedOxidizedCopperLantern,
    );
    register("minecraft:campfire", Block::Campfire);
    register("minecraft:soul_campfire", Block::SoulCampfire);
    register("minecraft:sweet_berry_bush", Block::SweetBerryBush);
    register("minecraft:warped_stem", Block::WarpedStem);
    register("minecraft:stripped_warped_stem", Block::StrippedWarpedStem);
    register("minecraft:warped_hyphae", Block::WarpedHyphae);
    register(
        "minecraft:stripped_warped_hyphae",
        Block::StrippedWarpedHyphae,
    );
    register("minecraft:warped_nylium", Block::WarpedNylium);
    register("minecraft:warped_fungus", Block::WarpedFungus);
    register("minecraft:warped_wart_block", Block::WarpedWartBlock);
    register("minecraft:warped_roots", Block::WarpedRoots);
    register("minecraft:nether_sprouts", Block::NetherSprouts);
    register("minecraft:crimson_stem", Block::CrimsonStem);
    register(
        "minecraft:stripped_crimson_stem",
        Block::StrippedCrimsonStem,
    );
    register("minecraft:crimson_hyphae", Block::CrimsonHyphae);
    register(
        "minecraft:stripped_crimson_hyphae",
        Block::StrippedCrimsonHyphae,
    );
    register("minecraft:crimson_nylium", Block::CrimsonNylium);
    register("minecraft:crimson_fungus", Block::CrimsonFungus);
    register("minecraft:shroomlight", Block::Shroomlight);
    register("minecraft:weeping_vines", Block::WeepingVines);
    register("minecraft:weeping_vines_plant", Block::WeepingVinesPlant);
    register("minecraft:twisting_vines", Block::TwistingVines);
    register("minecraft:twisting_vines_plant", Block::TwistingVinesPlant);
    register("minecraft:crimson_roots", Block::CrimsonRoots);
    register("minecraft:crimson_planks", Block::CrimsonPlanks);
    register("minecraft:warped_planks", Block::WarpedPlanks);
    register("minecraft:crimson_slab", Block::CrimsonSlab);
    register("minecraft:warped_slab", Block::WarpedSlab);
    register(
        "minecraft:crimson_pressure_plate",
        Block::CrimsonPressurePlate,
    );
    register(
        "minecraft:warped_pressure_plate",
        Block::WarpedPressurePlate,
    );
    register("minecraft:crimson_fence", Block::CrimsonFence);
    register("minecraft:warped_fence", Block::WarpedFence);
    register("minecraft:crimson_trapdoor", Block::CrimsonTrapdoor);
    register("minecraft:warped_trapdoor", Block::WarpedTrapdoor);
    register("minecraft:crimson_fence_gate", Block::CrimsonFenceGate);
    register("minecraft:warped_fence_gate", Block::WarpedFenceGate);
    register("minecraft:crimson_stairs", Block::CrimsonStairs);
    register("minecraft:warped_stairs", Block::WarpedStairs);
    register("minecraft:crimson_button", Block::CrimsonButton);
    register("minecraft:warped_button", Block::WarpedButton);
    register("minecraft:crimson_door", Block::CrimsonDoor);
    register("minecraft:warped_door", Block::WarpedDoor);
    register("minecraft:crimson_sign", Block::CrimsonSign);
    register("minecraft:warped_sign", Block::WarpedSign);
    register("minecraft:crimson_wall_sign", Block::CrimsonWallSign);
    register("minecraft:warped_wall_sign", Block::WarpedWallSign);
    register("minecraft:structure_block", Block::StructureBlock);
    register("minecraft:jigsaw", Block::Jigsaw);
    register("minecraft:test_block", Block::TestBlock);
    register("minecraft:test_instance_block", Block::TestInstanceBlock);
    register("minecraft:composter", Block::Composter);
    register("minecraft:target", Block::Target);
    register("minecraft:bee_nest", Block::BeeNest);
    register("minecraft:beehive", Block::Beehive);
    register("minecraft:honey_block", Block::HoneyBlock);
    register("minecraft:honeycomb_block", Block::HoneycombBlock);
    register("minecraft:netherite_block", Block::NetheriteBlock);
    register("minecraft:ancient_debris", Block::AncientDebris);
    register("minecraft:crying_obsidian", Block::CryingObsidian);
    register("minecraft:respawn_anchor", Block::RespawnAnchor);
    register(
        "minecraft:potted_crimson_fungus",
        Block::PottedCrimsonFungus,
    );
    register("minecraft:potted_warped_fungus", Block::PottedWarpedFungus);
    register("minecraft:potted_crimson_roots", Block::PottedCrimsonRoots);
    register("minecraft:potted_warped_roots", Block::PottedWarpedRoots);
    register("minecraft:lodestone", Block::Lodestone);
    register("minecraft:blackstone", Block::Blackstone);
    register("minecraft:blackstone_stairs", Block::BlackstoneStairs);
    register("minecraft:blackstone_wall", Block::BlackstoneWall);
    register("minecraft:blackstone_slab", Block::BlackstoneSlab);
    register("minecraft:polished_blackstone", Block::PolishedBlackstone);
    register(
        "minecraft:polished_blackstone_bricks",
        Block::PolishedBlackstoneBricks,
    );
    register(
        "minecraft:cracked_polished_blackstone_bricks",
        Block::CrackedPolishedBlackstoneBricks,
    );
    register(
        "minecraft:chiseled_polished_blackstone",
        Block::ChiseledPolishedBlackstone,
    );
    register(
        "minecraft:polished_blackstone_brick_slab",
        Block::PolishedBlackstoneBrickSlab,
    );
    register(
        "minecraft:polished_blackstone_brick_stairs",
        Block::PolishedBlackstoneBrickStairs,
    );
    register(
        "minecraft:polished_blackstone_brick_wall",
        Block::PolishedBlackstoneBrickWall,
    );
    register("minecraft:gilded_blackstone", Block::GildedBlackstone);
    register(
        "minecraft:polished_blackstone_stairs",
        Block::PolishedBlackstoneStairs,
    );
    register(
        "minecraft:polished_blackstone_slab",
        Block::PolishedBlackstoneSlab,
    );
    register(
        "minecraft:polished_blackstone_pressure_plate",
        Block::PolishedBlackstonePressurePlate,
    );
    register(
        "minecraft:polished_blackstone_button",
        Block::PolishedBlackstoneButton,
    );
    register(
        "minecraft:polished_blackstone_wall",
        Block::PolishedBlackstoneWall,
    );
    register(
        "minecraft:chiseled_nether_bricks",
        Block::ChiseledNetherBricks,
    );
    register(
        "minecraft:cracked_nether_bricks",
        Block::CrackedNetherBricks,
    );
    register("minecraft:quartz_bricks", Block::QuartzBricks);
    register("minecraft:candle", Block::Candle);
    register("minecraft:white_candle", Block::WhiteCandle);
    register("minecraft:orange_candle", Block::OrangeCandle);
    register("minecraft:magenta_candle", Block::MagentaCandle);
    register("minecraft:light_blue_candle", Block::LightBlueCandle);
    register("minecraft:yellow_candle", Block::YellowCandle);
    register("minecraft:lime_candle", Block::LimeCandle);
    register("minecraft:pink_candle", Block::PinkCandle);
    register("minecraft:gray_candle", Block::GrayCandle);
    register("minecraft:light_gray_candle", Block::LightGrayCandle);
    register("minecraft:cyan_candle", Block::CyanCandle);
    register("minecraft:purple_candle", Block::PurpleCandle);
    register("minecraft:blue_candle", Block::BlueCandle);
    register("minecraft:brown_candle", Block::BrownCandle);
    register("minecraft:green_candle", Block::GreenCandle);
    register("minecraft:red_candle", Block::RedCandle);
    register("minecraft:black_candle", Block::BlackCandle);
    register("minecraft:candle_cake", Block::CandleCake);
    register("minecraft:white_candle_cake", Block::WhiteCandleCake);
    register("minecraft:orange_candle_cake", Block::OrangeCandleCake);
    register("minecraft:magenta_candle_cake", Block::MagentaCandleCake);
    register(
        "minecraft:light_blue_candle_cake",
        Block::LightBlueCandleCake,
    );
    register("minecraft:yellow_candle_cake", Block::YellowCandleCake);
    register("minecraft:lime_candle_cake", Block::LimeCandleCake);
    register("minecraft:pink_candle_cake", Block::PinkCandleCake);
    register("minecraft:gray_candle_cake", Block::GrayCandleCake);
    register(
        "minecraft:light_gray_candle_cake",
        Block::LightGrayCandleCake,
    );
    register("minecraft:cyan_candle_cake", Block::CyanCandleCake);
    register("minecraft:purple_candle_cake", Block::PurpleCandleCake);
    register("minecraft:blue_candle_cake", Block::BlueCandleCake);
    register("minecraft:brown_candle_cake", Block::BrownCandleCake);
    register("minecraft:green_candle_cake", Block::GreenCandleCake);
    register("minecraft:red_candle_cake", Block::RedCandleCake);
    register("minecraft:black_candle_cake", Block::BlackCandleCake);
    register("minecraft:amethyst_block", Block::AmethystBlock);
    register("minecraft:budding_amethyst", Block::BuddingAmethyst);
    register("minecraft:amethyst_cluster", Block::AmethystCluster);
    register("minecraft:large_amethyst_bud", Block::LargeAmethystBud);
    register("minecraft:medium_amethyst_bud", Block::MediumAmethystBud);
    register("minecraft:small_amethyst_bud", Block::SmallAmethystBud);
    register("minecraft:tuff", Block::Tuff);
    register("minecraft:tuff_slab", Block::TuffSlab);
    register("minecraft:tuff_stairs", Block::TuffStairs);
    register("minecraft:tuff_wall", Block::TuffWall);
    register("minecraft:polished_tuff", Block::PolishedTuff);
    register("minecraft:polished_tuff_slab", Block::PolishedTuffSlab);
    register("minecraft:polished_tuff_stairs", Block::PolishedTuffStairs);
    register("minecraft:polished_tuff_wall", Block::PolishedTuffWall);
    register("minecraft:chiseled_tuff", Block::ChiseledTuff);
    register("minecraft:tuff_bricks", Block::TuffBricks);
    register("minecraft:tuff_brick_slab", Block::TuffBrickSlab);
    register("minecraft:tuff_brick_stairs", Block::TuffBrickStairs);
    register("minecraft:tuff_brick_wall", Block::TuffBrickWall);
    register("minecraft:chiseled_tuff_bricks", Block::ChiseledTuffBricks);
    register("minecraft:sulfur", Block::Sulfur);
    register("minecraft:potent_sulfur", Block::PotentSulfur);
    register("minecraft:sulfur_slab", Block::SulfurSlab);
    register("minecraft:sulfur_stairs", Block::SulfurStairs);
    register("minecraft:sulfur_wall", Block::SulfurWall);
    register("minecraft:polished_sulfur", Block::PolishedSulfur);
    register("minecraft:polished_sulfur_slab", Block::PolishedSulfurSlab);
    register(
        "minecraft:polished_sulfur_stairs",
        Block::PolishedSulfurStairs,
    );
    register("minecraft:polished_sulfur_wall", Block::PolishedSulfurWall);
    register("minecraft:sulfur_bricks", Block::SulfurBricks);
    register("minecraft:sulfur_brick_slab", Block::SulfurBrickSlab);
    register("minecraft:sulfur_brick_stairs", Block::SulfurBrickStairs);
    register("minecraft:sulfur_brick_wall", Block::SulfurBrickWall);
    register("minecraft:chiseled_sulfur", Block::ChiseledSulfur);
    register("minecraft:cinnabar", Block::Cinnabar);
    register("minecraft:cinnabar_slab", Block::CinnabarSlab);
    register("minecraft:cinnabar_stairs", Block::CinnabarStairs);
    register("minecraft:cinnabar_wall", Block::CinnabarWall);
    register("minecraft:polished_cinnabar", Block::PolishedCinnabar);
    register(
        "minecraft:polished_cinnabar_slab",
        Block::PolishedCinnabarSlab,
    );
    register(
        "minecraft:polished_cinnabar_stairs",
        Block::PolishedCinnabarStairs,
    );
    register(
        "minecraft:polished_cinnabar_wall",
        Block::PolishedCinnabarWall,
    );
    register("minecraft:cinnabar_bricks", Block::CinnabarBricks);
    register("minecraft:cinnabar_brick_slab", Block::CinnabarBrickSlab);
    register(
        "minecraft:cinnabar_brick_stairs",
        Block::CinnabarBrickStairs,
    );
    register("minecraft:cinnabar_brick_wall", Block::CinnabarBrickWall);
    register("minecraft:chiseled_cinnabar", Block::ChiseledCinnabar);
    register("minecraft:calcite", Block::Calcite);
    register("minecraft:tinted_glass", Block::TintedGlass);
    register("minecraft:powder_snow", Block::PowderSnow);
    register("minecraft:sculk_sensor", Block::SculkSensor);
    register(
        "minecraft:calibrated_sculk_sensor",
        Block::CalibratedSculkSensor,
    );
    register("minecraft:sculk", Block::Sculk);
    register("minecraft:sculk_vein", Block::SculkVein);
    register("minecraft:sculk_catalyst", Block::SculkCatalyst);
    register("minecraft:sculk_shrieker", Block::SculkShrieker);
    register("minecraft:copper_block", Block::CopperBlock);
    register("minecraft:exposed_copper", Block::ExposedCopper);
    register("minecraft:weathered_copper", Block::WeatheredCopper);
    register("minecraft:oxidized_copper", Block::OxidizedCopper);
    register("minecraft:waxed_copper_block", Block::WaxedCopperBlock);
    register("minecraft:waxed_exposed_copper", Block::WaxedExposedCopper);
    register(
        "minecraft:waxed_weathered_copper",
        Block::WaxedWeatheredCopper,
    );
    register(
        "minecraft:waxed_oxidized_copper",
        Block::WaxedOxidizedCopper,
    );
    register("minecraft:copper_ore", Block::CopperOre);
    register("minecraft:deepslate_copper_ore", Block::DeepslateCopperOre);
    register("minecraft:cut_copper", Block::CutCopper);
    register("minecraft:exposed_cut_copper", Block::ExposedCutCopper);
    register("minecraft:weathered_cut_copper", Block::WeatheredCutCopper);
    register("minecraft:oxidized_cut_copper", Block::OxidizedCutCopper);
    register("minecraft:waxed_cut_copper", Block::WaxedCutCopper);
    register(
        "minecraft:waxed_exposed_cut_copper",
        Block::WaxedExposedCutCopper,
    );
    register(
        "minecraft:waxed_weathered_cut_copper",
        Block::WaxedWeatheredCutCopper,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper",
        Block::WaxedOxidizedCutCopper,
    );
    register("minecraft:chiseled_copper", Block::ChiseledCopper);
    register(
        "minecraft:exposed_chiseled_copper",
        Block::ExposedChiseledCopper,
    );
    register(
        "minecraft:weathered_chiseled_copper",
        Block::WeatheredChiseledCopper,
    );
    register(
        "minecraft:oxidized_chiseled_copper",
        Block::OxidizedChiseledCopper,
    );
    register(
        "minecraft:waxed_chiseled_copper",
        Block::WaxedChiseledCopper,
    );
    register(
        "minecraft:waxed_exposed_chiseled_copper",
        Block::WaxedExposedChiseledCopper,
    );
    register(
        "minecraft:waxed_weathered_chiseled_copper",
        Block::WaxedWeatheredChiseledCopper,
    );
    register(
        "minecraft:waxed_oxidized_chiseled_copper",
        Block::WaxedOxidizedChiseledCopper,
    );
    register("minecraft:cut_copper_stairs", Block::CutCopperStairs);
    register(
        "minecraft:exposed_cut_copper_stairs",
        Block::ExposedCutCopperStairs,
    );
    register(
        "minecraft:weathered_cut_copper_stairs",
        Block::WeatheredCutCopperStairs,
    );
    register(
        "minecraft:oxidized_cut_copper_stairs",
        Block::OxidizedCutCopperStairs,
    );
    register(
        "minecraft:waxed_cut_copper_stairs",
        Block::WaxedCutCopperStairs,
    );
    register(
        "minecraft:waxed_exposed_cut_copper_stairs",
        Block::WaxedExposedCutCopperStairs,
    );
    register(
        "minecraft:waxed_weathered_cut_copper_stairs",
        Block::WaxedWeatheredCutCopperStairs,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper_stairs",
        Block::WaxedOxidizedCutCopperStairs,
    );
    register("minecraft:cut_copper_slab", Block::CutCopperSlab);
    register(
        "minecraft:exposed_cut_copper_slab",
        Block::ExposedCutCopperSlab,
    );
    register(
        "minecraft:weathered_cut_copper_slab",
        Block::WeatheredCutCopperSlab,
    );
    register(
        "minecraft:oxidized_cut_copper_slab",
        Block::OxidizedCutCopperSlab,
    );
    register("minecraft:waxed_cut_copper_slab", Block::WaxedCutCopperSlab);
    register(
        "minecraft:waxed_exposed_cut_copper_slab",
        Block::WaxedExposedCutCopperSlab,
    );
    register(
        "minecraft:waxed_weathered_cut_copper_slab",
        Block::WaxedWeatheredCutCopperSlab,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper_slab",
        Block::WaxedOxidizedCutCopperSlab,
    );
    register("minecraft:copper_door", Block::CopperDoor);
    register("minecraft:exposed_copper_door", Block::ExposedCopperDoor);
    register(
        "minecraft:weathered_copper_door",
        Block::WeatheredCopperDoor,
    );
    register("minecraft:oxidized_copper_door", Block::OxidizedCopperDoor);
    register("minecraft:waxed_copper_door", Block::WaxedCopperDoor);
    register(
        "minecraft:waxed_exposed_copper_door",
        Block::WaxedExposedCopperDoor,
    );
    register(
        "minecraft:waxed_weathered_copper_door",
        Block::WaxedWeatheredCopperDoor,
    );
    register(
        "minecraft:waxed_oxidized_copper_door",
        Block::WaxedOxidizedCopperDoor,
    );
    register("minecraft:copper_trapdoor", Block::CopperTrapdoor);
    register(
        "minecraft:exposed_copper_trapdoor",
        Block::ExposedCopperTrapdoor,
    );
    register(
        "minecraft:weathered_copper_trapdoor",
        Block::WeatheredCopperTrapdoor,
    );
    register(
        "minecraft:oxidized_copper_trapdoor",
        Block::OxidizedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_copper_trapdoor",
        Block::WaxedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_exposed_copper_trapdoor",
        Block::WaxedExposedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_weathered_copper_trapdoor",
        Block::WaxedWeatheredCopperTrapdoor,
    );
    register(
        "minecraft:waxed_oxidized_copper_trapdoor",
        Block::WaxedOxidizedCopperTrapdoor,
    );
    register("minecraft:copper_grate", Block::CopperGrate);
    register("minecraft:exposed_copper_grate", Block::ExposedCopperGrate);
    register(
        "minecraft:weathered_copper_grate",
        Block::WeatheredCopperGrate,
    );
    register(
        "minecraft:oxidized_copper_grate",
        Block::OxidizedCopperGrate,
    );
    register("minecraft:waxed_copper_grate", Block::WaxedCopperGrate);
    register(
        "minecraft:waxed_exposed_copper_grate",
        Block::WaxedExposedCopperGrate,
    );
    register(
        "minecraft:waxed_weathered_copper_grate",
        Block::WaxedWeatheredCopperGrate,
    );
    register(
        "minecraft:waxed_oxidized_copper_grate",
        Block::WaxedOxidizedCopperGrate,
    );
    register("minecraft:copper_bulb", Block::CopperBulb);
    register("minecraft:exposed_copper_bulb", Block::ExposedCopperBulb);
    register(
        "minecraft:weathered_copper_bulb",
        Block::WeatheredCopperBulb,
    );
    register("minecraft:oxidized_copper_bulb", Block::OxidizedCopperBulb);
    register("minecraft:waxed_copper_bulb", Block::WaxedCopperBulb);
    register(
        "minecraft:waxed_exposed_copper_bulb",
        Block::WaxedExposedCopperBulb,
    );
    register(
        "minecraft:waxed_weathered_copper_bulb",
        Block::WaxedWeatheredCopperBulb,
    );
    register(
        "minecraft:waxed_oxidized_copper_bulb",
        Block::WaxedOxidizedCopperBulb,
    );
    register("minecraft:copper_chest", Block::CopperChest);
    register("minecraft:exposed_copper_chest", Block::ExposedCopperChest);
    register(
        "minecraft:weathered_copper_chest",
        Block::WeatheredCopperChest,
    );
    register(
        "minecraft:oxidized_copper_chest",
        Block::OxidizedCopperChest,
    );
    register("minecraft:waxed_copper_chest", Block::WaxedCopperChest);
    register(
        "minecraft:waxed_exposed_copper_chest",
        Block::WaxedExposedCopperChest,
    );
    register(
        "minecraft:waxed_weathered_copper_chest",
        Block::WaxedWeatheredCopperChest,
    );
    register(
        "minecraft:waxed_oxidized_copper_chest",
        Block::WaxedOxidizedCopperChest,
    );
    register("minecraft:copper_golem_statue", Block::CopperGolemStatue);
    register(
        "minecraft:exposed_copper_golem_statue",
        Block::ExposedCopperGolemStatue,
    );
    register(
        "minecraft:weathered_copper_golem_statue",
        Block::WeatheredCopperGolemStatue,
    );
    register(
        "minecraft:oxidized_copper_golem_statue",
        Block::OxidizedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_copper_golem_statue",
        Block::WaxedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_exposed_copper_golem_statue",
        Block::WaxedExposedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_weathered_copper_golem_statue",
        Block::WaxedWeatheredCopperGolemStatue,
    );
    register(
        "minecraft:waxed_oxidized_copper_golem_statue",
        Block::WaxedOxidizedCopperGolemStatue,
    );
    register("minecraft:lightning_rod", Block::LightningRod);
    register(
        "minecraft:exposed_lightning_rod",
        Block::ExposedLightningRod,
    );
    register(
        "minecraft:weathered_lightning_rod",
        Block::WeatheredLightningRod,
    );
    register(
        "minecraft:oxidized_lightning_rod",
        Block::OxidizedLightningRod,
    );
    register("minecraft:waxed_lightning_rod", Block::WaxedLightningRod);
    register(
        "minecraft:waxed_exposed_lightning_rod",
        Block::WaxedExposedLightningRod,
    );
    register(
        "minecraft:waxed_weathered_lightning_rod",
        Block::WaxedWeatheredLightningRod,
    );
    register(
        "minecraft:waxed_oxidized_lightning_rod",
        Block::WaxedOxidizedLightningRod,
    );
    register("minecraft:dripstone_block", Block::DripstoneBlock);
    register("minecraft:pointed_dripstone", Block::PointedDripstone);
    register("minecraft:sulfur_spike", Block::SulfurSpike);
    register("minecraft:cave_vines", Block::CaveVines);
    register("minecraft:cave_vines_plant", Block::CaveVinesPlant);
    register("minecraft:spore_blossom", Block::SporeBlossom);
    register("minecraft:azalea", Block::Azalea);
    register("minecraft:flowering_azalea", Block::FloweringAzalea);
    register("minecraft:moss_carpet", Block::MossCarpet);
    register("minecraft:pink_petals", Block::PinkPetals);
    register("minecraft:wildflowers", Block::Wildflowers);
    register("minecraft:leaf_litter", Block::LeafLitter);
    register("minecraft:moss_block", Block::MossBlock);
    register("minecraft:big_dripleaf", Block::BigDripleaf);
    register("minecraft:big_dripleaf_stem", Block::BigDripleafStem);
    register("minecraft:small_dripleaf", Block::SmallDripleaf);
    register("minecraft:hanging_roots", Block::HangingRoots);
    register("minecraft:rooted_dirt", Block::RootedDirt);
    register("minecraft:mud", Block::Mud);
    register("minecraft:deepslate", Block::Deepslate);
    register("minecraft:cobbled_deepslate", Block::CobbledDeepslate);
    register(
        "minecraft:cobbled_deepslate_stairs",
        Block::CobbledDeepslateStairs,
    );
    register(
        "minecraft:cobbled_deepslate_slab",
        Block::CobbledDeepslateSlab,
    );
    register(
        "minecraft:cobbled_deepslate_wall",
        Block::CobbledDeepslateWall,
    );
    register("minecraft:polished_deepslate", Block::PolishedDeepslate);
    register(
        "minecraft:polished_deepslate_stairs",
        Block::PolishedDeepslateStairs,
    );
    register(
        "minecraft:polished_deepslate_slab",
        Block::PolishedDeepslateSlab,
    );
    register(
        "minecraft:polished_deepslate_wall",
        Block::PolishedDeepslateWall,
    );
    register("minecraft:deepslate_tiles", Block::DeepslateTiles);
    register(
        "minecraft:deepslate_tile_stairs",
        Block::DeepslateTileStairs,
    );
    register("minecraft:deepslate_tile_slab", Block::DeepslateTileSlab);
    register("minecraft:deepslate_tile_wall", Block::DeepslateTileWall);
    register("minecraft:deepslate_bricks", Block::DeepslateBricks);
    register(
        "minecraft:deepslate_brick_stairs",
        Block::DeepslateBrickStairs,
    );
    register("minecraft:deepslate_brick_slab", Block::DeepslateBrickSlab);
    register("minecraft:deepslate_brick_wall", Block::DeepslateBrickWall);
    register("minecraft:chiseled_deepslate", Block::ChiseledDeepslate);
    register(
        "minecraft:cracked_deepslate_bricks",
        Block::CrackedDeepslateBricks,
    );
    register(
        "minecraft:cracked_deepslate_tiles",
        Block::CrackedDeepslateTiles,
    );
    register("minecraft:infested_deepslate", Block::InfestedDeepslate);
    register("minecraft:smooth_basalt", Block::SmoothBasalt);
    register("minecraft:raw_iron_block", Block::RawIronBlock);
    register("minecraft:raw_copper_block", Block::RawCopperBlock);
    register("minecraft:raw_gold_block", Block::RawGoldBlock);
    register("minecraft:potted_azalea_bush", Block::PottedAzaleaBush);
    register(
        "minecraft:potted_flowering_azalea_bush",
        Block::PottedFloweringAzaleaBush,
    );
    register("minecraft:ochre_froglight", Block::OchreFroglight);
    register("minecraft:verdant_froglight", Block::VerdantFroglight);
    register(
        "minecraft:pearlescent_froglight",
        Block::PearlescentFroglight,
    );
    register("minecraft:frogspawn", Block::Frogspawn);
    register("minecraft:reinforced_deepslate", Block::ReinforcedDeepslate);
    register("minecraft:decorated_pot", Block::DecoratedPot);
    register("minecraft:crafter", Block::Crafter);
    register("minecraft:trial_spawner", Block::TrialSpawner);
    register("minecraft:vault", Block::Vault);
    register("minecraft:heavy_core", Block::HeavyCore);
    register("minecraft:pale_moss_block", Block::PaleMossBlock);
    register("minecraft:pale_moss_carpet", Block::PaleMossCarpet);
    register("minecraft:pale_hanging_moss", Block::PaleHangingMoss);
    register("minecraft:open_eyeblossom", Block::OpenEyeblossom);
    register("minecraft:closed_eyeblossom", Block::ClosedEyeblossom);
    register(
        "minecraft:potted_open_eyeblossom",
        Block::PottedOpenEyeblossom,
    );
    register(
        "minecraft:potted_closed_eyeblossom",
        Block::PottedClosedEyeblossom,
    );
    register("minecraft:firefly_bush", Block::FireflyBush);
}
