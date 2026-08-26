// This file was auto-generated. Do not edit it manually.

use crate::registry::Registry;
use cerium_macros::StaticObject;
use cerium_macros::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StaticObject, UnitEnum)]
#[repr(u16)]
pub enum Material {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    Deepslate,
    CobbledDeepslate,
    PolishedDeepslate,
    Calcite,
    Tuff,
    TuffSlab,
    TuffStairs,
    TuffWall,
    ChiseledTuff,
    PolishedTuff,
    PolishedTuffSlab,
    PolishedTuffStairs,
    PolishedTuffWall,
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
    DripstoneBlock,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    RootedDirt,
    Mud,
    CrimsonNylium,
    WarpedNylium,
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    CherryPlanks,
    DarkOakPlanks,
    PaleOakPlanks,
    MangrovePlanks,
    BambooPlanks,
    CrimsonPlanks,
    WarpedPlanks,
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
    Sand,
    SuspiciousSand,
    SuspiciousGravel,
    RedSand,
    Gravel,
    CoalOre,
    DeepslateCoalOre,
    IronOre,
    DeepslateIronOre,
    CopperOre,
    DeepslateCopperOre,
    GoldOre,
    DeepslateGoldOre,
    RedstoneOre,
    DeepslateRedstoneOre,
    EmeraldOre,
    DeepslateEmeraldOre,
    LapisOre,
    DeepslateLapisOre,
    DiamondOre,
    DeepslateDiamondOre,
    NetherGoldOre,
    NetherQuartzOre,
    AncientDebris,
    CoalBlock,
    RawIronBlock,
    RawCopperBlock,
    RawGoldBlock,
    HeavyCore,
    AmethystBlock,
    BuddingAmethyst,
    IronBlock,
    CopperBlock,
    ExposedCopper,
    WeatheredCopper,
    OxidizedCopper,
    WaxedCopperBlock,
    WaxedExposedCopper,
    WaxedWeatheredCopper,
    WaxedOxidizedCopper,
    GoldBlock,
    DiamondBlock,
    NetheriteBlock,
    ChiseledCopper,
    ExposedChiseledCopper,
    WeatheredChiseledCopper,
    OxidizedChiseledCopper,
    WaxedChiseledCopper,
    WaxedExposedChiseledCopper,
    WaxedWeatheredChiseledCopper,
    WaxedOxidizedChiseledCopper,
    CutCopper,
    ExposedCutCopper,
    WeatheredCutCopper,
    OxidizedCutCopper,
    WaxedCutCopper,
    WaxedExposedCutCopper,
    WaxedWeatheredCutCopper,
    WaxedOxidizedCutCopper,
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
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    AcaciaLog,
    CherryLog,
    PaleOakLog,
    DarkOakLog,
    MangroveLog,
    MangroveRoots,
    MuddyMangroveRoots,
    CrimsonStem,
    WarpedStem,
    BambooBlock,
    StrippedOakLog,
    StrippedSpruceLog,
    StrippedBirchLog,
    StrippedJungleLog,
    StrippedAcaciaLog,
    StrippedCherryLog,
    StrippedDarkOakLog,
    StrippedPaleOakLog,
    StrippedMangroveLog,
    StrippedCrimsonStem,
    StrippedWarpedStem,
    StrippedOakWood,
    StrippedSpruceWood,
    StrippedBirchWood,
    StrippedJungleWood,
    StrippedAcaciaWood,
    StrippedCherryWood,
    StrippedDarkOakWood,
    StrippedPaleOakWood,
    StrippedMangroveWood,
    StrippedCrimsonHyphae,
    StrippedWarpedHyphae,
    StrippedBambooBlock,
    OakWood,
    SpruceWood,
    BirchWood,
    JungleWood,
    AcaciaWood,
    CherryWood,
    PaleOakWood,
    DarkOakWood,
    MangroveWood,
    CrimsonHyphae,
    WarpedHyphae,
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
    TintedGlass,
    LapisBlock,
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    Cobweb,
    ShortGrass,
    Fern,
    Bush,
    Azalea,
    FloweringAzalea,
    DeadBush,
    FireflyBush,
    ShortDryGrass,
    TallDryGrass,
    Seagrass,
    SeaPickle,
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
    Dandelion,
    GoldenDandelion,
    OpenEyeblossom,
    ClosedEyeblossom,
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
    LilyOfTheValley,
    WitherRose,
    Torchflower,
    PitcherPlant,
    SporeBlossom,
    BrownMushroom,
    RedMushroom,
    CrimsonFungus,
    WarpedFungus,
    CrimsonRoots,
    WarpedRoots,
    NetherSprouts,
    WeepingVines,
    TwistingVines,
    SugarCane,
    Kelp,
    PinkPetals,
    Wildflowers,
    LeafLitter,
    MossCarpet,
    MossBlock,
    PaleMossCarpet,
    PaleHangingMoss,
    PaleMossBlock,
    HangingRoots,
    BigDripleaf,
    SmallDripleaf,
    Bamboo,
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
    CrimsonSlab,
    WarpedSlab,
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
    PrismarineSlab,
    PrismarineBrickSlab,
    DarkPrismarineSlab,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Bricks,
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
    Bookshelf,
    ChiseledBookshelf,
    DecoratedPot,
    MossyCobblestone,
    Obsidian,
    Torch,
    EndRod,
    ChorusPlant,
    ChorusFlower,
    PurpurBlock,
    PurpurPillar,
    PurpurStairs,
    Spawner,
    CreakingHeart,
    Chest,
    CraftingTable,
    Farmland,
    Furnace,
    Ladder,
    CobblestoneStairs,
    Snow,
    Ice,
    SnowBlock,
    Cactus,
    CactusFlower,
    Clay,
    Jukebox,
    OakFence,
    SpruceFence,
    BirchFence,
    JungleFence,
    AcaciaFence,
    CherryFence,
    DarkOakFence,
    PaleOakFence,
    MangroveFence,
    BambooFence,
    CrimsonFence,
    WarpedFence,
    Pumpkin,
    CarvedPumpkin,
    JackOLantern,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    PolishedBasalt,
    SmoothBasalt,
    SoulTorch,
    CopperTorch,
    Glowstone,
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    InfestedDeepslate,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    PackedMud,
    MudBricks,
    DeepslateBricks,
    CrackedDeepslateBricks,
    DeepslateTiles,
    CrackedDeepslateTiles,
    ChiseledDeepslate,
    ReinforcedDeepslate,
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
    Melon,
    Vine,
    GlowLichen,
    ResinClump,
    ResinBlock,
    ResinBricks,
    ResinBrickStairs,
    ResinBrickSlab,
    ResinBrickWall,
    ChiseledResinBricks,
    BrickStairs,
    StoneBrickStairs,
    MudBrickStairs,
    Mycelium,
    LilyPad,
    NetherBricks,
    CrackedNetherBricks,
    ChiseledNetherBricks,
    NetherBrickFence,
    NetherBrickStairs,
    Sculk,
    SculkVein,
    SculkCatalyst,
    SculkShrieker,
    EnchantingTable,
    EndPortalFrame,
    EndStone,
    EndStoneBricks,
    DragonEgg,
    SandstoneStairs,
    EnderChest,
    EmeraldBlock,
    OakStairs,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    AcaciaStairs,
    CherryStairs,
    DarkOakStairs,
    PaleOakStairs,
    MangroveStairs,
    BambooStairs,
    BambooMosaicStairs,
    CrimsonStairs,
    WarpedStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    MossyCobblestoneWall,
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
    BlackstoneWall,
    PolishedBlackstoneWall,
    PolishedBlackstoneBrickWall,
    CobbledDeepslateWall,
    PolishedDeepslateWall,
    DeepslateBrickWall,
    DeepslateTileWall,
    Anvil,
    ChippedAnvil,
    DamagedAnvil,
    ChiseledQuartzBlock,
    QuartzBlock,
    QuartzBricks,
    QuartzPillar,
    QuartzStairs,
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
    Barrier,
    Light,
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
    PackedIce,
    DirtPath,
    Sunflower,
    Lilac,
    RoseBush,
    Peony,
    TallGrass,
    LargeFern,
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
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs,
    PrismarineBrickStairs,
    DarkPrismarineStairs,
    SeaLantern,
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs,
    RepeatingCommandBlock,
    ChainCommandBlock,
    MagmaBlock,
    NetherWartBlock,
    WarpedWartBlock,
    RedNetherBricks,
    BoneBlock,
    StructureVoid,
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
    TubeCoral,
    BrainCoral,
    BubbleCoral,
    FireCoral,
    HornCoral,
    DeadBrainCoral,
    DeadBubbleCoral,
    DeadFireCoral,
    DeadHornCoral,
    DeadTubeCoral,
    TubeCoralFan,
    BrainCoralFan,
    BubbleCoralFan,
    FireCoralFan,
    HornCoralFan,
    DeadTubeCoralFan,
    DeadBrainCoralFan,
    DeadBubbleCoralFan,
    DeadFireCoralFan,
    DeadHornCoralFan,
    BlueIce,
    Conduit,
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
    CobbledDeepslateStairs,
    PolishedDeepslateStairs,
    DeepslateBrickStairs,
    DeepslateTileStairs,
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
    CobbledDeepslateSlab,
    PolishedDeepslateSlab,
    DeepslateBrickSlab,
    DeepslateTileSlab,
    Scaffolding,
    Redstone,
    RedstoneTorch,
    RedstoneBlock,
    Repeater,
    Comparator,
    Piston,
    StickyPiston,
    SlimeBlock,
    HoneyBlock,
    Observer,
    Hopper,
    Dispenser,
    Dropper,
    Lectern,
    Target,
    Lever,
    LightningRod,
    ExposedLightningRod,
    WeatheredLightningRod,
    OxidizedLightningRod,
    WaxedLightningRod,
    WaxedExposedLightningRod,
    WaxedWeatheredLightningRod,
    WaxedOxidizedLightningRod,
    DaylightDetector,
    SculkSensor,
    CalibratedSculkSensor,
    TripwireHook,
    TrappedChest,
    Tnt,
    RedstoneLamp,
    NoteBlock,
    StoneButton,
    PolishedBlackstoneButton,
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
    CrimsonButton,
    WarpedButton,
    StonePressurePlate,
    PolishedBlackstonePressurePlate,
    LightWeightedPressurePlate,
    HeavyWeightedPressurePlate,
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
    CrimsonPressurePlate,
    WarpedPressurePlate,
    IronDoor,
    OakDoor,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    CherryDoor,
    DarkOakDoor,
    PaleOakDoor,
    MangroveDoor,
    BambooDoor,
    CrimsonDoor,
    WarpedDoor,
    CopperDoor,
    ExposedCopperDoor,
    WeatheredCopperDoor,
    OxidizedCopperDoor,
    WaxedCopperDoor,
    WaxedExposedCopperDoor,
    WaxedWeatheredCopperDoor,
    WaxedOxidizedCopperDoor,
    IronTrapdoor,
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
    CrimsonTrapdoor,
    WarpedTrapdoor,
    CopperTrapdoor,
    ExposedCopperTrapdoor,
    WeatheredCopperTrapdoor,
    OxidizedCopperTrapdoor,
    WaxedCopperTrapdoor,
    WaxedExposedCopperTrapdoor,
    WaxedWeatheredCopperTrapdoor,
    WaxedOxidizedCopperTrapdoor,
    OakFenceGate,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    AcaciaFenceGate,
    CherryFenceGate,
    DarkOakFenceGate,
    PaleOakFenceGate,
    MangroveFenceGate,
    BambooFenceGate,
    CrimsonFenceGate,
    WarpedFenceGate,
    PoweredRail,
    DetectorRail,
    Rail,
    ActivatorRail,
    Saddle,
    WhiteHarness,
    OrangeHarness,
    MagentaHarness,
    LightBlueHarness,
    YellowHarness,
    LimeHarness,
    PinkHarness,
    GrayHarness,
    LightGrayHarness,
    CyanHarness,
    PurpleHarness,
    BlueHarness,
    BrownHarness,
    GreenHarness,
    RedHarness,
    BlackHarness,
    Minecart,
    ChestMinecart,
    FurnaceMinecart,
    TntMinecart,
    HopperMinecart,
    CarrotOnAStick,
    WarpedFungusOnAStick,
    PhantomMembrane,
    Elytra,
    OakBoat,
    OakChestBoat,
    SpruceBoat,
    SpruceChestBoat,
    BirchBoat,
    BirchChestBoat,
    JungleBoat,
    JungleChestBoat,
    AcaciaBoat,
    AcaciaChestBoat,
    CherryBoat,
    CherryChestBoat,
    DarkOakBoat,
    DarkOakChestBoat,
    PaleOakBoat,
    PaleOakChestBoat,
    MangroveBoat,
    MangroveChestBoat,
    BambooRaft,
    BambooChestRaft,
    StructureBlock,
    Jigsaw,
    TestBlock,
    TestInstanceBlock,
    TurtleHelmet,
    TurtleScute,
    ArmadilloScute,
    WolfArmor,
    FlintAndSteel,
    Bowl,
    Apple,
    Bow,
    Arrow,
    Coal,
    Charcoal,
    Diamond,
    Emerald,
    LapisLazuli,
    Quartz,
    AmethystShard,
    RawIron,
    IronIngot,
    RawCopper,
    CopperIngot,
    RawGold,
    GoldIngot,
    NetheriteIngot,
    NetheriteScrap,
    WoodenSword,
    WoodenShovel,
    WoodenPickaxe,
    WoodenAxe,
    WoodenHoe,
    CopperSword,
    CopperShovel,
    CopperPickaxe,
    CopperAxe,
    CopperHoe,
    StoneSword,
    StoneShovel,
    StonePickaxe,
    StoneAxe,
    StoneHoe,
    GoldenSword,
    GoldenShovel,
    GoldenPickaxe,
    GoldenAxe,
    GoldenHoe,
    IronSword,
    IronShovel,
    IronPickaxe,
    IronAxe,
    IronHoe,
    DiamondSword,
    DiamondShovel,
    DiamondPickaxe,
    DiamondAxe,
    DiamondHoe,
    NetheriteSword,
    NetheriteShovel,
    NetheritePickaxe,
    NetheriteAxe,
    NetheriteHoe,
    Stick,
    MushroomStew,
    String,
    Feather,
    Gunpowder,
    WheatSeeds,
    Wheat,
    Bread,
    LeatherHelmet,
    LeatherChestplate,
    LeatherLeggings,
    LeatherBoots,
    CopperHelmet,
    CopperChestplate,
    CopperLeggings,
    CopperBoots,
    ChainmailHelmet,
    ChainmailChestplate,
    ChainmailLeggings,
    ChainmailBoots,
    IronHelmet,
    IronChestplate,
    IronLeggings,
    IronBoots,
    DiamondHelmet,
    DiamondChestplate,
    DiamondLeggings,
    DiamondBoots,
    GoldenHelmet,
    GoldenChestplate,
    GoldenLeggings,
    GoldenBoots,
    NetheriteHelmet,
    NetheriteChestplate,
    NetheriteLeggings,
    NetheriteBoots,
    Flint,
    Porkchop,
    CookedPorkchop,
    Painting,
    GoldenApple,
    EnchantedGoldenApple,
    OakSign,
    SpruceSign,
    BirchSign,
    JungleSign,
    AcaciaSign,
    CherrySign,
    DarkOakSign,
    PaleOakSign,
    MangroveSign,
    BambooSign,
    CrimsonSign,
    WarpedSign,
    OakHangingSign,
    SpruceHangingSign,
    BirchHangingSign,
    JungleHangingSign,
    AcaciaHangingSign,
    CherryHangingSign,
    DarkOakHangingSign,
    PaleOakHangingSign,
    MangroveHangingSign,
    BambooHangingSign,
    CrimsonHangingSign,
    WarpedHangingSign,
    Bucket,
    WaterBucket,
    LavaBucket,
    PowderSnowBucket,
    Snowball,
    Leather,
    MilkBucket,
    PufferfishBucket,
    SalmonBucket,
    CodBucket,
    TropicalFishBucket,
    AxolotlBucket,
    SulfurCubeBucket,
    TadpoleBucket,
    Brick,
    ClayBall,
    DriedKelpBlock,
    Paper,
    Book,
    SlimeBall,
    Egg,
    BlueEgg,
    BrownEgg,
    Compass,
    RecoveryCompass,
    Bundle,
    WhiteBundle,
    OrangeBundle,
    MagentaBundle,
    LightBlueBundle,
    YellowBundle,
    LimeBundle,
    PinkBundle,
    GrayBundle,
    LightGrayBundle,
    CyanBundle,
    PurpleBundle,
    BlueBundle,
    BrownBundle,
    GreenBundle,
    RedBundle,
    BlackBundle,
    FishingRod,
    Clock,
    Spyglass,
    GlowstoneDust,
    Cod,
    Salmon,
    TropicalFish,
    Pufferfish,
    CookedCod,
    CookedSalmon,
    InkSac,
    GlowInkSac,
    CocoaBeans,
    WhiteDye,
    OrangeDye,
    MagentaDye,
    LightBlueDye,
    YellowDye,
    LimeDye,
    PinkDye,
    GrayDye,
    LightGrayDye,
    CyanDye,
    PurpleDye,
    BlueDye,
    BrownDye,
    GreenDye,
    RedDye,
    BlackDye,
    BoneMeal,
    Bone,
    Sugar,
    Cake,
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
    Cookie,
    Crafter,
    FilledMap,
    Shears,
    MelonSlice,
    DriedKelp,
    PumpkinSeeds,
    MelonSeeds,
    Beef,
    CookedBeef,
    Chicken,
    CookedChicken,
    RottenFlesh,
    EnderPearl,
    BlazeRod,
    GhastTear,
    GoldNugget,
    NetherWart,
    GlassBottle,
    Potion,
    SpiderEye,
    FermentedSpiderEye,
    BlazePowder,
    MagmaCream,
    BrewingStand,
    Cauldron,
    EnderEye,
    GlisteringMelonSlice,
    ChickenSpawnEgg,
    CowSpawnEgg,
    PigSpawnEgg,
    SheepSpawnEgg,
    CamelSpawnEgg,
    DonkeySpawnEgg,
    HorseSpawnEgg,
    MuleSpawnEgg,
    CatSpawnEgg,
    ParrotSpawnEgg,
    WolfSpawnEgg,
    ArmadilloSpawnEgg,
    BatSpawnEgg,
    BeeSpawnEgg,
    FoxSpawnEgg,
    GoatSpawnEgg,
    LlamaSpawnEgg,
    OcelotSpawnEgg,
    PandaSpawnEgg,
    PolarBearSpawnEgg,
    RabbitSpawnEgg,
    AxolotlSpawnEgg,
    CodSpawnEgg,
    DolphinSpawnEgg,
    FrogSpawnEgg,
    GlowSquidSpawnEgg,
    NautilusSpawnEgg,
    PufferfishSpawnEgg,
    SalmonSpawnEgg,
    SquidSpawnEgg,
    TadpoleSpawnEgg,
    TropicalFishSpawnEgg,
    TurtleSpawnEgg,
    AllaySpawnEgg,
    MooshroomSpawnEgg,
    SnifferSpawnEgg,
    SulfurCubeSpawnEgg,
    CopperGolemSpawnEgg,
    IronGolemSpawnEgg,
    SnowGolemSpawnEgg,
    TraderLlamaSpawnEgg,
    VillagerSpawnEgg,
    WanderingTraderSpawnEgg,
    BoggedSpawnEgg,
    CamelHuskSpawnEgg,
    DrownedSpawnEgg,
    HuskSpawnEgg,
    ParchedSpawnEgg,
    SkeletonSpawnEgg,
    SkeletonHorseSpawnEgg,
    StraySpawnEgg,
    WitherSpawnEgg,
    WitherSkeletonSpawnEgg,
    ZombieSpawnEgg,
    ZombieHorseSpawnEgg,
    ZombieNautilusSpawnEgg,
    ZombieVillagerSpawnEgg,
    CaveSpiderSpawnEgg,
    SpiderSpawnEgg,
    BreezeSpawnEgg,
    CreakingSpawnEgg,
    CreeperSpawnEgg,
    ElderGuardianSpawnEgg,
    GuardianSpawnEgg,
    PhantomSpawnEgg,
    SilverfishSpawnEgg,
    SlimeSpawnEgg,
    WardenSpawnEgg,
    WitchSpawnEgg,
    EvokerSpawnEgg,
    PillagerSpawnEgg,
    RavagerSpawnEgg,
    VindicatorSpawnEgg,
    VexSpawnEgg,
    BlazeSpawnEgg,
    GhastSpawnEgg,
    HappyGhastSpawnEgg,
    HoglinSpawnEgg,
    MagmaCubeSpawnEgg,
    PiglinSpawnEgg,
    PiglinBruteSpawnEgg,
    StriderSpawnEgg,
    ZoglinSpawnEgg,
    ZombifiedPiglinSpawnEgg,
    EnderDragonSpawnEgg,
    EndermanSpawnEgg,
    EndermiteSpawnEgg,
    ShulkerSpawnEgg,
    ExperienceBottle,
    FireCharge,
    WindCharge,
    WritableBook,
    WrittenBook,
    BreezeRod,
    Mace,
    ItemFrame,
    GlowItemFrame,
    FlowerPot,
    Carrot,
    Potato,
    BakedPotato,
    PoisonousPotato,
    Map,
    GoldenCarrot,
    SkeletonSkull,
    WitherSkeletonSkull,
    PlayerHead,
    ZombieHead,
    CreeperHead,
    DragonHead,
    PiglinHead,
    NetherStar,
    PumpkinPie,
    FireworkRocket,
    FireworkStar,
    EnchantedBook,
    NetherBrick,
    ResinBrick,
    PrismarineShard,
    PrismarineCrystals,
    Rabbit,
    CookedRabbit,
    RabbitStew,
    RabbitFoot,
    RabbitHide,
    ArmorStand,
    CopperHorseArmor,
    IronHorseArmor,
    GoldenHorseArmor,
    DiamondHorseArmor,
    NetheriteHorseArmor,
    LeatherHorseArmor,
    Lead,
    NameTag,
    CommandBlockMinecart,
    Mutton,
    CookedMutton,
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
    EndCrystal,
    ChorusFruit,
    PoppedChorusFruit,
    TorchflowerSeeds,
    PitcherPod,
    Beetroot,
    BeetrootSeeds,
    BeetrootSoup,
    DragonBreath,
    SplashPotion,
    SpectralArrow,
    TippedArrow,
    LingeringPotion,
    Shield,
    WoodenSpear,
    StoneSpear,
    CopperSpear,
    IronSpear,
    GoldenSpear,
    DiamondSpear,
    NetheriteSpear,
    TotemOfUndying,
    ShulkerShell,
    IronNugget,
    CopperNugget,
    KnowledgeBook,
    DebugStick,
    MusicDisc13,
    MusicDiscCat,
    MusicDiscBlocks,
    MusicDiscBounce,
    MusicDiscChirp,
    MusicDiscCreator,
    MusicDiscCreatorMusicBox,
    MusicDiscFar,
    MusicDiscLavaChicken,
    MusicDiscMall,
    MusicDiscMellohi,
    MusicDiscStal,
    MusicDiscStrad,
    MusicDiscWard,
    MusicDisc11,
    MusicDiscWait,
    MusicDiscOtherside,
    MusicDiscRelic,
    MusicDisc5,
    MusicDiscPigstep,
    MusicDiscPrecipice,
    MusicDiscTears,
    DiscFragment5,
    Trident,
    NautilusShell,
    IronNautilusArmor,
    GoldenNautilusArmor,
    DiamondNautilusArmor,
    NetheriteNautilusArmor,
    CopperNautilusArmor,
    HeartOfTheSea,
    Crossbow,
    SuspiciousStew,
    Loom,
    FlowerBannerPattern,
    CreeperBannerPattern,
    SkullBannerPattern,
    MojangBannerPattern,
    GlobeBannerPattern,
    PiglinBannerPattern,
    FlowBannerPattern,
    GusterBannerPattern,
    FieldMasonedBannerPattern,
    BordureIndentedBannerPattern,
    GoatHorn,
    Composter,
    Barrel,
    Smoker,
    BlastFurnace,
    CartographyTable,
    FletchingTable,
    Grindstone,
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
    SweetBerries,
    GlowBerries,
    Campfire,
    SoulCampfire,
    Shroomlight,
    Honeycomb,
    BeeNest,
    Beehive,
    HoneyBottle,
    HoneycombBlock,
    Lodestone,
    CryingObsidian,
    Blackstone,
    BlackstoneSlab,
    BlackstoneStairs,
    GildedBlackstone,
    PolishedBlackstone,
    PolishedBlackstoneSlab,
    PolishedBlackstoneStairs,
    ChiseledPolishedBlackstone,
    PolishedBlackstoneBricks,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneBrickStairs,
    CrackedPolishedBlackstoneBricks,
    RespawnAnchor,
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
    SmallAmethystBud,
    MediumAmethystBud,
    LargeAmethystBud,
    AmethystCluster,
    PointedDripstone,
    SulfurSpike,
    OchreFroglight,
    VerdantFroglight,
    PearlescentFroglight,
    Frogspawn,
    EchoShard,
    Brush,
    NetheriteUpgradeSmithingTemplate,
    SentryArmorTrimSmithingTemplate,
    DuneArmorTrimSmithingTemplate,
    CoastArmorTrimSmithingTemplate,
    WildArmorTrimSmithingTemplate,
    WardArmorTrimSmithingTemplate,
    EyeArmorTrimSmithingTemplate,
    VexArmorTrimSmithingTemplate,
    TideArmorTrimSmithingTemplate,
    SnoutArmorTrimSmithingTemplate,
    RibArmorTrimSmithingTemplate,
    SpireArmorTrimSmithingTemplate,
    WayfinderArmorTrimSmithingTemplate,
    ShaperArmorTrimSmithingTemplate,
    SilenceArmorTrimSmithingTemplate,
    RaiserArmorTrimSmithingTemplate,
    HostArmorTrimSmithingTemplate,
    FlowArmorTrimSmithingTemplate,
    BoltArmorTrimSmithingTemplate,
    AnglerPotterySherd,
    ArcherPotterySherd,
    ArmsUpPotterySherd,
    BladePotterySherd,
    BrewerPotterySherd,
    BurnPotterySherd,
    DangerPotterySherd,
    ExplorerPotterySherd,
    FlowPotterySherd,
    FriendPotterySherd,
    GusterPotterySherd,
    HeartPotterySherd,
    HeartbreakPotterySherd,
    HowlPotterySherd,
    MinerPotterySherd,
    MournerPotterySherd,
    PlentyPotterySherd,
    PrizePotterySherd,
    ScrapePotterySherd,
    SheafPotterySherd,
    ShelterPotterySherd,
    SkullPotterySherd,
    SnortPotterySherd,
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
    TrialSpawner,
    TrialKey,
    OminousTrialKey,
    Vault,
    OminousBottle,
}
pub static AIR: MaterialData = MaterialData { block: None };
pub static STONE: MaterialData = MaterialData {
    block: Some(Block::Stone),
};
pub static GRANITE: MaterialData = MaterialData {
    block: Some(Block::Granite),
};
pub static POLISHED_GRANITE: MaterialData = MaterialData {
    block: Some(Block::PolishedGranite),
};
pub static DIORITE: MaterialData = MaterialData {
    block: Some(Block::Diorite),
};
pub static POLISHED_DIORITE: MaterialData = MaterialData {
    block: Some(Block::PolishedDiorite),
};
pub static ANDESITE: MaterialData = MaterialData {
    block: Some(Block::Andesite),
};
pub static POLISHED_ANDESITE: MaterialData = MaterialData {
    block: Some(Block::PolishedAndesite),
};
pub static DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::Deepslate),
};
pub static COBBLED_DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::CobbledDeepslate),
};
pub static POLISHED_DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::PolishedDeepslate),
};
pub static CALCITE: MaterialData = MaterialData {
    block: Some(Block::Calcite),
};
pub static TUFF: MaterialData = MaterialData {
    block: Some(Block::Tuff),
};
pub static TUFF_SLAB: MaterialData = MaterialData {
    block: Some(Block::TuffSlab),
};
pub static TUFF_STAIRS: MaterialData = MaterialData {
    block: Some(Block::TuffStairs),
};
pub static TUFF_WALL: MaterialData = MaterialData {
    block: Some(Block::TuffWall),
};
pub static CHISELED_TUFF: MaterialData = MaterialData {
    block: Some(Block::ChiseledTuff),
};
pub static POLISHED_TUFF: MaterialData = MaterialData {
    block: Some(Block::PolishedTuff),
};
pub static POLISHED_TUFF_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedTuffSlab),
};
pub static POLISHED_TUFF_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedTuffStairs),
};
pub static POLISHED_TUFF_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedTuffWall),
};
pub static TUFF_BRICKS: MaterialData = MaterialData {
    block: Some(Block::TuffBricks),
};
pub static TUFF_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::TuffBrickSlab),
};
pub static TUFF_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::TuffBrickStairs),
};
pub static TUFF_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::TuffBrickWall),
};
pub static CHISELED_TUFF_BRICKS: MaterialData = MaterialData {
    block: Some(Block::ChiseledTuffBricks),
};
pub static SULFUR: MaterialData = MaterialData {
    block: Some(Block::Sulfur),
};
pub static POTENT_SULFUR: MaterialData = MaterialData {
    block: Some(Block::PotentSulfur),
};
pub static SULFUR_SLAB: MaterialData = MaterialData {
    block: Some(Block::SulfurSlab),
};
pub static SULFUR_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SulfurStairs),
};
pub static SULFUR_WALL: MaterialData = MaterialData {
    block: Some(Block::SulfurWall),
};
pub static POLISHED_SULFUR: MaterialData = MaterialData {
    block: Some(Block::PolishedSulfur),
};
pub static POLISHED_SULFUR_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedSulfurSlab),
};
pub static POLISHED_SULFUR_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedSulfurStairs),
};
pub static POLISHED_SULFUR_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedSulfurWall),
};
pub static SULFUR_BRICKS: MaterialData = MaterialData {
    block: Some(Block::SulfurBricks),
};
pub static SULFUR_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::SulfurBrickSlab),
};
pub static SULFUR_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SulfurBrickStairs),
};
pub static SULFUR_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::SulfurBrickWall),
};
pub static CHISELED_SULFUR: MaterialData = MaterialData {
    block: Some(Block::ChiseledSulfur),
};
pub static CINNABAR: MaterialData = MaterialData {
    block: Some(Block::Cinnabar),
};
pub static CINNABAR_SLAB: MaterialData = MaterialData {
    block: Some(Block::CinnabarSlab),
};
pub static CINNABAR_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CinnabarStairs),
};
pub static CINNABAR_WALL: MaterialData = MaterialData {
    block: Some(Block::CinnabarWall),
};
pub static POLISHED_CINNABAR: MaterialData = MaterialData {
    block: Some(Block::PolishedCinnabar),
};
pub static POLISHED_CINNABAR_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedCinnabarSlab),
};
pub static POLISHED_CINNABAR_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedCinnabarStairs),
};
pub static POLISHED_CINNABAR_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedCinnabarWall),
};
pub static CINNABAR_BRICKS: MaterialData = MaterialData {
    block: Some(Block::CinnabarBricks),
};
pub static CINNABAR_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::CinnabarBrickSlab),
};
pub static CINNABAR_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CinnabarBrickStairs),
};
pub static CINNABAR_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::CinnabarBrickWall),
};
pub static CHISELED_CINNABAR: MaterialData = MaterialData {
    block: Some(Block::ChiseledCinnabar),
};
pub static DRIPSTONE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DripstoneBlock),
};
pub static GRASS_BLOCK: MaterialData = MaterialData {
    block: Some(Block::GrassBlock),
};
pub static DIRT: MaterialData = MaterialData {
    block: Some(Block::Dirt),
};
pub static COARSE_DIRT: MaterialData = MaterialData {
    block: Some(Block::CoarseDirt),
};
pub static PODZOL: MaterialData = MaterialData {
    block: Some(Block::Podzol),
};
pub static ROOTED_DIRT: MaterialData = MaterialData {
    block: Some(Block::RootedDirt),
};
pub static MUD: MaterialData = MaterialData {
    block: Some(Block::Mud),
};
pub static CRIMSON_NYLIUM: MaterialData = MaterialData {
    block: Some(Block::CrimsonNylium),
};
pub static WARPED_NYLIUM: MaterialData = MaterialData {
    block: Some(Block::WarpedNylium),
};
pub static COBBLESTONE: MaterialData = MaterialData {
    block: Some(Block::Cobblestone),
};
pub static OAK_PLANKS: MaterialData = MaterialData {
    block: Some(Block::OakPlanks),
};
pub static SPRUCE_PLANKS: MaterialData = MaterialData {
    block: Some(Block::SprucePlanks),
};
pub static BIRCH_PLANKS: MaterialData = MaterialData {
    block: Some(Block::BirchPlanks),
};
pub static JUNGLE_PLANKS: MaterialData = MaterialData {
    block: Some(Block::JunglePlanks),
};
pub static ACACIA_PLANKS: MaterialData = MaterialData {
    block: Some(Block::AcaciaPlanks),
};
pub static CHERRY_PLANKS: MaterialData = MaterialData {
    block: Some(Block::CherryPlanks),
};
pub static DARK_OAK_PLANKS: MaterialData = MaterialData {
    block: Some(Block::DarkOakPlanks),
};
pub static PALE_OAK_PLANKS: MaterialData = MaterialData {
    block: Some(Block::PaleOakPlanks),
};
pub static MANGROVE_PLANKS: MaterialData = MaterialData {
    block: Some(Block::MangrovePlanks),
};
pub static BAMBOO_PLANKS: MaterialData = MaterialData {
    block: Some(Block::BambooPlanks),
};
pub static CRIMSON_PLANKS: MaterialData = MaterialData {
    block: Some(Block::CrimsonPlanks),
};
pub static WARPED_PLANKS: MaterialData = MaterialData {
    block: Some(Block::WarpedPlanks),
};
pub static BAMBOO_MOSAIC: MaterialData = MaterialData {
    block: Some(Block::BambooMosaic),
};
pub static OAK_SAPLING: MaterialData = MaterialData {
    block: Some(Block::OakSapling),
};
pub static SPRUCE_SAPLING: MaterialData = MaterialData {
    block: Some(Block::SpruceSapling),
};
pub static BIRCH_SAPLING: MaterialData = MaterialData {
    block: Some(Block::BirchSapling),
};
pub static JUNGLE_SAPLING: MaterialData = MaterialData {
    block: Some(Block::JungleSapling),
};
pub static ACACIA_SAPLING: MaterialData = MaterialData {
    block: Some(Block::AcaciaSapling),
};
pub static CHERRY_SAPLING: MaterialData = MaterialData {
    block: Some(Block::CherrySapling),
};
pub static DARK_OAK_SAPLING: MaterialData = MaterialData {
    block: Some(Block::DarkOakSapling),
};
pub static PALE_OAK_SAPLING: MaterialData = MaterialData {
    block: Some(Block::PaleOakSapling),
};
pub static MANGROVE_PROPAGULE: MaterialData = MaterialData {
    block: Some(Block::MangrovePropagule),
};
pub static BEDROCK: MaterialData = MaterialData {
    block: Some(Block::Bedrock),
};
pub static SAND: MaterialData = MaterialData {
    block: Some(Block::Sand),
};
pub static SUSPICIOUS_SAND: MaterialData = MaterialData {
    block: Some(Block::SuspiciousSand),
};
pub static SUSPICIOUS_GRAVEL: MaterialData = MaterialData {
    block: Some(Block::SuspiciousGravel),
};
pub static RED_SAND: MaterialData = MaterialData {
    block: Some(Block::RedSand),
};
pub static GRAVEL: MaterialData = MaterialData {
    block: Some(Block::Gravel),
};
pub static COAL_ORE: MaterialData = MaterialData {
    block: Some(Block::CoalOre),
};
pub static DEEPSLATE_COAL_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateCoalOre),
};
pub static IRON_ORE: MaterialData = MaterialData {
    block: Some(Block::IronOre),
};
pub static DEEPSLATE_IRON_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateIronOre),
};
pub static COPPER_ORE: MaterialData = MaterialData {
    block: Some(Block::CopperOre),
};
pub static DEEPSLATE_COPPER_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateCopperOre),
};
pub static GOLD_ORE: MaterialData = MaterialData {
    block: Some(Block::GoldOre),
};
pub static DEEPSLATE_GOLD_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateGoldOre),
};
pub static REDSTONE_ORE: MaterialData = MaterialData {
    block: Some(Block::RedstoneOre),
};
pub static DEEPSLATE_REDSTONE_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateRedstoneOre),
};
pub static EMERALD_ORE: MaterialData = MaterialData {
    block: Some(Block::EmeraldOre),
};
pub static DEEPSLATE_EMERALD_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateEmeraldOre),
};
pub static LAPIS_ORE: MaterialData = MaterialData {
    block: Some(Block::LapisOre),
};
pub static DEEPSLATE_LAPIS_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateLapisOre),
};
pub static DIAMOND_ORE: MaterialData = MaterialData {
    block: Some(Block::DiamondOre),
};
pub static DEEPSLATE_DIAMOND_ORE: MaterialData = MaterialData {
    block: Some(Block::DeepslateDiamondOre),
};
pub static NETHER_GOLD_ORE: MaterialData = MaterialData {
    block: Some(Block::NetherGoldOre),
};
pub static NETHER_QUARTZ_ORE: MaterialData = MaterialData {
    block: Some(Block::NetherQuartzOre),
};
pub static ANCIENT_DEBRIS: MaterialData = MaterialData {
    block: Some(Block::AncientDebris),
};
pub static COAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::CoalBlock),
};
pub static RAW_IRON_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RawIronBlock),
};
pub static RAW_COPPER_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RawCopperBlock),
};
pub static RAW_GOLD_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RawGoldBlock),
};
pub static HEAVY_CORE: MaterialData = MaterialData {
    block: Some(Block::HeavyCore),
};
pub static AMETHYST_BLOCK: MaterialData = MaterialData {
    block: Some(Block::AmethystBlock),
};
pub static BUDDING_AMETHYST: MaterialData = MaterialData {
    block: Some(Block::BuddingAmethyst),
};
pub static IRON_BLOCK: MaterialData = MaterialData {
    block: Some(Block::IronBlock),
};
pub static COPPER_BLOCK: MaterialData = MaterialData {
    block: Some(Block::CopperBlock),
};
pub static EXPOSED_COPPER: MaterialData = MaterialData {
    block: Some(Block::ExposedCopper),
};
pub static WEATHERED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopper),
};
pub static OXIDIZED_COPPER: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopper),
};
pub static WAXED_COPPER_BLOCK: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperBlock),
};
pub static WAXED_EXPOSED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopper),
};
pub static WAXED_WEATHERED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopper),
};
pub static WAXED_OXIDIZED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopper),
};
pub static GOLD_BLOCK: MaterialData = MaterialData {
    block: Some(Block::GoldBlock),
};
pub static DIAMOND_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DiamondBlock),
};
pub static NETHERITE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::NetheriteBlock),
};
pub static CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::ChiseledCopper),
};
pub static EXPOSED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::ExposedChiseledCopper),
};
pub static WEATHERED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WeatheredChiseledCopper),
};
pub static OXIDIZED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::OxidizedChiseledCopper),
};
pub static WAXED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedChiseledCopper),
};
pub static WAXED_EXPOSED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedChiseledCopper),
};
pub static WAXED_WEATHERED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredChiseledCopper),
};
pub static WAXED_OXIDIZED_CHISELED_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedChiseledCopper),
};
pub static CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::CutCopper),
};
pub static EXPOSED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::ExposedCutCopper),
};
pub static WEATHERED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::WeatheredCutCopper),
};
pub static OXIDIZED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::OxidizedCutCopper),
};
pub static WAXED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedCutCopper),
};
pub static WAXED_EXPOSED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCutCopper),
};
pub static WAXED_WEATHERED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCutCopper),
};
pub static WAXED_OXIDIZED_CUT_COPPER: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCutCopper),
};
pub static CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CutCopperStairs),
};
pub static EXPOSED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::ExposedCutCopperStairs),
};
pub static WEATHERED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WeatheredCutCopperStairs),
};
pub static OXIDIZED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::OxidizedCutCopperStairs),
};
pub static WAXED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WaxedCutCopperStairs),
};
pub static WAXED_EXPOSED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCutCopperStairs),
};
pub static WAXED_WEATHERED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCutCopperStairs),
};
pub static WAXED_OXIDIZED_CUT_COPPER_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCutCopperStairs),
};
pub static CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::CutCopperSlab),
};
pub static EXPOSED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::ExposedCutCopperSlab),
};
pub static WEATHERED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::WeatheredCutCopperSlab),
};
pub static OXIDIZED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::OxidizedCutCopperSlab),
};
pub static WAXED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::WaxedCutCopperSlab),
};
pub static WAXED_EXPOSED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCutCopperSlab),
};
pub static WAXED_WEATHERED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCutCopperSlab),
};
pub static WAXED_OXIDIZED_CUT_COPPER_SLAB: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCutCopperSlab),
};
pub static OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::OakLog),
};
pub static SPRUCE_LOG: MaterialData = MaterialData {
    block: Some(Block::SpruceLog),
};
pub static BIRCH_LOG: MaterialData = MaterialData {
    block: Some(Block::BirchLog),
};
pub static JUNGLE_LOG: MaterialData = MaterialData {
    block: Some(Block::JungleLog),
};
pub static ACACIA_LOG: MaterialData = MaterialData {
    block: Some(Block::AcaciaLog),
};
pub static CHERRY_LOG: MaterialData = MaterialData {
    block: Some(Block::CherryLog),
};
pub static PALE_OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::PaleOakLog),
};
pub static DARK_OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::DarkOakLog),
};
pub static MANGROVE_LOG: MaterialData = MaterialData {
    block: Some(Block::MangroveLog),
};
pub static MANGROVE_ROOTS: MaterialData = MaterialData {
    block: Some(Block::MangroveRoots),
};
pub static MUDDY_MANGROVE_ROOTS: MaterialData = MaterialData {
    block: Some(Block::MuddyMangroveRoots),
};
pub static CRIMSON_STEM: MaterialData = MaterialData {
    block: Some(Block::CrimsonStem),
};
pub static WARPED_STEM: MaterialData = MaterialData {
    block: Some(Block::WarpedStem),
};
pub static BAMBOO_BLOCK: MaterialData = MaterialData {
    block: Some(Block::BambooBlock),
};
pub static STRIPPED_OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedOakLog),
};
pub static STRIPPED_SPRUCE_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedSpruceLog),
};
pub static STRIPPED_BIRCH_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedBirchLog),
};
pub static STRIPPED_JUNGLE_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedJungleLog),
};
pub static STRIPPED_ACACIA_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedAcaciaLog),
};
pub static STRIPPED_CHERRY_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedCherryLog),
};
pub static STRIPPED_DARK_OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedDarkOakLog),
};
pub static STRIPPED_PALE_OAK_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedPaleOakLog),
};
pub static STRIPPED_MANGROVE_LOG: MaterialData = MaterialData {
    block: Some(Block::StrippedMangroveLog),
};
pub static STRIPPED_CRIMSON_STEM: MaterialData = MaterialData {
    block: Some(Block::StrippedCrimsonStem),
};
pub static STRIPPED_WARPED_STEM: MaterialData = MaterialData {
    block: Some(Block::StrippedWarpedStem),
};
pub static STRIPPED_OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedOakWood),
};
pub static STRIPPED_SPRUCE_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedSpruceWood),
};
pub static STRIPPED_BIRCH_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedBirchWood),
};
pub static STRIPPED_JUNGLE_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedJungleWood),
};
pub static STRIPPED_ACACIA_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedAcaciaWood),
};
pub static STRIPPED_CHERRY_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedCherryWood),
};
pub static STRIPPED_DARK_OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedDarkOakWood),
};
pub static STRIPPED_PALE_OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedPaleOakWood),
};
pub static STRIPPED_MANGROVE_WOOD: MaterialData = MaterialData {
    block: Some(Block::StrippedMangroveWood),
};
pub static STRIPPED_CRIMSON_HYPHAE: MaterialData = MaterialData {
    block: Some(Block::StrippedCrimsonHyphae),
};
pub static STRIPPED_WARPED_HYPHAE: MaterialData = MaterialData {
    block: Some(Block::StrippedWarpedHyphae),
};
pub static STRIPPED_BAMBOO_BLOCK: MaterialData = MaterialData {
    block: Some(Block::StrippedBambooBlock),
};
pub static OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::OakWood),
};
pub static SPRUCE_WOOD: MaterialData = MaterialData {
    block: Some(Block::SpruceWood),
};
pub static BIRCH_WOOD: MaterialData = MaterialData {
    block: Some(Block::BirchWood),
};
pub static JUNGLE_WOOD: MaterialData = MaterialData {
    block: Some(Block::JungleWood),
};
pub static ACACIA_WOOD: MaterialData = MaterialData {
    block: Some(Block::AcaciaWood),
};
pub static CHERRY_WOOD: MaterialData = MaterialData {
    block: Some(Block::CherryWood),
};
pub static PALE_OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::PaleOakWood),
};
pub static DARK_OAK_WOOD: MaterialData = MaterialData {
    block: Some(Block::DarkOakWood),
};
pub static MANGROVE_WOOD: MaterialData = MaterialData {
    block: Some(Block::MangroveWood),
};
pub static CRIMSON_HYPHAE: MaterialData = MaterialData {
    block: Some(Block::CrimsonHyphae),
};
pub static WARPED_HYPHAE: MaterialData = MaterialData {
    block: Some(Block::WarpedHyphae),
};
pub static OAK_LEAVES: MaterialData = MaterialData {
    block: Some(Block::OakLeaves),
};
pub static SPRUCE_LEAVES: MaterialData = MaterialData {
    block: Some(Block::SpruceLeaves),
};
pub static BIRCH_LEAVES: MaterialData = MaterialData {
    block: Some(Block::BirchLeaves),
};
pub static JUNGLE_LEAVES: MaterialData = MaterialData {
    block: Some(Block::JungleLeaves),
};
pub static ACACIA_LEAVES: MaterialData = MaterialData {
    block: Some(Block::AcaciaLeaves),
};
pub static CHERRY_LEAVES: MaterialData = MaterialData {
    block: Some(Block::CherryLeaves),
};
pub static DARK_OAK_LEAVES: MaterialData = MaterialData {
    block: Some(Block::DarkOakLeaves),
};
pub static PALE_OAK_LEAVES: MaterialData = MaterialData {
    block: Some(Block::PaleOakLeaves),
};
pub static MANGROVE_LEAVES: MaterialData = MaterialData {
    block: Some(Block::MangroveLeaves),
};
pub static AZALEA_LEAVES: MaterialData = MaterialData {
    block: Some(Block::AzaleaLeaves),
};
pub static FLOWERING_AZALEA_LEAVES: MaterialData = MaterialData {
    block: Some(Block::FloweringAzaleaLeaves),
};
pub static SPONGE: MaterialData = MaterialData {
    block: Some(Block::Sponge),
};
pub static WET_SPONGE: MaterialData = MaterialData {
    block: Some(Block::WetSponge),
};
pub static GLASS: MaterialData = MaterialData {
    block: Some(Block::Glass),
};
pub static TINTED_GLASS: MaterialData = MaterialData {
    block: Some(Block::TintedGlass),
};
pub static LAPIS_BLOCK: MaterialData = MaterialData {
    block: Some(Block::LapisBlock),
};
pub static SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::Sandstone),
};
pub static CHISELED_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::ChiseledSandstone),
};
pub static CUT_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::CutSandstone),
};
pub static COBWEB: MaterialData = MaterialData {
    block: Some(Block::Cobweb),
};
pub static SHORT_GRASS: MaterialData = MaterialData {
    block: Some(Block::ShortGrass),
};
pub static FERN: MaterialData = MaterialData {
    block: Some(Block::Fern),
};
pub static BUSH: MaterialData = MaterialData {
    block: Some(Block::Bush),
};
pub static AZALEA: MaterialData = MaterialData {
    block: Some(Block::Azalea),
};
pub static FLOWERING_AZALEA: MaterialData = MaterialData {
    block: Some(Block::FloweringAzalea),
};
pub static DEAD_BUSH: MaterialData = MaterialData {
    block: Some(Block::DeadBush),
};
pub static FIREFLY_BUSH: MaterialData = MaterialData {
    block: Some(Block::FireflyBush),
};
pub static SHORT_DRY_GRASS: MaterialData = MaterialData {
    block: Some(Block::ShortDryGrass),
};
pub static TALL_DRY_GRASS: MaterialData = MaterialData {
    block: Some(Block::TallDryGrass),
};
pub static SEAGRASS: MaterialData = MaterialData {
    block: Some(Block::Seagrass),
};
pub static SEA_PICKLE: MaterialData = MaterialData {
    block: Some(Block::SeaPickle),
};
pub static WHITE_WOOL: MaterialData = MaterialData {
    block: Some(Block::WhiteWool),
};
pub static ORANGE_WOOL: MaterialData = MaterialData {
    block: Some(Block::OrangeWool),
};
pub static MAGENTA_WOOL: MaterialData = MaterialData {
    block: Some(Block::MagentaWool),
};
pub static LIGHT_BLUE_WOOL: MaterialData = MaterialData {
    block: Some(Block::LightBlueWool),
};
pub static YELLOW_WOOL: MaterialData = MaterialData {
    block: Some(Block::YellowWool),
};
pub static LIME_WOOL: MaterialData = MaterialData {
    block: Some(Block::LimeWool),
};
pub static PINK_WOOL: MaterialData = MaterialData {
    block: Some(Block::PinkWool),
};
pub static GRAY_WOOL: MaterialData = MaterialData {
    block: Some(Block::GrayWool),
};
pub static LIGHT_GRAY_WOOL: MaterialData = MaterialData {
    block: Some(Block::LightGrayWool),
};
pub static CYAN_WOOL: MaterialData = MaterialData {
    block: Some(Block::CyanWool),
};
pub static PURPLE_WOOL: MaterialData = MaterialData {
    block: Some(Block::PurpleWool),
};
pub static BLUE_WOOL: MaterialData = MaterialData {
    block: Some(Block::BlueWool),
};
pub static BROWN_WOOL: MaterialData = MaterialData {
    block: Some(Block::BrownWool),
};
pub static GREEN_WOOL: MaterialData = MaterialData {
    block: Some(Block::GreenWool),
};
pub static RED_WOOL: MaterialData = MaterialData {
    block: Some(Block::RedWool),
};
pub static BLACK_WOOL: MaterialData = MaterialData {
    block: Some(Block::BlackWool),
};
pub static DANDELION: MaterialData = MaterialData {
    block: Some(Block::Dandelion),
};
pub static GOLDEN_DANDELION: MaterialData = MaterialData {
    block: Some(Block::GoldenDandelion),
};
pub static OPEN_EYEBLOSSOM: MaterialData = MaterialData {
    block: Some(Block::OpenEyeblossom),
};
pub static CLOSED_EYEBLOSSOM: MaterialData = MaterialData {
    block: Some(Block::ClosedEyeblossom),
};
pub static POPPY: MaterialData = MaterialData {
    block: Some(Block::Poppy),
};
pub static BLUE_ORCHID: MaterialData = MaterialData {
    block: Some(Block::BlueOrchid),
};
pub static ALLIUM: MaterialData = MaterialData {
    block: Some(Block::Allium),
};
pub static AZURE_BLUET: MaterialData = MaterialData {
    block: Some(Block::AzureBluet),
};
pub static RED_TULIP: MaterialData = MaterialData {
    block: Some(Block::RedTulip),
};
pub static ORANGE_TULIP: MaterialData = MaterialData {
    block: Some(Block::OrangeTulip),
};
pub static WHITE_TULIP: MaterialData = MaterialData {
    block: Some(Block::WhiteTulip),
};
pub static PINK_TULIP: MaterialData = MaterialData {
    block: Some(Block::PinkTulip),
};
pub static OXEYE_DAISY: MaterialData = MaterialData {
    block: Some(Block::OxeyeDaisy),
};
pub static CORNFLOWER: MaterialData = MaterialData {
    block: Some(Block::Cornflower),
};
pub static LILY_OF_THE_VALLEY: MaterialData = MaterialData {
    block: Some(Block::LilyOfTheValley),
};
pub static WITHER_ROSE: MaterialData = MaterialData {
    block: Some(Block::WitherRose),
};
pub static TORCHFLOWER: MaterialData = MaterialData {
    block: Some(Block::Torchflower),
};
pub static PITCHER_PLANT: MaterialData = MaterialData {
    block: Some(Block::PitcherPlant),
};
pub static SPORE_BLOSSOM: MaterialData = MaterialData {
    block: Some(Block::SporeBlossom),
};
pub static BROWN_MUSHROOM: MaterialData = MaterialData {
    block: Some(Block::BrownMushroom),
};
pub static RED_MUSHROOM: MaterialData = MaterialData {
    block: Some(Block::RedMushroom),
};
pub static CRIMSON_FUNGUS: MaterialData = MaterialData {
    block: Some(Block::CrimsonFungus),
};
pub static WARPED_FUNGUS: MaterialData = MaterialData {
    block: Some(Block::WarpedFungus),
};
pub static CRIMSON_ROOTS: MaterialData = MaterialData {
    block: Some(Block::CrimsonRoots),
};
pub static WARPED_ROOTS: MaterialData = MaterialData {
    block: Some(Block::WarpedRoots),
};
pub static NETHER_SPROUTS: MaterialData = MaterialData {
    block: Some(Block::NetherSprouts),
};
pub static WEEPING_VINES: MaterialData = MaterialData {
    block: Some(Block::WeepingVines),
};
pub static TWISTING_VINES: MaterialData = MaterialData {
    block: Some(Block::TwistingVines),
};
pub static SUGAR_CANE: MaterialData = MaterialData {
    block: Some(Block::SugarCane),
};
pub static KELP: MaterialData = MaterialData {
    block: Some(Block::Kelp),
};
pub static PINK_PETALS: MaterialData = MaterialData {
    block: Some(Block::PinkPetals),
};
pub static WILDFLOWERS: MaterialData = MaterialData {
    block: Some(Block::Wildflowers),
};
pub static LEAF_LITTER: MaterialData = MaterialData {
    block: Some(Block::LeafLitter),
};
pub static MOSS_CARPET: MaterialData = MaterialData {
    block: Some(Block::MossCarpet),
};
pub static MOSS_BLOCK: MaterialData = MaterialData {
    block: Some(Block::MossBlock),
};
pub static PALE_MOSS_CARPET: MaterialData = MaterialData {
    block: Some(Block::PaleMossCarpet),
};
pub static PALE_HANGING_MOSS: MaterialData = MaterialData {
    block: Some(Block::PaleHangingMoss),
};
pub static PALE_MOSS_BLOCK: MaterialData = MaterialData {
    block: Some(Block::PaleMossBlock),
};
pub static HANGING_ROOTS: MaterialData = MaterialData {
    block: Some(Block::HangingRoots),
};
pub static BIG_DRIPLEAF: MaterialData = MaterialData {
    block: Some(Block::BigDripleaf),
};
pub static SMALL_DRIPLEAF: MaterialData = MaterialData {
    block: Some(Block::SmallDripleaf),
};
pub static BAMBOO: MaterialData = MaterialData {
    block: Some(Block::Bamboo),
};
pub static OAK_SLAB: MaterialData = MaterialData {
    block: Some(Block::OakSlab),
};
pub static SPRUCE_SLAB: MaterialData = MaterialData {
    block: Some(Block::SpruceSlab),
};
pub static BIRCH_SLAB: MaterialData = MaterialData {
    block: Some(Block::BirchSlab),
};
pub static JUNGLE_SLAB: MaterialData = MaterialData {
    block: Some(Block::JungleSlab),
};
pub static ACACIA_SLAB: MaterialData = MaterialData {
    block: Some(Block::AcaciaSlab),
};
pub static CHERRY_SLAB: MaterialData = MaterialData {
    block: Some(Block::CherrySlab),
};
pub static DARK_OAK_SLAB: MaterialData = MaterialData {
    block: Some(Block::DarkOakSlab),
};
pub static PALE_OAK_SLAB: MaterialData = MaterialData {
    block: Some(Block::PaleOakSlab),
};
pub static MANGROVE_SLAB: MaterialData = MaterialData {
    block: Some(Block::MangroveSlab),
};
pub static BAMBOO_SLAB: MaterialData = MaterialData {
    block: Some(Block::BambooSlab),
};
pub static BAMBOO_MOSAIC_SLAB: MaterialData = MaterialData {
    block: Some(Block::BambooMosaicSlab),
};
pub static CRIMSON_SLAB: MaterialData = MaterialData {
    block: Some(Block::CrimsonSlab),
};
pub static WARPED_SLAB: MaterialData = MaterialData {
    block: Some(Block::WarpedSlab),
};
pub static STONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::StoneSlab),
};
pub static SMOOTH_STONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::SmoothStoneSlab),
};
pub static SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::SandstoneSlab),
};
pub static CUT_SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::CutSandstoneSlab),
};
pub static PETRIFIED_OAK_SLAB: MaterialData = MaterialData {
    block: Some(Block::PetrifiedOakSlab),
};
pub static COBBLESTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::CobblestoneSlab),
};
pub static BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::BrickSlab),
};
pub static STONE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::StoneBrickSlab),
};
pub static MUD_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::MudBrickSlab),
};
pub static NETHER_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::NetherBrickSlab),
};
pub static QUARTZ_SLAB: MaterialData = MaterialData {
    block: Some(Block::QuartzSlab),
};
pub static RED_SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::RedSandstoneSlab),
};
pub static CUT_RED_SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::CutRedSandstoneSlab),
};
pub static PURPUR_SLAB: MaterialData = MaterialData {
    block: Some(Block::PurpurSlab),
};
pub static PRISMARINE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PrismarineSlab),
};
pub static PRISMARINE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::PrismarineBrickSlab),
};
pub static DARK_PRISMARINE_SLAB: MaterialData = MaterialData {
    block: Some(Block::DarkPrismarineSlab),
};
pub static SMOOTH_QUARTZ: MaterialData = MaterialData {
    block: Some(Block::SmoothQuartz),
};
pub static SMOOTH_RED_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::SmoothRedSandstone),
};
pub static SMOOTH_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::SmoothSandstone),
};
pub static SMOOTH_STONE: MaterialData = MaterialData {
    block: Some(Block::SmoothStone),
};
pub static BRICKS: MaterialData = MaterialData {
    block: Some(Block::Bricks),
};
pub static ACACIA_SHELF: MaterialData = MaterialData {
    block: Some(Block::AcaciaShelf),
};
pub static BAMBOO_SHELF: MaterialData = MaterialData {
    block: Some(Block::BambooShelf),
};
pub static BIRCH_SHELF: MaterialData = MaterialData {
    block: Some(Block::BirchShelf),
};
pub static CHERRY_SHELF: MaterialData = MaterialData {
    block: Some(Block::CherryShelf),
};
pub static CRIMSON_SHELF: MaterialData = MaterialData {
    block: Some(Block::CrimsonShelf),
};
pub static DARK_OAK_SHELF: MaterialData = MaterialData {
    block: Some(Block::DarkOakShelf),
};
pub static JUNGLE_SHELF: MaterialData = MaterialData {
    block: Some(Block::JungleShelf),
};
pub static MANGROVE_SHELF: MaterialData = MaterialData {
    block: Some(Block::MangroveShelf),
};
pub static OAK_SHELF: MaterialData = MaterialData {
    block: Some(Block::OakShelf),
};
pub static PALE_OAK_SHELF: MaterialData = MaterialData {
    block: Some(Block::PaleOakShelf),
};
pub static SPRUCE_SHELF: MaterialData = MaterialData {
    block: Some(Block::SpruceShelf),
};
pub static WARPED_SHELF: MaterialData = MaterialData {
    block: Some(Block::WarpedShelf),
};
pub static BOOKSHELF: MaterialData = MaterialData {
    block: Some(Block::Bookshelf),
};
pub static CHISELED_BOOKSHELF: MaterialData = MaterialData {
    block: Some(Block::ChiseledBookshelf),
};
pub static DECORATED_POT: MaterialData = MaterialData {
    block: Some(Block::DecoratedPot),
};
pub static MOSSY_COBBLESTONE: MaterialData = MaterialData {
    block: Some(Block::MossyCobblestone),
};
pub static OBSIDIAN: MaterialData = MaterialData {
    block: Some(Block::Obsidian),
};
pub static TORCH: MaterialData = MaterialData {
    block: Some(Block::Torch),
};
pub static END_ROD: MaterialData = MaterialData {
    block: Some(Block::EndRod),
};
pub static CHORUS_PLANT: MaterialData = MaterialData {
    block: Some(Block::ChorusPlant),
};
pub static CHORUS_FLOWER: MaterialData = MaterialData {
    block: Some(Block::ChorusFlower),
};
pub static PURPUR_BLOCK: MaterialData = MaterialData {
    block: Some(Block::PurpurBlock),
};
pub static PURPUR_PILLAR: MaterialData = MaterialData {
    block: Some(Block::PurpurPillar),
};
pub static PURPUR_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PurpurStairs),
};
pub static SPAWNER: MaterialData = MaterialData {
    block: Some(Block::Spawner),
};
pub static CREAKING_HEART: MaterialData = MaterialData {
    block: Some(Block::CreakingHeart),
};
pub static CHEST: MaterialData = MaterialData {
    block: Some(Block::Chest),
};
pub static CRAFTING_TABLE: MaterialData = MaterialData {
    block: Some(Block::CraftingTable),
};
pub static FARMLAND: MaterialData = MaterialData {
    block: Some(Block::Farmland),
};
pub static FURNACE: MaterialData = MaterialData {
    block: Some(Block::Furnace),
};
pub static LADDER: MaterialData = MaterialData {
    block: Some(Block::Ladder),
};
pub static COBBLESTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CobblestoneStairs),
};
pub static SNOW: MaterialData = MaterialData {
    block: Some(Block::Snow),
};
pub static ICE: MaterialData = MaterialData {
    block: Some(Block::Ice),
};
pub static SNOW_BLOCK: MaterialData = MaterialData {
    block: Some(Block::SnowBlock),
};
pub static CACTUS: MaterialData = MaterialData {
    block: Some(Block::Cactus),
};
pub static CACTUS_FLOWER: MaterialData = MaterialData {
    block: Some(Block::CactusFlower),
};
pub static CLAY: MaterialData = MaterialData {
    block: Some(Block::Clay),
};
pub static JUKEBOX: MaterialData = MaterialData {
    block: Some(Block::Jukebox),
};
pub static OAK_FENCE: MaterialData = MaterialData {
    block: Some(Block::OakFence),
};
pub static SPRUCE_FENCE: MaterialData = MaterialData {
    block: Some(Block::SpruceFence),
};
pub static BIRCH_FENCE: MaterialData = MaterialData {
    block: Some(Block::BirchFence),
};
pub static JUNGLE_FENCE: MaterialData = MaterialData {
    block: Some(Block::JungleFence),
};
pub static ACACIA_FENCE: MaterialData = MaterialData {
    block: Some(Block::AcaciaFence),
};
pub static CHERRY_FENCE: MaterialData = MaterialData {
    block: Some(Block::CherryFence),
};
pub static DARK_OAK_FENCE: MaterialData = MaterialData {
    block: Some(Block::DarkOakFence),
};
pub static PALE_OAK_FENCE: MaterialData = MaterialData {
    block: Some(Block::PaleOakFence),
};
pub static MANGROVE_FENCE: MaterialData = MaterialData {
    block: Some(Block::MangroveFence),
};
pub static BAMBOO_FENCE: MaterialData = MaterialData {
    block: Some(Block::BambooFence),
};
pub static CRIMSON_FENCE: MaterialData = MaterialData {
    block: Some(Block::CrimsonFence),
};
pub static WARPED_FENCE: MaterialData = MaterialData {
    block: Some(Block::WarpedFence),
};
pub static PUMPKIN: MaterialData = MaterialData {
    block: Some(Block::Pumpkin),
};
pub static CARVED_PUMPKIN: MaterialData = MaterialData {
    block: Some(Block::CarvedPumpkin),
};
pub static JACK_O_LANTERN: MaterialData = MaterialData {
    block: Some(Block::JackOLantern),
};
pub static NETHERRACK: MaterialData = MaterialData {
    block: Some(Block::Netherrack),
};
pub static SOUL_SAND: MaterialData = MaterialData {
    block: Some(Block::SoulSand),
};
pub static SOUL_SOIL: MaterialData = MaterialData {
    block: Some(Block::SoulSoil),
};
pub static BASALT: MaterialData = MaterialData {
    block: Some(Block::Basalt),
};
pub static POLISHED_BASALT: MaterialData = MaterialData {
    block: Some(Block::PolishedBasalt),
};
pub static SMOOTH_BASALT: MaterialData = MaterialData {
    block: Some(Block::SmoothBasalt),
};
pub static SOUL_TORCH: MaterialData = MaterialData {
    block: Some(Block::SoulTorch),
};
pub static COPPER_TORCH: MaterialData = MaterialData {
    block: Some(Block::CopperTorch),
};
pub static GLOWSTONE: MaterialData = MaterialData {
    block: Some(Block::Glowstone),
};
pub static INFESTED_STONE: MaterialData = MaterialData {
    block: Some(Block::InfestedStone),
};
pub static INFESTED_COBBLESTONE: MaterialData = MaterialData {
    block: Some(Block::InfestedCobblestone),
};
pub static INFESTED_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::InfestedStoneBricks),
};
pub static INFESTED_MOSSY_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::InfestedMossyStoneBricks),
};
pub static INFESTED_CRACKED_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::InfestedCrackedStoneBricks),
};
pub static INFESTED_CHISELED_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::InfestedChiseledStoneBricks),
};
pub static INFESTED_DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::InfestedDeepslate),
};
pub static STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::StoneBricks),
};
pub static MOSSY_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::MossyStoneBricks),
};
pub static CRACKED_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::CrackedStoneBricks),
};
pub static CHISELED_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::ChiseledStoneBricks),
};
pub static PACKED_MUD: MaterialData = MaterialData {
    block: Some(Block::PackedMud),
};
pub static MUD_BRICKS: MaterialData = MaterialData {
    block: Some(Block::MudBricks),
};
pub static DEEPSLATE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::DeepslateBricks),
};
pub static CRACKED_DEEPSLATE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::CrackedDeepslateBricks),
};
pub static DEEPSLATE_TILES: MaterialData = MaterialData {
    block: Some(Block::DeepslateTiles),
};
pub static CRACKED_DEEPSLATE_TILES: MaterialData = MaterialData {
    block: Some(Block::CrackedDeepslateTiles),
};
pub static CHISELED_DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::ChiseledDeepslate),
};
pub static REINFORCED_DEEPSLATE: MaterialData = MaterialData {
    block: Some(Block::ReinforcedDeepslate),
};
pub static BROWN_MUSHROOM_BLOCK: MaterialData = MaterialData {
    block: Some(Block::BrownMushroomBlock),
};
pub static RED_MUSHROOM_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RedMushroomBlock),
};
pub static MUSHROOM_STEM: MaterialData = MaterialData {
    block: Some(Block::MushroomStem),
};
pub static IRON_BARS: MaterialData = MaterialData {
    block: Some(Block::IronBars),
};
pub static COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::CopperBars),
};
pub static EXPOSED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperBars),
};
pub static WEATHERED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperBars),
};
pub static OXIDIZED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperBars),
};
pub static WAXED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperBars),
};
pub static WAXED_EXPOSED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperBars),
};
pub static WAXED_WEATHERED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperBars),
};
pub static WAXED_OXIDIZED_COPPER_BARS: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperBars),
};
pub static IRON_CHAIN: MaterialData = MaterialData {
    block: Some(Block::IronChain),
};
pub static COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::CopperChain),
};
pub static EXPOSED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperChain),
};
pub static WEATHERED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperChain),
};
pub static OXIDIZED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperChain),
};
pub static WAXED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperChain),
};
pub static WAXED_EXPOSED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperChain),
};
pub static WAXED_WEATHERED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperChain),
};
pub static WAXED_OXIDIZED_COPPER_CHAIN: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperChain),
};
pub static GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::GlassPane),
};
pub static MELON: MaterialData = MaterialData {
    block: Some(Block::Melon),
};
pub static VINE: MaterialData = MaterialData {
    block: Some(Block::Vine),
};
pub static GLOW_LICHEN: MaterialData = MaterialData {
    block: Some(Block::GlowLichen),
};
pub static RESIN_CLUMP: MaterialData = MaterialData {
    block: Some(Block::ResinClump),
};
pub static RESIN_BLOCK: MaterialData = MaterialData {
    block: Some(Block::ResinBlock),
};
pub static RESIN_BRICKS: MaterialData = MaterialData {
    block: Some(Block::ResinBricks),
};
pub static RESIN_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::ResinBrickStairs),
};
pub static RESIN_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::ResinBrickSlab),
};
pub static RESIN_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::ResinBrickWall),
};
pub static CHISELED_RESIN_BRICKS: MaterialData = MaterialData {
    block: Some(Block::ChiseledResinBricks),
};
pub static BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::BrickStairs),
};
pub static STONE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::StoneBrickStairs),
};
pub static MUD_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::MudBrickStairs),
};
pub static MYCELIUM: MaterialData = MaterialData {
    block: Some(Block::Mycelium),
};
pub static LILY_PAD: MaterialData = MaterialData {
    block: Some(Block::LilyPad),
};
pub static NETHER_BRICKS: MaterialData = MaterialData {
    block: Some(Block::NetherBricks),
};
pub static CRACKED_NETHER_BRICKS: MaterialData = MaterialData {
    block: Some(Block::CrackedNetherBricks),
};
pub static CHISELED_NETHER_BRICKS: MaterialData = MaterialData {
    block: Some(Block::ChiseledNetherBricks),
};
pub static NETHER_BRICK_FENCE: MaterialData = MaterialData {
    block: Some(Block::NetherBrickFence),
};
pub static NETHER_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::NetherBrickStairs),
};
pub static SCULK: MaterialData = MaterialData {
    block: Some(Block::Sculk),
};
pub static SCULK_VEIN: MaterialData = MaterialData {
    block: Some(Block::SculkVein),
};
pub static SCULK_CATALYST: MaterialData = MaterialData {
    block: Some(Block::SculkCatalyst),
};
pub static SCULK_SHRIEKER: MaterialData = MaterialData {
    block: Some(Block::SculkShrieker),
};
pub static ENCHANTING_TABLE: MaterialData = MaterialData {
    block: Some(Block::EnchantingTable),
};
pub static END_PORTAL_FRAME: MaterialData = MaterialData {
    block: Some(Block::EndPortalFrame),
};
pub static END_STONE: MaterialData = MaterialData {
    block: Some(Block::EndStone),
};
pub static END_STONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::EndStoneBricks),
};
pub static DRAGON_EGG: MaterialData = MaterialData {
    block: Some(Block::DragonEgg),
};
pub static SANDSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SandstoneStairs),
};
pub static ENDER_CHEST: MaterialData = MaterialData {
    block: Some(Block::EnderChest),
};
pub static EMERALD_BLOCK: MaterialData = MaterialData {
    block: Some(Block::EmeraldBlock),
};
pub static OAK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::OakStairs),
};
pub static SPRUCE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SpruceStairs),
};
pub static BIRCH_STAIRS: MaterialData = MaterialData {
    block: Some(Block::BirchStairs),
};
pub static JUNGLE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::JungleStairs),
};
pub static ACACIA_STAIRS: MaterialData = MaterialData {
    block: Some(Block::AcaciaStairs),
};
pub static CHERRY_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CherryStairs),
};
pub static DARK_OAK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::DarkOakStairs),
};
pub static PALE_OAK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PaleOakStairs),
};
pub static MANGROVE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::MangroveStairs),
};
pub static BAMBOO_STAIRS: MaterialData = MaterialData {
    block: Some(Block::BambooStairs),
};
pub static BAMBOO_MOSAIC_STAIRS: MaterialData = MaterialData {
    block: Some(Block::BambooMosaicStairs),
};
pub static CRIMSON_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CrimsonStairs),
};
pub static WARPED_STAIRS: MaterialData = MaterialData {
    block: Some(Block::WarpedStairs),
};
pub static COMMAND_BLOCK: MaterialData = MaterialData {
    block: Some(Block::CommandBlock),
};
pub static BEACON: MaterialData = MaterialData {
    block: Some(Block::Beacon),
};
pub static COBBLESTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::CobblestoneWall),
};
pub static MOSSY_COBBLESTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::MossyCobblestoneWall),
};
pub static BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::BrickWall),
};
pub static PRISMARINE_WALL: MaterialData = MaterialData {
    block: Some(Block::PrismarineWall),
};
pub static RED_SANDSTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::RedSandstoneWall),
};
pub static MOSSY_STONE_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::MossyStoneBrickWall),
};
pub static GRANITE_WALL: MaterialData = MaterialData {
    block: Some(Block::GraniteWall),
};
pub static STONE_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::StoneBrickWall),
};
pub static MUD_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::MudBrickWall),
};
pub static NETHER_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::NetherBrickWall),
};
pub static ANDESITE_WALL: MaterialData = MaterialData {
    block: Some(Block::AndesiteWall),
};
pub static RED_NETHER_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::RedNetherBrickWall),
};
pub static SANDSTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::SandstoneWall),
};
pub static END_STONE_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::EndStoneBrickWall),
};
pub static DIORITE_WALL: MaterialData = MaterialData {
    block: Some(Block::DioriteWall),
};
pub static BLACKSTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::BlackstoneWall),
};
pub static POLISHED_BLACKSTONE_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneWall),
};
pub static POLISHED_BLACKSTONE_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneBrickWall),
};
pub static COBBLED_DEEPSLATE_WALL: MaterialData = MaterialData {
    block: Some(Block::CobbledDeepslateWall),
};
pub static POLISHED_DEEPSLATE_WALL: MaterialData = MaterialData {
    block: Some(Block::PolishedDeepslateWall),
};
pub static DEEPSLATE_BRICK_WALL: MaterialData = MaterialData {
    block: Some(Block::DeepslateBrickWall),
};
pub static DEEPSLATE_TILE_WALL: MaterialData = MaterialData {
    block: Some(Block::DeepslateTileWall),
};
pub static ANVIL: MaterialData = MaterialData {
    block: Some(Block::Anvil),
};
pub static CHIPPED_ANVIL: MaterialData = MaterialData {
    block: Some(Block::ChippedAnvil),
};
pub static DAMAGED_ANVIL: MaterialData = MaterialData {
    block: Some(Block::DamagedAnvil),
};
pub static CHISELED_QUARTZ_BLOCK: MaterialData = MaterialData {
    block: Some(Block::ChiseledQuartzBlock),
};
pub static QUARTZ_BLOCK: MaterialData = MaterialData {
    block: Some(Block::QuartzBlock),
};
pub static QUARTZ_BRICKS: MaterialData = MaterialData {
    block: Some(Block::QuartzBricks),
};
pub static QUARTZ_PILLAR: MaterialData = MaterialData {
    block: Some(Block::QuartzPillar),
};
pub static QUARTZ_STAIRS: MaterialData = MaterialData {
    block: Some(Block::QuartzStairs),
};
pub static WHITE_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::WhiteTerracotta),
};
pub static ORANGE_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::OrangeTerracotta),
};
pub static MAGENTA_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::MagentaTerracotta),
};
pub static LIGHT_BLUE_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LightBlueTerracotta),
};
pub static YELLOW_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::YellowTerracotta),
};
pub static LIME_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LimeTerracotta),
};
pub static PINK_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::PinkTerracotta),
};
pub static GRAY_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::GrayTerracotta),
};
pub static LIGHT_GRAY_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LightGrayTerracotta),
};
pub static CYAN_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::CyanTerracotta),
};
pub static PURPLE_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::PurpleTerracotta),
};
pub static BLUE_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BlueTerracotta),
};
pub static BROWN_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BrownTerracotta),
};
pub static GREEN_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::GreenTerracotta),
};
pub static RED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::RedTerracotta),
};
pub static BLACK_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BlackTerracotta),
};
pub static BARRIER: MaterialData = MaterialData {
    block: Some(Block::Barrier),
};
pub static LIGHT: MaterialData = MaterialData {
    block: Some(Block::Light),
};
pub static HAY_BLOCK: MaterialData = MaterialData {
    block: Some(Block::HayBlock),
};
pub static WHITE_CARPET: MaterialData = MaterialData {
    block: Some(Block::WhiteCarpet),
};
pub static ORANGE_CARPET: MaterialData = MaterialData {
    block: Some(Block::OrangeCarpet),
};
pub static MAGENTA_CARPET: MaterialData = MaterialData {
    block: Some(Block::MagentaCarpet),
};
pub static LIGHT_BLUE_CARPET: MaterialData = MaterialData {
    block: Some(Block::LightBlueCarpet),
};
pub static YELLOW_CARPET: MaterialData = MaterialData {
    block: Some(Block::YellowCarpet),
};
pub static LIME_CARPET: MaterialData = MaterialData {
    block: Some(Block::LimeCarpet),
};
pub static PINK_CARPET: MaterialData = MaterialData {
    block: Some(Block::PinkCarpet),
};
pub static GRAY_CARPET: MaterialData = MaterialData {
    block: Some(Block::GrayCarpet),
};
pub static LIGHT_GRAY_CARPET: MaterialData = MaterialData {
    block: Some(Block::LightGrayCarpet),
};
pub static CYAN_CARPET: MaterialData = MaterialData {
    block: Some(Block::CyanCarpet),
};
pub static PURPLE_CARPET: MaterialData = MaterialData {
    block: Some(Block::PurpleCarpet),
};
pub static BLUE_CARPET: MaterialData = MaterialData {
    block: Some(Block::BlueCarpet),
};
pub static BROWN_CARPET: MaterialData = MaterialData {
    block: Some(Block::BrownCarpet),
};
pub static GREEN_CARPET: MaterialData = MaterialData {
    block: Some(Block::GreenCarpet),
};
pub static RED_CARPET: MaterialData = MaterialData {
    block: Some(Block::RedCarpet),
};
pub static BLACK_CARPET: MaterialData = MaterialData {
    block: Some(Block::BlackCarpet),
};
pub static TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::Terracotta),
};
pub static PACKED_ICE: MaterialData = MaterialData {
    block: Some(Block::PackedIce),
};
pub static DIRT_PATH: MaterialData = MaterialData {
    block: Some(Block::DirtPath),
};
pub static SUNFLOWER: MaterialData = MaterialData {
    block: Some(Block::Sunflower),
};
pub static LILAC: MaterialData = MaterialData {
    block: Some(Block::Lilac),
};
pub static ROSE_BUSH: MaterialData = MaterialData {
    block: Some(Block::RoseBush),
};
pub static PEONY: MaterialData = MaterialData {
    block: Some(Block::Peony),
};
pub static TALL_GRASS: MaterialData = MaterialData {
    block: Some(Block::TallGrass),
};
pub static LARGE_FERN: MaterialData = MaterialData {
    block: Some(Block::LargeFern),
};
pub static WHITE_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::WhiteStainedGlass),
};
pub static ORANGE_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::OrangeStainedGlass),
};
pub static MAGENTA_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::MagentaStainedGlass),
};
pub static LIGHT_BLUE_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::LightBlueStainedGlass),
};
pub static YELLOW_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::YellowStainedGlass),
};
pub static LIME_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::LimeStainedGlass),
};
pub static PINK_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::PinkStainedGlass),
};
pub static GRAY_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::GrayStainedGlass),
};
pub static LIGHT_GRAY_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::LightGrayStainedGlass),
};
pub static CYAN_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::CyanStainedGlass),
};
pub static PURPLE_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::PurpleStainedGlass),
};
pub static BLUE_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::BlueStainedGlass),
};
pub static BROWN_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::BrownStainedGlass),
};
pub static GREEN_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::GreenStainedGlass),
};
pub static RED_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::RedStainedGlass),
};
pub static BLACK_STAINED_GLASS: MaterialData = MaterialData {
    block: Some(Block::BlackStainedGlass),
};
pub static WHITE_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::WhiteStainedGlassPane),
};
pub static ORANGE_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::OrangeStainedGlassPane),
};
pub static MAGENTA_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::MagentaStainedGlassPane),
};
pub static LIGHT_BLUE_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::LightBlueStainedGlassPane),
};
pub static YELLOW_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::YellowStainedGlassPane),
};
pub static LIME_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::LimeStainedGlassPane),
};
pub static PINK_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::PinkStainedGlassPane),
};
pub static GRAY_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::GrayStainedGlassPane),
};
pub static LIGHT_GRAY_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::LightGrayStainedGlassPane),
};
pub static CYAN_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::CyanStainedGlassPane),
};
pub static PURPLE_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::PurpleStainedGlassPane),
};
pub static BLUE_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::BlueStainedGlassPane),
};
pub static BROWN_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::BrownStainedGlassPane),
};
pub static GREEN_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::GreenStainedGlassPane),
};
pub static RED_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::RedStainedGlassPane),
};
pub static BLACK_STAINED_GLASS_PANE: MaterialData = MaterialData {
    block: Some(Block::BlackStainedGlassPane),
};
pub static PRISMARINE: MaterialData = MaterialData {
    block: Some(Block::Prismarine),
};
pub static PRISMARINE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::PrismarineBricks),
};
pub static DARK_PRISMARINE: MaterialData = MaterialData {
    block: Some(Block::DarkPrismarine),
};
pub static PRISMARINE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PrismarineStairs),
};
pub static PRISMARINE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PrismarineBrickStairs),
};
pub static DARK_PRISMARINE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::DarkPrismarineStairs),
};
pub static SEA_LANTERN: MaterialData = MaterialData {
    block: Some(Block::SeaLantern),
};
pub static RED_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::RedSandstone),
};
pub static CHISELED_RED_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::ChiseledRedSandstone),
};
pub static CUT_RED_SANDSTONE: MaterialData = MaterialData {
    block: Some(Block::CutRedSandstone),
};
pub static RED_SANDSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::RedSandstoneStairs),
};
pub static REPEATING_COMMAND_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RepeatingCommandBlock),
};
pub static CHAIN_COMMAND_BLOCK: MaterialData = MaterialData {
    block: Some(Block::ChainCommandBlock),
};
pub static MAGMA_BLOCK: MaterialData = MaterialData {
    block: Some(Block::MagmaBlock),
};
pub static NETHER_WART_BLOCK: MaterialData = MaterialData {
    block: Some(Block::NetherWartBlock),
};
pub static WARPED_WART_BLOCK: MaterialData = MaterialData {
    block: Some(Block::WarpedWartBlock),
};
pub static RED_NETHER_BRICKS: MaterialData = MaterialData {
    block: Some(Block::RedNetherBricks),
};
pub static BONE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::BoneBlock),
};
pub static STRUCTURE_VOID: MaterialData = MaterialData {
    block: Some(Block::StructureVoid),
};
pub static SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::ShulkerBox),
};
pub static WHITE_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::WhiteShulkerBox),
};
pub static ORANGE_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::OrangeShulkerBox),
};
pub static MAGENTA_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::MagentaShulkerBox),
};
pub static LIGHT_BLUE_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::LightBlueShulkerBox),
};
pub static YELLOW_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::YellowShulkerBox),
};
pub static LIME_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::LimeShulkerBox),
};
pub static PINK_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::PinkShulkerBox),
};
pub static GRAY_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::GrayShulkerBox),
};
pub static LIGHT_GRAY_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::LightGrayShulkerBox),
};
pub static CYAN_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::CyanShulkerBox),
};
pub static PURPLE_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::PurpleShulkerBox),
};
pub static BLUE_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::BlueShulkerBox),
};
pub static BROWN_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::BrownShulkerBox),
};
pub static GREEN_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::GreenShulkerBox),
};
pub static RED_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::RedShulkerBox),
};
pub static BLACK_SHULKER_BOX: MaterialData = MaterialData {
    block: Some(Block::BlackShulkerBox),
};
pub static WHITE_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::WhiteGlazedTerracotta),
};
pub static ORANGE_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::OrangeGlazedTerracotta),
};
pub static MAGENTA_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::MagentaGlazedTerracotta),
};
pub static LIGHT_BLUE_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LightBlueGlazedTerracotta),
};
pub static YELLOW_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::YellowGlazedTerracotta),
};
pub static LIME_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LimeGlazedTerracotta),
};
pub static PINK_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::PinkGlazedTerracotta),
};
pub static GRAY_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::GrayGlazedTerracotta),
};
pub static LIGHT_GRAY_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::LightGrayGlazedTerracotta),
};
pub static CYAN_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::CyanGlazedTerracotta),
};
pub static PURPLE_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::PurpleGlazedTerracotta),
};
pub static BLUE_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BlueGlazedTerracotta),
};
pub static BROWN_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BrownGlazedTerracotta),
};
pub static GREEN_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::GreenGlazedTerracotta),
};
pub static RED_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::RedGlazedTerracotta),
};
pub static BLACK_GLAZED_TERRACOTTA: MaterialData = MaterialData {
    block: Some(Block::BlackGlazedTerracotta),
};
pub static WHITE_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::WhiteConcrete),
};
pub static ORANGE_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::OrangeConcrete),
};
pub static MAGENTA_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::MagentaConcrete),
};
pub static LIGHT_BLUE_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::LightBlueConcrete),
};
pub static YELLOW_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::YellowConcrete),
};
pub static LIME_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::LimeConcrete),
};
pub static PINK_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::PinkConcrete),
};
pub static GRAY_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::GrayConcrete),
};
pub static LIGHT_GRAY_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::LightGrayConcrete),
};
pub static CYAN_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::CyanConcrete),
};
pub static PURPLE_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::PurpleConcrete),
};
pub static BLUE_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::BlueConcrete),
};
pub static BROWN_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::BrownConcrete),
};
pub static GREEN_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::GreenConcrete),
};
pub static RED_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::RedConcrete),
};
pub static BLACK_CONCRETE: MaterialData = MaterialData {
    block: Some(Block::BlackConcrete),
};
pub static WHITE_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::WhiteConcretePowder),
};
pub static ORANGE_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::OrangeConcretePowder),
};
pub static MAGENTA_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::MagentaConcretePowder),
};
pub static LIGHT_BLUE_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::LightBlueConcretePowder),
};
pub static YELLOW_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::YellowConcretePowder),
};
pub static LIME_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::LimeConcretePowder),
};
pub static PINK_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::PinkConcretePowder),
};
pub static GRAY_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::GrayConcretePowder),
};
pub static LIGHT_GRAY_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::LightGrayConcretePowder),
};
pub static CYAN_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::CyanConcretePowder),
};
pub static PURPLE_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::PurpleConcretePowder),
};
pub static BLUE_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::BlueConcretePowder),
};
pub static BROWN_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::BrownConcretePowder),
};
pub static GREEN_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::GreenConcretePowder),
};
pub static RED_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::RedConcretePowder),
};
pub static BLACK_CONCRETE_POWDER: MaterialData = MaterialData {
    block: Some(Block::BlackConcretePowder),
};
pub static TURTLE_EGG: MaterialData = MaterialData {
    block: Some(Block::TurtleEgg),
};
pub static SNIFFER_EGG: MaterialData = MaterialData {
    block: Some(Block::SnifferEgg),
};
pub static DRIED_GHAST: MaterialData = MaterialData {
    block: Some(Block::DriedGhast),
};
pub static DEAD_TUBE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DeadTubeCoralBlock),
};
pub static DEAD_BRAIN_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DeadBrainCoralBlock),
};
pub static DEAD_BUBBLE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DeadBubbleCoralBlock),
};
pub static DEAD_FIRE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DeadFireCoralBlock),
};
pub static DEAD_HORN_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DeadHornCoralBlock),
};
pub static TUBE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::TubeCoralBlock),
};
pub static BRAIN_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::BrainCoralBlock),
};
pub static BUBBLE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::BubbleCoralBlock),
};
pub static FIRE_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::FireCoralBlock),
};
pub static HORN_CORAL_BLOCK: MaterialData = MaterialData {
    block: Some(Block::HornCoralBlock),
};
pub static TUBE_CORAL: MaterialData = MaterialData {
    block: Some(Block::TubeCoral),
};
pub static BRAIN_CORAL: MaterialData = MaterialData {
    block: Some(Block::BrainCoral),
};
pub static BUBBLE_CORAL: MaterialData = MaterialData {
    block: Some(Block::BubbleCoral),
};
pub static FIRE_CORAL: MaterialData = MaterialData {
    block: Some(Block::FireCoral),
};
pub static HORN_CORAL: MaterialData = MaterialData {
    block: Some(Block::HornCoral),
};
pub static DEAD_BRAIN_CORAL: MaterialData = MaterialData {
    block: Some(Block::DeadBrainCoral),
};
pub static DEAD_BUBBLE_CORAL: MaterialData = MaterialData {
    block: Some(Block::DeadBubbleCoral),
};
pub static DEAD_FIRE_CORAL: MaterialData = MaterialData {
    block: Some(Block::DeadFireCoral),
};
pub static DEAD_HORN_CORAL: MaterialData = MaterialData {
    block: Some(Block::DeadHornCoral),
};
pub static DEAD_TUBE_CORAL: MaterialData = MaterialData {
    block: Some(Block::DeadTubeCoral),
};
pub static TUBE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::TubeCoralFan),
};
pub static BRAIN_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::BrainCoralFan),
};
pub static BUBBLE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::BubbleCoralFan),
};
pub static FIRE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::FireCoralFan),
};
pub static HORN_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::HornCoralFan),
};
pub static DEAD_TUBE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::DeadTubeCoralFan),
};
pub static DEAD_BRAIN_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::DeadBrainCoralFan),
};
pub static DEAD_BUBBLE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::DeadBubbleCoralFan),
};
pub static DEAD_FIRE_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::DeadFireCoralFan),
};
pub static DEAD_HORN_CORAL_FAN: MaterialData = MaterialData {
    block: Some(Block::DeadHornCoralFan),
};
pub static BLUE_ICE: MaterialData = MaterialData {
    block: Some(Block::BlueIce),
};
pub static CONDUIT: MaterialData = MaterialData {
    block: Some(Block::Conduit),
};
pub static POLISHED_GRANITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedGraniteStairs),
};
pub static SMOOTH_RED_SANDSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SmoothRedSandstoneStairs),
};
pub static MOSSY_STONE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::MossyStoneBrickStairs),
};
pub static POLISHED_DIORITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedDioriteStairs),
};
pub static MOSSY_COBBLESTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::MossyCobblestoneStairs),
};
pub static END_STONE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::EndStoneBrickStairs),
};
pub static STONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::StoneStairs),
};
pub static SMOOTH_SANDSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SmoothSandstoneStairs),
};
pub static SMOOTH_QUARTZ_STAIRS: MaterialData = MaterialData {
    block: Some(Block::SmoothQuartzStairs),
};
pub static GRANITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::GraniteStairs),
};
pub static ANDESITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::AndesiteStairs),
};
pub static RED_NETHER_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::RedNetherBrickStairs),
};
pub static POLISHED_ANDESITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedAndesiteStairs),
};
pub static DIORITE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::DioriteStairs),
};
pub static COBBLED_DEEPSLATE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::CobbledDeepslateStairs),
};
pub static POLISHED_DEEPSLATE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedDeepslateStairs),
};
pub static DEEPSLATE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::DeepslateBrickStairs),
};
pub static DEEPSLATE_TILE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::DeepslateTileStairs),
};
pub static POLISHED_GRANITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedGraniteSlab),
};
pub static SMOOTH_RED_SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::SmoothRedSandstoneSlab),
};
pub static MOSSY_STONE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::MossyStoneBrickSlab),
};
pub static POLISHED_DIORITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedDioriteSlab),
};
pub static MOSSY_COBBLESTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::MossyCobblestoneSlab),
};
pub static END_STONE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::EndStoneBrickSlab),
};
pub static SMOOTH_SANDSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::SmoothSandstoneSlab),
};
pub static SMOOTH_QUARTZ_SLAB: MaterialData = MaterialData {
    block: Some(Block::SmoothQuartzSlab),
};
pub static GRANITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::GraniteSlab),
};
pub static ANDESITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::AndesiteSlab),
};
pub static RED_NETHER_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::RedNetherBrickSlab),
};
pub static POLISHED_ANDESITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedAndesiteSlab),
};
pub static DIORITE_SLAB: MaterialData = MaterialData {
    block: Some(Block::DioriteSlab),
};
pub static COBBLED_DEEPSLATE_SLAB: MaterialData = MaterialData {
    block: Some(Block::CobbledDeepslateSlab),
};
pub static POLISHED_DEEPSLATE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedDeepslateSlab),
};
pub static DEEPSLATE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::DeepslateBrickSlab),
};
pub static DEEPSLATE_TILE_SLAB: MaterialData = MaterialData {
    block: Some(Block::DeepslateTileSlab),
};
pub static SCAFFOLDING: MaterialData = MaterialData {
    block: Some(Block::Scaffolding),
};
pub static REDSTONE: MaterialData = MaterialData {
    block: Some(Block::RedstoneWire),
};
pub static REDSTONE_TORCH: MaterialData = MaterialData {
    block: Some(Block::RedstoneTorch),
};
pub static REDSTONE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::RedstoneBlock),
};
pub static REPEATER: MaterialData = MaterialData {
    block: Some(Block::Repeater),
};
pub static COMPARATOR: MaterialData = MaterialData {
    block: Some(Block::Comparator),
};
pub static PISTON: MaterialData = MaterialData {
    block: Some(Block::Piston),
};
pub static STICKY_PISTON: MaterialData = MaterialData {
    block: Some(Block::StickyPiston),
};
pub static SLIME_BLOCK: MaterialData = MaterialData {
    block: Some(Block::SlimeBlock),
};
pub static HONEY_BLOCK: MaterialData = MaterialData {
    block: Some(Block::HoneyBlock),
};
pub static OBSERVER: MaterialData = MaterialData {
    block: Some(Block::Observer),
};
pub static HOPPER: MaterialData = MaterialData {
    block: Some(Block::Hopper),
};
pub static DISPENSER: MaterialData = MaterialData {
    block: Some(Block::Dispenser),
};
pub static DROPPER: MaterialData = MaterialData {
    block: Some(Block::Dropper),
};
pub static LECTERN: MaterialData = MaterialData {
    block: Some(Block::Lectern),
};
pub static TARGET: MaterialData = MaterialData {
    block: Some(Block::Target),
};
pub static LEVER: MaterialData = MaterialData {
    block: Some(Block::Lever),
};
pub static LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::LightningRod),
};
pub static EXPOSED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::ExposedLightningRod),
};
pub static WEATHERED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::WeatheredLightningRod),
};
pub static OXIDIZED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::OxidizedLightningRod),
};
pub static WAXED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::WaxedLightningRod),
};
pub static WAXED_EXPOSED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedLightningRod),
};
pub static WAXED_WEATHERED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredLightningRod),
};
pub static WAXED_OXIDIZED_LIGHTNING_ROD: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedLightningRod),
};
pub static DAYLIGHT_DETECTOR: MaterialData = MaterialData {
    block: Some(Block::DaylightDetector),
};
pub static SCULK_SENSOR: MaterialData = MaterialData {
    block: Some(Block::SculkSensor),
};
pub static CALIBRATED_SCULK_SENSOR: MaterialData = MaterialData {
    block: Some(Block::CalibratedSculkSensor),
};
pub static TRIPWIRE_HOOK: MaterialData = MaterialData {
    block: Some(Block::TripwireHook),
};
pub static TRAPPED_CHEST: MaterialData = MaterialData {
    block: Some(Block::TrappedChest),
};
pub static TNT: MaterialData = MaterialData {
    block: Some(Block::Tnt),
};
pub static REDSTONE_LAMP: MaterialData = MaterialData {
    block: Some(Block::RedstoneLamp),
};
pub static NOTE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::NoteBlock),
};
pub static STONE_BUTTON: MaterialData = MaterialData {
    block: Some(Block::StoneButton),
};
pub static POLISHED_BLACKSTONE_BUTTON: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneButton),
};
pub static OAK_BUTTON: MaterialData = MaterialData {
    block: Some(Block::OakButton),
};
pub static SPRUCE_BUTTON: MaterialData = MaterialData {
    block: Some(Block::SpruceButton),
};
pub static BIRCH_BUTTON: MaterialData = MaterialData {
    block: Some(Block::BirchButton),
};
pub static JUNGLE_BUTTON: MaterialData = MaterialData {
    block: Some(Block::JungleButton),
};
pub static ACACIA_BUTTON: MaterialData = MaterialData {
    block: Some(Block::AcaciaButton),
};
pub static CHERRY_BUTTON: MaterialData = MaterialData {
    block: Some(Block::CherryButton),
};
pub static DARK_OAK_BUTTON: MaterialData = MaterialData {
    block: Some(Block::DarkOakButton),
};
pub static PALE_OAK_BUTTON: MaterialData = MaterialData {
    block: Some(Block::PaleOakButton),
};
pub static MANGROVE_BUTTON: MaterialData = MaterialData {
    block: Some(Block::MangroveButton),
};
pub static BAMBOO_BUTTON: MaterialData = MaterialData {
    block: Some(Block::BambooButton),
};
pub static CRIMSON_BUTTON: MaterialData = MaterialData {
    block: Some(Block::CrimsonButton),
};
pub static WARPED_BUTTON: MaterialData = MaterialData {
    block: Some(Block::WarpedButton),
};
pub static STONE_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::StonePressurePlate),
};
pub static POLISHED_BLACKSTONE_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstonePressurePlate),
};
pub static LIGHT_WEIGHTED_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::LightWeightedPressurePlate),
};
pub static HEAVY_WEIGHTED_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::HeavyWeightedPressurePlate),
};
pub static OAK_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::OakPressurePlate),
};
pub static SPRUCE_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::SprucePressurePlate),
};
pub static BIRCH_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::BirchPressurePlate),
};
pub static JUNGLE_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::JunglePressurePlate),
};
pub static ACACIA_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::AcaciaPressurePlate),
};
pub static CHERRY_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::CherryPressurePlate),
};
pub static DARK_OAK_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::DarkOakPressurePlate),
};
pub static PALE_OAK_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::PaleOakPressurePlate),
};
pub static MANGROVE_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::MangrovePressurePlate),
};
pub static BAMBOO_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::BambooPressurePlate),
};
pub static CRIMSON_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::CrimsonPressurePlate),
};
pub static WARPED_PRESSURE_PLATE: MaterialData = MaterialData {
    block: Some(Block::WarpedPressurePlate),
};
pub static IRON_DOOR: MaterialData = MaterialData {
    block: Some(Block::IronDoor),
};
pub static OAK_DOOR: MaterialData = MaterialData {
    block: Some(Block::OakDoor),
};
pub static SPRUCE_DOOR: MaterialData = MaterialData {
    block: Some(Block::SpruceDoor),
};
pub static BIRCH_DOOR: MaterialData = MaterialData {
    block: Some(Block::BirchDoor),
};
pub static JUNGLE_DOOR: MaterialData = MaterialData {
    block: Some(Block::JungleDoor),
};
pub static ACACIA_DOOR: MaterialData = MaterialData {
    block: Some(Block::AcaciaDoor),
};
pub static CHERRY_DOOR: MaterialData = MaterialData {
    block: Some(Block::CherryDoor),
};
pub static DARK_OAK_DOOR: MaterialData = MaterialData {
    block: Some(Block::DarkOakDoor),
};
pub static PALE_OAK_DOOR: MaterialData = MaterialData {
    block: Some(Block::PaleOakDoor),
};
pub static MANGROVE_DOOR: MaterialData = MaterialData {
    block: Some(Block::MangroveDoor),
};
pub static BAMBOO_DOOR: MaterialData = MaterialData {
    block: Some(Block::BambooDoor),
};
pub static CRIMSON_DOOR: MaterialData = MaterialData {
    block: Some(Block::CrimsonDoor),
};
pub static WARPED_DOOR: MaterialData = MaterialData {
    block: Some(Block::WarpedDoor),
};
pub static COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::CopperDoor),
};
pub static EXPOSED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperDoor),
};
pub static WEATHERED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperDoor),
};
pub static OXIDIZED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperDoor),
};
pub static WAXED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperDoor),
};
pub static WAXED_EXPOSED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperDoor),
};
pub static WAXED_WEATHERED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperDoor),
};
pub static WAXED_OXIDIZED_COPPER_DOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperDoor),
};
pub static IRON_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::IronTrapdoor),
};
pub static OAK_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::OakTrapdoor),
};
pub static SPRUCE_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::SpruceTrapdoor),
};
pub static BIRCH_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::BirchTrapdoor),
};
pub static JUNGLE_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::JungleTrapdoor),
};
pub static ACACIA_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::AcaciaTrapdoor),
};
pub static CHERRY_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::CherryTrapdoor),
};
pub static DARK_OAK_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::DarkOakTrapdoor),
};
pub static PALE_OAK_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::PaleOakTrapdoor),
};
pub static MANGROVE_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::MangroveTrapdoor),
};
pub static BAMBOO_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::BambooTrapdoor),
};
pub static CRIMSON_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::CrimsonTrapdoor),
};
pub static WARPED_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WarpedTrapdoor),
};
pub static COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::CopperTrapdoor),
};
pub static EXPOSED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperTrapdoor),
};
pub static WEATHERED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperTrapdoor),
};
pub static OXIDIZED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperTrapdoor),
};
pub static WAXED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperTrapdoor),
};
pub static WAXED_EXPOSED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperTrapdoor),
};
pub static WAXED_WEATHERED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperTrapdoor),
};
pub static WAXED_OXIDIZED_COPPER_TRAPDOOR: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperTrapdoor),
};
pub static OAK_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::OakFenceGate),
};
pub static SPRUCE_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::SpruceFenceGate),
};
pub static BIRCH_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::BirchFenceGate),
};
pub static JUNGLE_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::JungleFenceGate),
};
pub static ACACIA_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::AcaciaFenceGate),
};
pub static CHERRY_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::CherryFenceGate),
};
pub static DARK_OAK_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::DarkOakFenceGate),
};
pub static PALE_OAK_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::PaleOakFenceGate),
};
pub static MANGROVE_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::MangroveFenceGate),
};
pub static BAMBOO_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::BambooFenceGate),
};
pub static CRIMSON_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::CrimsonFenceGate),
};
pub static WARPED_FENCE_GATE: MaterialData = MaterialData {
    block: Some(Block::WarpedFenceGate),
};
pub static POWERED_RAIL: MaterialData = MaterialData {
    block: Some(Block::PoweredRail),
};
pub static DETECTOR_RAIL: MaterialData = MaterialData {
    block: Some(Block::DetectorRail),
};
pub static RAIL: MaterialData = MaterialData {
    block: Some(Block::Rail),
};
pub static ACTIVATOR_RAIL: MaterialData = MaterialData {
    block: Some(Block::ActivatorRail),
};
pub static SADDLE: MaterialData = MaterialData { block: None };
pub static WHITE_HARNESS: MaterialData = MaterialData { block: None };
pub static ORANGE_HARNESS: MaterialData = MaterialData { block: None };
pub static MAGENTA_HARNESS: MaterialData = MaterialData { block: None };
pub static LIGHT_BLUE_HARNESS: MaterialData = MaterialData { block: None };
pub static YELLOW_HARNESS: MaterialData = MaterialData { block: None };
pub static LIME_HARNESS: MaterialData = MaterialData { block: None };
pub static PINK_HARNESS: MaterialData = MaterialData { block: None };
pub static GRAY_HARNESS: MaterialData = MaterialData { block: None };
pub static LIGHT_GRAY_HARNESS: MaterialData = MaterialData { block: None };
pub static CYAN_HARNESS: MaterialData = MaterialData { block: None };
pub static PURPLE_HARNESS: MaterialData = MaterialData { block: None };
pub static BLUE_HARNESS: MaterialData = MaterialData { block: None };
pub static BROWN_HARNESS: MaterialData = MaterialData { block: None };
pub static GREEN_HARNESS: MaterialData = MaterialData { block: None };
pub static RED_HARNESS: MaterialData = MaterialData { block: None };
pub static BLACK_HARNESS: MaterialData = MaterialData { block: None };
pub static MINECART: MaterialData = MaterialData { block: None };
pub static CHEST_MINECART: MaterialData = MaterialData { block: None };
pub static FURNACE_MINECART: MaterialData = MaterialData { block: None };
pub static TNT_MINECART: MaterialData = MaterialData { block: None };
pub static HOPPER_MINECART: MaterialData = MaterialData { block: None };
pub static CARROT_ON_A_STICK: MaterialData = MaterialData { block: None };
pub static WARPED_FUNGUS_ON_A_STICK: MaterialData = MaterialData { block: None };
pub static PHANTOM_MEMBRANE: MaterialData = MaterialData { block: None };
pub static ELYTRA: MaterialData = MaterialData { block: None };
pub static OAK_BOAT: MaterialData = MaterialData { block: None };
pub static OAK_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static SPRUCE_BOAT: MaterialData = MaterialData { block: None };
pub static SPRUCE_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static BIRCH_BOAT: MaterialData = MaterialData { block: None };
pub static BIRCH_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static JUNGLE_BOAT: MaterialData = MaterialData { block: None };
pub static JUNGLE_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static ACACIA_BOAT: MaterialData = MaterialData { block: None };
pub static ACACIA_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static CHERRY_BOAT: MaterialData = MaterialData { block: None };
pub static CHERRY_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static DARK_OAK_BOAT: MaterialData = MaterialData { block: None };
pub static DARK_OAK_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static PALE_OAK_BOAT: MaterialData = MaterialData { block: None };
pub static PALE_OAK_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static MANGROVE_BOAT: MaterialData = MaterialData { block: None };
pub static MANGROVE_CHEST_BOAT: MaterialData = MaterialData { block: None };
pub static BAMBOO_RAFT: MaterialData = MaterialData { block: None };
pub static BAMBOO_CHEST_RAFT: MaterialData = MaterialData { block: None };
pub static STRUCTURE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::StructureBlock),
};
pub static JIGSAW: MaterialData = MaterialData {
    block: Some(Block::Jigsaw),
};
pub static TEST_BLOCK: MaterialData = MaterialData {
    block: Some(Block::TestBlock),
};
pub static TEST_INSTANCE_BLOCK: MaterialData = MaterialData {
    block: Some(Block::TestInstanceBlock),
};
pub static TURTLE_HELMET: MaterialData = MaterialData { block: None };
pub static TURTLE_SCUTE: MaterialData = MaterialData { block: None };
pub static ARMADILLO_SCUTE: MaterialData = MaterialData { block: None };
pub static WOLF_ARMOR: MaterialData = MaterialData { block: None };
pub static FLINT_AND_STEEL: MaterialData = MaterialData { block: None };
pub static BOWL: MaterialData = MaterialData { block: None };
pub static APPLE: MaterialData = MaterialData { block: None };
pub static BOW: MaterialData = MaterialData { block: None };
pub static ARROW: MaterialData = MaterialData { block: None };
pub static COAL: MaterialData = MaterialData { block: None };
pub static CHARCOAL: MaterialData = MaterialData { block: None };
pub static DIAMOND: MaterialData = MaterialData { block: None };
pub static EMERALD: MaterialData = MaterialData { block: None };
pub static LAPIS_LAZULI: MaterialData = MaterialData { block: None };
pub static QUARTZ: MaterialData = MaterialData { block: None };
pub static AMETHYST_SHARD: MaterialData = MaterialData { block: None };
pub static RAW_IRON: MaterialData = MaterialData { block: None };
pub static IRON_INGOT: MaterialData = MaterialData { block: None };
pub static RAW_COPPER: MaterialData = MaterialData { block: None };
pub static COPPER_INGOT: MaterialData = MaterialData { block: None };
pub static RAW_GOLD: MaterialData = MaterialData { block: None };
pub static GOLD_INGOT: MaterialData = MaterialData { block: None };
pub static NETHERITE_INGOT: MaterialData = MaterialData { block: None };
pub static NETHERITE_SCRAP: MaterialData = MaterialData { block: None };
pub static WOODEN_SWORD: MaterialData = MaterialData { block: None };
pub static WOODEN_SHOVEL: MaterialData = MaterialData { block: None };
pub static WOODEN_PICKAXE: MaterialData = MaterialData { block: None };
pub static WOODEN_AXE: MaterialData = MaterialData { block: None };
pub static WOODEN_HOE: MaterialData = MaterialData { block: None };
pub static COPPER_SWORD: MaterialData = MaterialData { block: None };
pub static COPPER_SHOVEL: MaterialData = MaterialData { block: None };
pub static COPPER_PICKAXE: MaterialData = MaterialData { block: None };
pub static COPPER_AXE: MaterialData = MaterialData { block: None };
pub static COPPER_HOE: MaterialData = MaterialData { block: None };
pub static STONE_SWORD: MaterialData = MaterialData { block: None };
pub static STONE_SHOVEL: MaterialData = MaterialData { block: None };
pub static STONE_PICKAXE: MaterialData = MaterialData { block: None };
pub static STONE_AXE: MaterialData = MaterialData { block: None };
pub static STONE_HOE: MaterialData = MaterialData { block: None };
pub static GOLDEN_SWORD: MaterialData = MaterialData { block: None };
pub static GOLDEN_SHOVEL: MaterialData = MaterialData { block: None };
pub static GOLDEN_PICKAXE: MaterialData = MaterialData { block: None };
pub static GOLDEN_AXE: MaterialData = MaterialData { block: None };
pub static GOLDEN_HOE: MaterialData = MaterialData { block: None };
pub static IRON_SWORD: MaterialData = MaterialData { block: None };
pub static IRON_SHOVEL: MaterialData = MaterialData { block: None };
pub static IRON_PICKAXE: MaterialData = MaterialData { block: None };
pub static IRON_AXE: MaterialData = MaterialData { block: None };
pub static IRON_HOE: MaterialData = MaterialData { block: None };
pub static DIAMOND_SWORD: MaterialData = MaterialData { block: None };
pub static DIAMOND_SHOVEL: MaterialData = MaterialData { block: None };
pub static DIAMOND_PICKAXE: MaterialData = MaterialData { block: None };
pub static DIAMOND_AXE: MaterialData = MaterialData { block: None };
pub static DIAMOND_HOE: MaterialData = MaterialData { block: None };
pub static NETHERITE_SWORD: MaterialData = MaterialData { block: None };
pub static NETHERITE_SHOVEL: MaterialData = MaterialData { block: None };
pub static NETHERITE_PICKAXE: MaterialData = MaterialData { block: None };
pub static NETHERITE_AXE: MaterialData = MaterialData { block: None };
pub static NETHERITE_HOE: MaterialData = MaterialData { block: None };
pub static STICK: MaterialData = MaterialData { block: None };
pub static MUSHROOM_STEW: MaterialData = MaterialData { block: None };
pub static STRING: MaterialData = MaterialData {
    block: Some(Block::Tripwire),
};
pub static FEATHER: MaterialData = MaterialData { block: None };
pub static GUNPOWDER: MaterialData = MaterialData { block: None };
pub static WHEAT_SEEDS: MaterialData = MaterialData {
    block: Some(Block::Wheat),
};
pub static WHEAT: MaterialData = MaterialData { block: None };
pub static BREAD: MaterialData = MaterialData { block: None };
pub static LEATHER_HELMET: MaterialData = MaterialData { block: None };
pub static LEATHER_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static LEATHER_LEGGINGS: MaterialData = MaterialData { block: None };
pub static LEATHER_BOOTS: MaterialData = MaterialData { block: None };
pub static COPPER_HELMET: MaterialData = MaterialData { block: None };
pub static COPPER_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static COPPER_LEGGINGS: MaterialData = MaterialData { block: None };
pub static COPPER_BOOTS: MaterialData = MaterialData { block: None };
pub static CHAINMAIL_HELMET: MaterialData = MaterialData { block: None };
pub static CHAINMAIL_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static CHAINMAIL_LEGGINGS: MaterialData = MaterialData { block: None };
pub static CHAINMAIL_BOOTS: MaterialData = MaterialData { block: None };
pub static IRON_HELMET: MaterialData = MaterialData { block: None };
pub static IRON_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static IRON_LEGGINGS: MaterialData = MaterialData { block: None };
pub static IRON_BOOTS: MaterialData = MaterialData { block: None };
pub static DIAMOND_HELMET: MaterialData = MaterialData { block: None };
pub static DIAMOND_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static DIAMOND_LEGGINGS: MaterialData = MaterialData { block: None };
pub static DIAMOND_BOOTS: MaterialData = MaterialData { block: None };
pub static GOLDEN_HELMET: MaterialData = MaterialData { block: None };
pub static GOLDEN_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static GOLDEN_LEGGINGS: MaterialData = MaterialData { block: None };
pub static GOLDEN_BOOTS: MaterialData = MaterialData { block: None };
pub static NETHERITE_HELMET: MaterialData = MaterialData { block: None };
pub static NETHERITE_CHESTPLATE: MaterialData = MaterialData { block: None };
pub static NETHERITE_LEGGINGS: MaterialData = MaterialData { block: None };
pub static NETHERITE_BOOTS: MaterialData = MaterialData { block: None };
pub static FLINT: MaterialData = MaterialData { block: None };
pub static PORKCHOP: MaterialData = MaterialData { block: None };
pub static COOKED_PORKCHOP: MaterialData = MaterialData { block: None };
pub static PAINTING: MaterialData = MaterialData { block: None };
pub static GOLDEN_APPLE: MaterialData = MaterialData { block: None };
pub static ENCHANTED_GOLDEN_APPLE: MaterialData = MaterialData { block: None };
pub static OAK_SIGN: MaterialData = MaterialData {
    block: Some(Block::OakSign),
};
pub static SPRUCE_SIGN: MaterialData = MaterialData {
    block: Some(Block::SpruceSign),
};
pub static BIRCH_SIGN: MaterialData = MaterialData {
    block: Some(Block::BirchSign),
};
pub static JUNGLE_SIGN: MaterialData = MaterialData {
    block: Some(Block::JungleSign),
};
pub static ACACIA_SIGN: MaterialData = MaterialData {
    block: Some(Block::AcaciaSign),
};
pub static CHERRY_SIGN: MaterialData = MaterialData {
    block: Some(Block::CherrySign),
};
pub static DARK_OAK_SIGN: MaterialData = MaterialData {
    block: Some(Block::DarkOakSign),
};
pub static PALE_OAK_SIGN: MaterialData = MaterialData {
    block: Some(Block::PaleOakSign),
};
pub static MANGROVE_SIGN: MaterialData = MaterialData {
    block: Some(Block::MangroveSign),
};
pub static BAMBOO_SIGN: MaterialData = MaterialData {
    block: Some(Block::BambooSign),
};
pub static CRIMSON_SIGN: MaterialData = MaterialData {
    block: Some(Block::CrimsonSign),
};
pub static WARPED_SIGN: MaterialData = MaterialData {
    block: Some(Block::WarpedSign),
};
pub static OAK_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::OakHangingSign),
};
pub static SPRUCE_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::SpruceHangingSign),
};
pub static BIRCH_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::BirchHangingSign),
};
pub static JUNGLE_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::JungleHangingSign),
};
pub static ACACIA_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::AcaciaHangingSign),
};
pub static CHERRY_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::CherryHangingSign),
};
pub static DARK_OAK_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::DarkOakHangingSign),
};
pub static PALE_OAK_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::PaleOakHangingSign),
};
pub static MANGROVE_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::MangroveHangingSign),
};
pub static BAMBOO_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::BambooHangingSign),
};
pub static CRIMSON_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::CrimsonHangingSign),
};
pub static WARPED_HANGING_SIGN: MaterialData = MaterialData {
    block: Some(Block::WarpedHangingSign),
};
pub static BUCKET: MaterialData = MaterialData { block: None };
pub static WATER_BUCKET: MaterialData = MaterialData { block: None };
pub static LAVA_BUCKET: MaterialData = MaterialData { block: None };
pub static POWDER_SNOW_BUCKET: MaterialData = MaterialData {
    block: Some(Block::PowderSnow),
};
pub static SNOWBALL: MaterialData = MaterialData { block: None };
pub static LEATHER: MaterialData = MaterialData { block: None };
pub static MILK_BUCKET: MaterialData = MaterialData { block: None };
pub static PUFFERFISH_BUCKET: MaterialData = MaterialData { block: None };
pub static SALMON_BUCKET: MaterialData = MaterialData { block: None };
pub static COD_BUCKET: MaterialData = MaterialData { block: None };
pub static TROPICAL_FISH_BUCKET: MaterialData = MaterialData { block: None };
pub static AXOLOTL_BUCKET: MaterialData = MaterialData { block: None };
pub static SULFUR_CUBE_BUCKET: MaterialData = MaterialData { block: None };
pub static TADPOLE_BUCKET: MaterialData = MaterialData { block: None };
pub static BRICK: MaterialData = MaterialData { block: None };
pub static CLAY_BALL: MaterialData = MaterialData { block: None };
pub static DRIED_KELP_BLOCK: MaterialData = MaterialData {
    block: Some(Block::DriedKelpBlock),
};
pub static PAPER: MaterialData = MaterialData { block: None };
pub static BOOK: MaterialData = MaterialData { block: None };
pub static SLIME_BALL: MaterialData = MaterialData { block: None };
pub static EGG: MaterialData = MaterialData { block: None };
pub static BLUE_EGG: MaterialData = MaterialData { block: None };
pub static BROWN_EGG: MaterialData = MaterialData { block: None };
pub static COMPASS: MaterialData = MaterialData { block: None };
pub static RECOVERY_COMPASS: MaterialData = MaterialData { block: None };
pub static BUNDLE: MaterialData = MaterialData { block: None };
pub static WHITE_BUNDLE: MaterialData = MaterialData { block: None };
pub static ORANGE_BUNDLE: MaterialData = MaterialData { block: None };
pub static MAGENTA_BUNDLE: MaterialData = MaterialData { block: None };
pub static LIGHT_BLUE_BUNDLE: MaterialData = MaterialData { block: None };
pub static YELLOW_BUNDLE: MaterialData = MaterialData { block: None };
pub static LIME_BUNDLE: MaterialData = MaterialData { block: None };
pub static PINK_BUNDLE: MaterialData = MaterialData { block: None };
pub static GRAY_BUNDLE: MaterialData = MaterialData { block: None };
pub static LIGHT_GRAY_BUNDLE: MaterialData = MaterialData { block: None };
pub static CYAN_BUNDLE: MaterialData = MaterialData { block: None };
pub static PURPLE_BUNDLE: MaterialData = MaterialData { block: None };
pub static BLUE_BUNDLE: MaterialData = MaterialData { block: None };
pub static BROWN_BUNDLE: MaterialData = MaterialData { block: None };
pub static GREEN_BUNDLE: MaterialData = MaterialData { block: None };
pub static RED_BUNDLE: MaterialData = MaterialData { block: None };
pub static BLACK_BUNDLE: MaterialData = MaterialData { block: None };
pub static FISHING_ROD: MaterialData = MaterialData { block: None };
pub static CLOCK: MaterialData = MaterialData { block: None };
pub static SPYGLASS: MaterialData = MaterialData { block: None };
pub static GLOWSTONE_DUST: MaterialData = MaterialData { block: None };
pub static COD: MaterialData = MaterialData { block: None };
pub static SALMON: MaterialData = MaterialData { block: None };
pub static TROPICAL_FISH: MaterialData = MaterialData { block: None };
pub static PUFFERFISH: MaterialData = MaterialData { block: None };
pub static COOKED_COD: MaterialData = MaterialData { block: None };
pub static COOKED_SALMON: MaterialData = MaterialData { block: None };
pub static INK_SAC: MaterialData = MaterialData { block: None };
pub static GLOW_INK_SAC: MaterialData = MaterialData { block: None };
pub static COCOA_BEANS: MaterialData = MaterialData {
    block: Some(Block::Cocoa),
};
pub static WHITE_DYE: MaterialData = MaterialData { block: None };
pub static ORANGE_DYE: MaterialData = MaterialData { block: None };
pub static MAGENTA_DYE: MaterialData = MaterialData { block: None };
pub static LIGHT_BLUE_DYE: MaterialData = MaterialData { block: None };
pub static YELLOW_DYE: MaterialData = MaterialData { block: None };
pub static LIME_DYE: MaterialData = MaterialData { block: None };
pub static PINK_DYE: MaterialData = MaterialData { block: None };
pub static GRAY_DYE: MaterialData = MaterialData { block: None };
pub static LIGHT_GRAY_DYE: MaterialData = MaterialData { block: None };
pub static CYAN_DYE: MaterialData = MaterialData { block: None };
pub static PURPLE_DYE: MaterialData = MaterialData { block: None };
pub static BLUE_DYE: MaterialData = MaterialData { block: None };
pub static BROWN_DYE: MaterialData = MaterialData { block: None };
pub static GREEN_DYE: MaterialData = MaterialData { block: None };
pub static RED_DYE: MaterialData = MaterialData { block: None };
pub static BLACK_DYE: MaterialData = MaterialData { block: None };
pub static BONE_MEAL: MaterialData = MaterialData { block: None };
pub static BONE: MaterialData = MaterialData { block: None };
pub static SUGAR: MaterialData = MaterialData { block: None };
pub static CAKE: MaterialData = MaterialData {
    block: Some(Block::Cake),
};
pub static WHITE_BED: MaterialData = MaterialData {
    block: Some(Block::WhiteBed),
};
pub static ORANGE_BED: MaterialData = MaterialData {
    block: Some(Block::OrangeBed),
};
pub static MAGENTA_BED: MaterialData = MaterialData {
    block: Some(Block::MagentaBed),
};
pub static LIGHT_BLUE_BED: MaterialData = MaterialData {
    block: Some(Block::LightBlueBed),
};
pub static YELLOW_BED: MaterialData = MaterialData {
    block: Some(Block::YellowBed),
};
pub static LIME_BED: MaterialData = MaterialData {
    block: Some(Block::LimeBed),
};
pub static PINK_BED: MaterialData = MaterialData {
    block: Some(Block::PinkBed),
};
pub static GRAY_BED: MaterialData = MaterialData {
    block: Some(Block::GrayBed),
};
pub static LIGHT_GRAY_BED: MaterialData = MaterialData {
    block: Some(Block::LightGrayBed),
};
pub static CYAN_BED: MaterialData = MaterialData {
    block: Some(Block::CyanBed),
};
pub static PURPLE_BED: MaterialData = MaterialData {
    block: Some(Block::PurpleBed),
};
pub static BLUE_BED: MaterialData = MaterialData {
    block: Some(Block::BlueBed),
};
pub static BROWN_BED: MaterialData = MaterialData {
    block: Some(Block::BrownBed),
};
pub static GREEN_BED: MaterialData = MaterialData {
    block: Some(Block::GreenBed),
};
pub static RED_BED: MaterialData = MaterialData {
    block: Some(Block::RedBed),
};
pub static BLACK_BED: MaterialData = MaterialData {
    block: Some(Block::BlackBed),
};
pub static COOKIE: MaterialData = MaterialData { block: None };
pub static CRAFTER: MaterialData = MaterialData {
    block: Some(Block::Crafter),
};
pub static FILLED_MAP: MaterialData = MaterialData { block: None };
pub static SHEARS: MaterialData = MaterialData { block: None };
pub static MELON_SLICE: MaterialData = MaterialData { block: None };
pub static DRIED_KELP: MaterialData = MaterialData { block: None };
pub static PUMPKIN_SEEDS: MaterialData = MaterialData {
    block: Some(Block::PumpkinStem),
};
pub static MELON_SEEDS: MaterialData = MaterialData {
    block: Some(Block::MelonStem),
};
pub static BEEF: MaterialData = MaterialData { block: None };
pub static COOKED_BEEF: MaterialData = MaterialData { block: None };
pub static CHICKEN: MaterialData = MaterialData { block: None };
pub static COOKED_CHICKEN: MaterialData = MaterialData { block: None };
pub static ROTTEN_FLESH: MaterialData = MaterialData { block: None };
pub static ENDER_PEARL: MaterialData = MaterialData { block: None };
pub static BLAZE_ROD: MaterialData = MaterialData { block: None };
pub static GHAST_TEAR: MaterialData = MaterialData { block: None };
pub static GOLD_NUGGET: MaterialData = MaterialData { block: None };
pub static NETHER_WART: MaterialData = MaterialData {
    block: Some(Block::NetherWart),
};
pub static GLASS_BOTTLE: MaterialData = MaterialData { block: None };
pub static POTION: MaterialData = MaterialData { block: None };
pub static SPIDER_EYE: MaterialData = MaterialData { block: None };
pub static FERMENTED_SPIDER_EYE: MaterialData = MaterialData { block: None };
pub static BLAZE_POWDER: MaterialData = MaterialData { block: None };
pub static MAGMA_CREAM: MaterialData = MaterialData { block: None };
pub static BREWING_STAND: MaterialData = MaterialData {
    block: Some(Block::BrewingStand),
};
pub static CAULDRON: MaterialData = MaterialData {
    block: Some(Block::Cauldron),
};
pub static ENDER_EYE: MaterialData = MaterialData { block: None };
pub static GLISTERING_MELON_SLICE: MaterialData = MaterialData { block: None };
pub static CHICKEN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static COW_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PIG_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SHEEP_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CAMEL_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static DONKEY_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static HORSE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static MULE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CAT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PARROT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WOLF_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ARMADILLO_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static BAT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static BEE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static FOX_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static GOAT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static LLAMA_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static OCELOT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PANDA_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static POLAR_BEAR_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static RABBIT_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static AXOLOTL_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static COD_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static DOLPHIN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static FROG_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static GLOW_SQUID_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static NAUTILUS_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PUFFERFISH_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SALMON_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SQUID_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static TADPOLE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static TROPICAL_FISH_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static TURTLE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ALLAY_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static MOOSHROOM_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SNIFFER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SULFUR_CUBE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static COPPER_GOLEM_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static IRON_GOLEM_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SNOW_GOLEM_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static TRADER_LLAMA_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static VILLAGER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WANDERING_TRADER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static BOGGED_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CAMEL_HUSK_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static DROWNED_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static HUSK_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PARCHED_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SKELETON_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SKELETON_HORSE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static STRAY_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WITHER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WITHER_SKELETON_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOMBIE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOMBIE_HORSE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOMBIE_NAUTILUS_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOMBIE_VILLAGER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CAVE_SPIDER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SPIDER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static BREEZE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CREAKING_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static CREEPER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ELDER_GUARDIAN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static GUARDIAN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PHANTOM_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SILVERFISH_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SLIME_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WARDEN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static WITCH_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static EVOKER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PILLAGER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static RAVAGER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static VINDICATOR_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static VEX_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static BLAZE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static GHAST_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static HAPPY_GHAST_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static HOGLIN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static MAGMA_CUBE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PIGLIN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static PIGLIN_BRUTE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static STRIDER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOGLIN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ZOMBIFIED_PIGLIN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ENDER_DRAGON_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ENDERMAN_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static ENDERMITE_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static SHULKER_SPAWN_EGG: MaterialData = MaterialData { block: None };
pub static EXPERIENCE_BOTTLE: MaterialData = MaterialData { block: None };
pub static FIRE_CHARGE: MaterialData = MaterialData { block: None };
pub static WIND_CHARGE: MaterialData = MaterialData { block: None };
pub static WRITABLE_BOOK: MaterialData = MaterialData { block: None };
pub static WRITTEN_BOOK: MaterialData = MaterialData { block: None };
pub static BREEZE_ROD: MaterialData = MaterialData { block: None };
pub static MACE: MaterialData = MaterialData { block: None };
pub static ITEM_FRAME: MaterialData = MaterialData { block: None };
pub static GLOW_ITEM_FRAME: MaterialData = MaterialData { block: None };
pub static FLOWER_POT: MaterialData = MaterialData {
    block: Some(Block::FlowerPot),
};
pub static CARROT: MaterialData = MaterialData {
    block: Some(Block::Carrots),
};
pub static POTATO: MaterialData = MaterialData {
    block: Some(Block::Potatoes),
};
pub static BAKED_POTATO: MaterialData = MaterialData { block: None };
pub static POISONOUS_POTATO: MaterialData = MaterialData { block: None };
pub static MAP: MaterialData = MaterialData { block: None };
pub static GOLDEN_CARROT: MaterialData = MaterialData { block: None };
pub static SKELETON_SKULL: MaterialData = MaterialData {
    block: Some(Block::SkeletonSkull),
};
pub static WITHER_SKELETON_SKULL: MaterialData = MaterialData {
    block: Some(Block::WitherSkeletonSkull),
};
pub static PLAYER_HEAD: MaterialData = MaterialData {
    block: Some(Block::PlayerHead),
};
pub static ZOMBIE_HEAD: MaterialData = MaterialData {
    block: Some(Block::ZombieHead),
};
pub static CREEPER_HEAD: MaterialData = MaterialData {
    block: Some(Block::CreeperHead),
};
pub static DRAGON_HEAD: MaterialData = MaterialData {
    block: Some(Block::DragonHead),
};
pub static PIGLIN_HEAD: MaterialData = MaterialData {
    block: Some(Block::PiglinHead),
};
pub static NETHER_STAR: MaterialData = MaterialData { block: None };
pub static PUMPKIN_PIE: MaterialData = MaterialData { block: None };
pub static FIREWORK_ROCKET: MaterialData = MaterialData { block: None };
pub static FIREWORK_STAR: MaterialData = MaterialData { block: None };
pub static ENCHANTED_BOOK: MaterialData = MaterialData { block: None };
pub static NETHER_BRICK: MaterialData = MaterialData { block: None };
pub static RESIN_BRICK: MaterialData = MaterialData { block: None };
pub static PRISMARINE_SHARD: MaterialData = MaterialData { block: None };
pub static PRISMARINE_CRYSTALS: MaterialData = MaterialData { block: None };
pub static RABBIT: MaterialData = MaterialData { block: None };
pub static COOKED_RABBIT: MaterialData = MaterialData { block: None };
pub static RABBIT_STEW: MaterialData = MaterialData { block: None };
pub static RABBIT_FOOT: MaterialData = MaterialData { block: None };
pub static RABBIT_HIDE: MaterialData = MaterialData { block: None };
pub static ARMOR_STAND: MaterialData = MaterialData { block: None };
pub static COPPER_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static IRON_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static GOLDEN_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static DIAMOND_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static NETHERITE_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static LEATHER_HORSE_ARMOR: MaterialData = MaterialData { block: None };
pub static LEAD: MaterialData = MaterialData { block: None };
pub static NAME_TAG: MaterialData = MaterialData { block: None };
pub static COMMAND_BLOCK_MINECART: MaterialData = MaterialData { block: None };
pub static MUTTON: MaterialData = MaterialData { block: None };
pub static COOKED_MUTTON: MaterialData = MaterialData { block: None };
pub static WHITE_BANNER: MaterialData = MaterialData {
    block: Some(Block::WhiteBanner),
};
pub static ORANGE_BANNER: MaterialData = MaterialData {
    block: Some(Block::OrangeBanner),
};
pub static MAGENTA_BANNER: MaterialData = MaterialData {
    block: Some(Block::MagentaBanner),
};
pub static LIGHT_BLUE_BANNER: MaterialData = MaterialData {
    block: Some(Block::LightBlueBanner),
};
pub static YELLOW_BANNER: MaterialData = MaterialData {
    block: Some(Block::YellowBanner),
};
pub static LIME_BANNER: MaterialData = MaterialData {
    block: Some(Block::LimeBanner),
};
pub static PINK_BANNER: MaterialData = MaterialData {
    block: Some(Block::PinkBanner),
};
pub static GRAY_BANNER: MaterialData = MaterialData {
    block: Some(Block::GrayBanner),
};
pub static LIGHT_GRAY_BANNER: MaterialData = MaterialData {
    block: Some(Block::LightGrayBanner),
};
pub static CYAN_BANNER: MaterialData = MaterialData {
    block: Some(Block::CyanBanner),
};
pub static PURPLE_BANNER: MaterialData = MaterialData {
    block: Some(Block::PurpleBanner),
};
pub static BLUE_BANNER: MaterialData = MaterialData {
    block: Some(Block::BlueBanner),
};
pub static BROWN_BANNER: MaterialData = MaterialData {
    block: Some(Block::BrownBanner),
};
pub static GREEN_BANNER: MaterialData = MaterialData {
    block: Some(Block::GreenBanner),
};
pub static RED_BANNER: MaterialData = MaterialData {
    block: Some(Block::RedBanner),
};
pub static BLACK_BANNER: MaterialData = MaterialData {
    block: Some(Block::BlackBanner),
};
pub static END_CRYSTAL: MaterialData = MaterialData { block: None };
pub static CHORUS_FRUIT: MaterialData = MaterialData { block: None };
pub static POPPED_CHORUS_FRUIT: MaterialData = MaterialData { block: None };
pub static TORCHFLOWER_SEEDS: MaterialData = MaterialData {
    block: Some(Block::TorchflowerCrop),
};
pub static PITCHER_POD: MaterialData = MaterialData {
    block: Some(Block::PitcherCrop),
};
pub static BEETROOT: MaterialData = MaterialData { block: None };
pub static BEETROOT_SEEDS: MaterialData = MaterialData {
    block: Some(Block::Beetroots),
};
pub static BEETROOT_SOUP: MaterialData = MaterialData { block: None };
pub static DRAGON_BREATH: MaterialData = MaterialData { block: None };
pub static SPLASH_POTION: MaterialData = MaterialData { block: None };
pub static SPECTRAL_ARROW: MaterialData = MaterialData { block: None };
pub static TIPPED_ARROW: MaterialData = MaterialData { block: None };
pub static LINGERING_POTION: MaterialData = MaterialData { block: None };
pub static SHIELD: MaterialData = MaterialData { block: None };
pub static WOODEN_SPEAR: MaterialData = MaterialData { block: None };
pub static STONE_SPEAR: MaterialData = MaterialData { block: None };
pub static COPPER_SPEAR: MaterialData = MaterialData { block: None };
pub static IRON_SPEAR: MaterialData = MaterialData { block: None };
pub static GOLDEN_SPEAR: MaterialData = MaterialData { block: None };
pub static DIAMOND_SPEAR: MaterialData = MaterialData { block: None };
pub static NETHERITE_SPEAR: MaterialData = MaterialData { block: None };
pub static TOTEM_OF_UNDYING: MaterialData = MaterialData { block: None };
pub static SHULKER_SHELL: MaterialData = MaterialData { block: None };
pub static IRON_NUGGET: MaterialData = MaterialData { block: None };
pub static COPPER_NUGGET: MaterialData = MaterialData { block: None };
pub static KNOWLEDGE_BOOK: MaterialData = MaterialData { block: None };
pub static DEBUG_STICK: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_13: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_CAT: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_BLOCKS: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_BOUNCE: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_CHIRP: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_CREATOR: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_CREATOR_MUSIC_BOX: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_FAR: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_LAVA_CHICKEN: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_MALL: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_MELLOHI: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_STAL: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_STRAD: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_WARD: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_11: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_WAIT: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_OTHERSIDE: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_RELIC: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_5: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_PIGSTEP: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_PRECIPICE: MaterialData = MaterialData { block: None };
pub static MUSIC_DISC_TEARS: MaterialData = MaterialData { block: None };
pub static DISC_FRAGMENT_5: MaterialData = MaterialData { block: None };
pub static TRIDENT: MaterialData = MaterialData { block: None };
pub static NAUTILUS_SHELL: MaterialData = MaterialData { block: None };
pub static IRON_NAUTILUS_ARMOR: MaterialData = MaterialData { block: None };
pub static GOLDEN_NAUTILUS_ARMOR: MaterialData = MaterialData { block: None };
pub static DIAMOND_NAUTILUS_ARMOR: MaterialData = MaterialData { block: None };
pub static NETHERITE_NAUTILUS_ARMOR: MaterialData = MaterialData { block: None };
pub static COPPER_NAUTILUS_ARMOR: MaterialData = MaterialData { block: None };
pub static HEART_OF_THE_SEA: MaterialData = MaterialData { block: None };
pub static CROSSBOW: MaterialData = MaterialData { block: None };
pub static SUSPICIOUS_STEW: MaterialData = MaterialData { block: None };
pub static LOOM: MaterialData = MaterialData {
    block: Some(Block::Loom),
};
pub static FLOWER_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static CREEPER_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static SKULL_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static MOJANG_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static GLOBE_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static PIGLIN_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static FLOW_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static GUSTER_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static FIELD_MASONED_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static BORDURE_INDENTED_BANNER_PATTERN: MaterialData = MaterialData { block: None };
pub static GOAT_HORN: MaterialData = MaterialData { block: None };
pub static COMPOSTER: MaterialData = MaterialData {
    block: Some(Block::Composter),
};
pub static BARREL: MaterialData = MaterialData {
    block: Some(Block::Barrel),
};
pub static SMOKER: MaterialData = MaterialData {
    block: Some(Block::Smoker),
};
pub static BLAST_FURNACE: MaterialData = MaterialData {
    block: Some(Block::BlastFurnace),
};
pub static CARTOGRAPHY_TABLE: MaterialData = MaterialData {
    block: Some(Block::CartographyTable),
};
pub static FLETCHING_TABLE: MaterialData = MaterialData {
    block: Some(Block::FletchingTable),
};
pub static GRINDSTONE: MaterialData = MaterialData {
    block: Some(Block::Grindstone),
};
pub static SMITHING_TABLE: MaterialData = MaterialData {
    block: Some(Block::SmithingTable),
};
pub static STONECUTTER: MaterialData = MaterialData {
    block: Some(Block::Stonecutter),
};
pub static BELL: MaterialData = MaterialData {
    block: Some(Block::Bell),
};
pub static LANTERN: MaterialData = MaterialData {
    block: Some(Block::Lantern),
};
pub static SOUL_LANTERN: MaterialData = MaterialData {
    block: Some(Block::SoulLantern),
};
pub static COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::CopperLantern),
};
pub static EXPOSED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperLantern),
};
pub static WEATHERED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperLantern),
};
pub static OXIDIZED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperLantern),
};
pub static WAXED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperLantern),
};
pub static WAXED_EXPOSED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperLantern),
};
pub static WAXED_WEATHERED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperLantern),
};
pub static WAXED_OXIDIZED_COPPER_LANTERN: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperLantern),
};
pub static SWEET_BERRIES: MaterialData = MaterialData {
    block: Some(Block::SweetBerryBush),
};
pub static GLOW_BERRIES: MaterialData = MaterialData {
    block: Some(Block::CaveVines),
};
pub static CAMPFIRE: MaterialData = MaterialData {
    block: Some(Block::Campfire),
};
pub static SOUL_CAMPFIRE: MaterialData = MaterialData {
    block: Some(Block::SoulCampfire),
};
pub static SHROOMLIGHT: MaterialData = MaterialData {
    block: Some(Block::Shroomlight),
};
pub static HONEYCOMB: MaterialData = MaterialData { block: None };
pub static BEE_NEST: MaterialData = MaterialData {
    block: Some(Block::BeeNest),
};
pub static BEEHIVE: MaterialData = MaterialData {
    block: Some(Block::Beehive),
};
pub static HONEY_BOTTLE: MaterialData = MaterialData { block: None };
pub static HONEYCOMB_BLOCK: MaterialData = MaterialData {
    block: Some(Block::HoneycombBlock),
};
pub static LODESTONE: MaterialData = MaterialData {
    block: Some(Block::Lodestone),
};
pub static CRYING_OBSIDIAN: MaterialData = MaterialData {
    block: Some(Block::CryingObsidian),
};
pub static BLACKSTONE: MaterialData = MaterialData {
    block: Some(Block::Blackstone),
};
pub static BLACKSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::BlackstoneSlab),
};
pub static BLACKSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::BlackstoneStairs),
};
pub static GILDED_BLACKSTONE: MaterialData = MaterialData {
    block: Some(Block::GildedBlackstone),
};
pub static POLISHED_BLACKSTONE: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstone),
};
pub static POLISHED_BLACKSTONE_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneSlab),
};
pub static POLISHED_BLACKSTONE_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneStairs),
};
pub static CHISELED_POLISHED_BLACKSTONE: MaterialData = MaterialData {
    block: Some(Block::ChiseledPolishedBlackstone),
};
pub static POLISHED_BLACKSTONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneBricks),
};
pub static POLISHED_BLACKSTONE_BRICK_SLAB: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneBrickSlab),
};
pub static POLISHED_BLACKSTONE_BRICK_STAIRS: MaterialData = MaterialData {
    block: Some(Block::PolishedBlackstoneBrickStairs),
};
pub static CRACKED_POLISHED_BLACKSTONE_BRICKS: MaterialData = MaterialData {
    block: Some(Block::CrackedPolishedBlackstoneBricks),
};
pub static RESPAWN_ANCHOR: MaterialData = MaterialData {
    block: Some(Block::RespawnAnchor),
};
pub static CANDLE: MaterialData = MaterialData {
    block: Some(Block::Candle),
};
pub static WHITE_CANDLE: MaterialData = MaterialData {
    block: Some(Block::WhiteCandle),
};
pub static ORANGE_CANDLE: MaterialData = MaterialData {
    block: Some(Block::OrangeCandle),
};
pub static MAGENTA_CANDLE: MaterialData = MaterialData {
    block: Some(Block::MagentaCandle),
};
pub static LIGHT_BLUE_CANDLE: MaterialData = MaterialData {
    block: Some(Block::LightBlueCandle),
};
pub static YELLOW_CANDLE: MaterialData = MaterialData {
    block: Some(Block::YellowCandle),
};
pub static LIME_CANDLE: MaterialData = MaterialData {
    block: Some(Block::LimeCandle),
};
pub static PINK_CANDLE: MaterialData = MaterialData {
    block: Some(Block::PinkCandle),
};
pub static GRAY_CANDLE: MaterialData = MaterialData {
    block: Some(Block::GrayCandle),
};
pub static LIGHT_GRAY_CANDLE: MaterialData = MaterialData {
    block: Some(Block::LightGrayCandle),
};
pub static CYAN_CANDLE: MaterialData = MaterialData {
    block: Some(Block::CyanCandle),
};
pub static PURPLE_CANDLE: MaterialData = MaterialData {
    block: Some(Block::PurpleCandle),
};
pub static BLUE_CANDLE: MaterialData = MaterialData {
    block: Some(Block::BlueCandle),
};
pub static BROWN_CANDLE: MaterialData = MaterialData {
    block: Some(Block::BrownCandle),
};
pub static GREEN_CANDLE: MaterialData = MaterialData {
    block: Some(Block::GreenCandle),
};
pub static RED_CANDLE: MaterialData = MaterialData {
    block: Some(Block::RedCandle),
};
pub static BLACK_CANDLE: MaterialData = MaterialData {
    block: Some(Block::BlackCandle),
};
pub static SMALL_AMETHYST_BUD: MaterialData = MaterialData {
    block: Some(Block::SmallAmethystBud),
};
pub static MEDIUM_AMETHYST_BUD: MaterialData = MaterialData {
    block: Some(Block::MediumAmethystBud),
};
pub static LARGE_AMETHYST_BUD: MaterialData = MaterialData {
    block: Some(Block::LargeAmethystBud),
};
pub static AMETHYST_CLUSTER: MaterialData = MaterialData {
    block: Some(Block::AmethystCluster),
};
pub static POINTED_DRIPSTONE: MaterialData = MaterialData {
    block: Some(Block::PointedDripstone),
};
pub static SULFUR_SPIKE: MaterialData = MaterialData {
    block: Some(Block::SulfurSpike),
};
pub static OCHRE_FROGLIGHT: MaterialData = MaterialData {
    block: Some(Block::OchreFroglight),
};
pub static VERDANT_FROGLIGHT: MaterialData = MaterialData {
    block: Some(Block::VerdantFroglight),
};
pub static PEARLESCENT_FROGLIGHT: MaterialData = MaterialData {
    block: Some(Block::PearlescentFroglight),
};
pub static FROGSPAWN: MaterialData = MaterialData {
    block: Some(Block::Frogspawn),
};
pub static ECHO_SHARD: MaterialData = MaterialData { block: None };
pub static BRUSH: MaterialData = MaterialData { block: None };
pub static NETHERITE_UPGRADE_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static SENTRY_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static DUNE_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static COAST_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static WILD_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static WARD_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static EYE_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static VEX_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static TIDE_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static SNOUT_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static RIB_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static SPIRE_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static WAYFINDER_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static SHAPER_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static SILENCE_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static RAISER_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static HOST_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static FLOW_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static BOLT_ARMOR_TRIM_SMITHING_TEMPLATE: MaterialData = MaterialData { block: None };
pub static ANGLER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static ARCHER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static ARMS_UP_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static BLADE_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static BREWER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static BURN_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static DANGER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static EXPLORER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static FLOW_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static FRIEND_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static GUSTER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static HEART_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static HEARTBREAK_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static HOWL_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static MINER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static MOURNER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static PLENTY_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static PRIZE_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static SCRAPE_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static SHEAF_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static SHELTER_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static SKULL_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static SNORT_POTTERY_SHERD: MaterialData = MaterialData { block: None };
pub static COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::CopperGrate),
};
pub static EXPOSED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperGrate),
};
pub static WEATHERED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperGrate),
};
pub static OXIDIZED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperGrate),
};
pub static WAXED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperGrate),
};
pub static WAXED_EXPOSED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperGrate),
};
pub static WAXED_WEATHERED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperGrate),
};
pub static WAXED_OXIDIZED_COPPER_GRATE: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperGrate),
};
pub static COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::CopperBulb),
};
pub static EXPOSED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperBulb),
};
pub static WEATHERED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperBulb),
};
pub static OXIDIZED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperBulb),
};
pub static WAXED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperBulb),
};
pub static WAXED_EXPOSED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperBulb),
};
pub static WAXED_WEATHERED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperBulb),
};
pub static WAXED_OXIDIZED_COPPER_BULB: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperBulb),
};
pub static COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::CopperChest),
};
pub static EXPOSED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperChest),
};
pub static WEATHERED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperChest),
};
pub static OXIDIZED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperChest),
};
pub static WAXED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperChest),
};
pub static WAXED_EXPOSED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperChest),
};
pub static WAXED_WEATHERED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperChest),
};
pub static WAXED_OXIDIZED_COPPER_CHEST: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperChest),
};
pub static COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::CopperGolemStatue),
};
pub static EXPOSED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::ExposedCopperGolemStatue),
};
pub static WEATHERED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::WeatheredCopperGolemStatue),
};
pub static OXIDIZED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::OxidizedCopperGolemStatue),
};
pub static WAXED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::WaxedCopperGolemStatue),
};
pub static WAXED_EXPOSED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::WaxedExposedCopperGolemStatue),
};
pub static WAXED_WEATHERED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::WaxedWeatheredCopperGolemStatue),
};
pub static WAXED_OXIDIZED_COPPER_GOLEM_STATUE: MaterialData = MaterialData {
    block: Some(Block::WaxedOxidizedCopperGolemStatue),
};
pub static TRIAL_SPAWNER: MaterialData = MaterialData {
    block: Some(Block::TrialSpawner),
};
pub static TRIAL_KEY: MaterialData = MaterialData { block: None };
pub static OMINOUS_TRIAL_KEY: MaterialData = MaterialData { block: None };
pub static VAULT: MaterialData = MaterialData {
    block: Some(Block::Vault),
};
pub static OMINOUS_BOTTLE: MaterialData = MaterialData { block: None };
pub(crate) fn register_all(registry: &mut Registry<Material>) {
    let mut register = |key: &'static str, value: Material| {
        Registry::register(registry, key.into(), value);
    };
    register("minecraft:air", Material::Air);
    register("minecraft:stone", Material::Stone);
    register("minecraft:granite", Material::Granite);
    register("minecraft:polished_granite", Material::PolishedGranite);
    register("minecraft:diorite", Material::Diorite);
    register("minecraft:polished_diorite", Material::PolishedDiorite);
    register("minecraft:andesite", Material::Andesite);
    register("minecraft:polished_andesite", Material::PolishedAndesite);
    register("minecraft:deepslate", Material::Deepslate);
    register("minecraft:cobbled_deepslate", Material::CobbledDeepslate);
    register("minecraft:polished_deepslate", Material::PolishedDeepslate);
    register("minecraft:calcite", Material::Calcite);
    register("minecraft:tuff", Material::Tuff);
    register("minecraft:tuff_slab", Material::TuffSlab);
    register("minecraft:tuff_stairs", Material::TuffStairs);
    register("minecraft:tuff_wall", Material::TuffWall);
    register("minecraft:chiseled_tuff", Material::ChiseledTuff);
    register("minecraft:polished_tuff", Material::PolishedTuff);
    register("minecraft:polished_tuff_slab", Material::PolishedTuffSlab);
    register(
        "minecraft:polished_tuff_stairs",
        Material::PolishedTuffStairs,
    );
    register("minecraft:polished_tuff_wall", Material::PolishedTuffWall);
    register("minecraft:tuff_bricks", Material::TuffBricks);
    register("minecraft:tuff_brick_slab", Material::TuffBrickSlab);
    register("minecraft:tuff_brick_stairs", Material::TuffBrickStairs);
    register("minecraft:tuff_brick_wall", Material::TuffBrickWall);
    register(
        "minecraft:chiseled_tuff_bricks",
        Material::ChiseledTuffBricks,
    );
    register("minecraft:sulfur", Material::Sulfur);
    register("minecraft:potent_sulfur", Material::PotentSulfur);
    register("minecraft:sulfur_slab", Material::SulfurSlab);
    register("minecraft:sulfur_stairs", Material::SulfurStairs);
    register("minecraft:sulfur_wall", Material::SulfurWall);
    register("minecraft:polished_sulfur", Material::PolishedSulfur);
    register(
        "minecraft:polished_sulfur_slab",
        Material::PolishedSulfurSlab,
    );
    register(
        "minecraft:polished_sulfur_stairs",
        Material::PolishedSulfurStairs,
    );
    register(
        "minecraft:polished_sulfur_wall",
        Material::PolishedSulfurWall,
    );
    register("minecraft:sulfur_bricks", Material::SulfurBricks);
    register("minecraft:sulfur_brick_slab", Material::SulfurBrickSlab);
    register("minecraft:sulfur_brick_stairs", Material::SulfurBrickStairs);
    register("minecraft:sulfur_brick_wall", Material::SulfurBrickWall);
    register("minecraft:chiseled_sulfur", Material::ChiseledSulfur);
    register("minecraft:cinnabar", Material::Cinnabar);
    register("minecraft:cinnabar_slab", Material::CinnabarSlab);
    register("minecraft:cinnabar_stairs", Material::CinnabarStairs);
    register("minecraft:cinnabar_wall", Material::CinnabarWall);
    register("minecraft:polished_cinnabar", Material::PolishedCinnabar);
    register(
        "minecraft:polished_cinnabar_slab",
        Material::PolishedCinnabarSlab,
    );
    register(
        "minecraft:polished_cinnabar_stairs",
        Material::PolishedCinnabarStairs,
    );
    register(
        "minecraft:polished_cinnabar_wall",
        Material::PolishedCinnabarWall,
    );
    register("minecraft:cinnabar_bricks", Material::CinnabarBricks);
    register("minecraft:cinnabar_brick_slab", Material::CinnabarBrickSlab);
    register(
        "minecraft:cinnabar_brick_stairs",
        Material::CinnabarBrickStairs,
    );
    register("minecraft:cinnabar_brick_wall", Material::CinnabarBrickWall);
    register("minecraft:chiseled_cinnabar", Material::ChiseledCinnabar);
    register("minecraft:dripstone_block", Material::DripstoneBlock);
    register("minecraft:grass_block", Material::GrassBlock);
    register("minecraft:dirt", Material::Dirt);
    register("minecraft:coarse_dirt", Material::CoarseDirt);
    register("minecraft:podzol", Material::Podzol);
    register("minecraft:rooted_dirt", Material::RootedDirt);
    register("minecraft:mud", Material::Mud);
    register("minecraft:crimson_nylium", Material::CrimsonNylium);
    register("minecraft:warped_nylium", Material::WarpedNylium);
    register("minecraft:cobblestone", Material::Cobblestone);
    register("minecraft:oak_planks", Material::OakPlanks);
    register("minecraft:spruce_planks", Material::SprucePlanks);
    register("minecraft:birch_planks", Material::BirchPlanks);
    register("minecraft:jungle_planks", Material::JunglePlanks);
    register("minecraft:acacia_planks", Material::AcaciaPlanks);
    register("minecraft:cherry_planks", Material::CherryPlanks);
    register("minecraft:dark_oak_planks", Material::DarkOakPlanks);
    register("minecraft:pale_oak_planks", Material::PaleOakPlanks);
    register("minecraft:mangrove_planks", Material::MangrovePlanks);
    register("minecraft:bamboo_planks", Material::BambooPlanks);
    register("minecraft:crimson_planks", Material::CrimsonPlanks);
    register("minecraft:warped_planks", Material::WarpedPlanks);
    register("minecraft:bamboo_mosaic", Material::BambooMosaic);
    register("minecraft:oak_sapling", Material::OakSapling);
    register("minecraft:spruce_sapling", Material::SpruceSapling);
    register("minecraft:birch_sapling", Material::BirchSapling);
    register("minecraft:jungle_sapling", Material::JungleSapling);
    register("minecraft:acacia_sapling", Material::AcaciaSapling);
    register("minecraft:cherry_sapling", Material::CherrySapling);
    register("minecraft:dark_oak_sapling", Material::DarkOakSapling);
    register("minecraft:pale_oak_sapling", Material::PaleOakSapling);
    register("minecraft:mangrove_propagule", Material::MangrovePropagule);
    register("minecraft:bedrock", Material::Bedrock);
    register("minecraft:sand", Material::Sand);
    register("minecraft:suspicious_sand", Material::SuspiciousSand);
    register("minecraft:suspicious_gravel", Material::SuspiciousGravel);
    register("minecraft:red_sand", Material::RedSand);
    register("minecraft:gravel", Material::Gravel);
    register("minecraft:coal_ore", Material::CoalOre);
    register("minecraft:deepslate_coal_ore", Material::DeepslateCoalOre);
    register("minecraft:iron_ore", Material::IronOre);
    register("minecraft:deepslate_iron_ore", Material::DeepslateIronOre);
    register("minecraft:copper_ore", Material::CopperOre);
    register(
        "minecraft:deepslate_copper_ore",
        Material::DeepslateCopperOre,
    );
    register("minecraft:gold_ore", Material::GoldOre);
    register("minecraft:deepslate_gold_ore", Material::DeepslateGoldOre);
    register("minecraft:redstone_ore", Material::RedstoneOre);
    register(
        "minecraft:deepslate_redstone_ore",
        Material::DeepslateRedstoneOre,
    );
    register("minecraft:emerald_ore", Material::EmeraldOre);
    register(
        "minecraft:deepslate_emerald_ore",
        Material::DeepslateEmeraldOre,
    );
    register("minecraft:lapis_ore", Material::LapisOre);
    register("minecraft:deepslate_lapis_ore", Material::DeepslateLapisOre);
    register("minecraft:diamond_ore", Material::DiamondOre);
    register(
        "minecraft:deepslate_diamond_ore",
        Material::DeepslateDiamondOre,
    );
    register("minecraft:nether_gold_ore", Material::NetherGoldOre);
    register("minecraft:nether_quartz_ore", Material::NetherQuartzOre);
    register("minecraft:ancient_debris", Material::AncientDebris);
    register("minecraft:coal_block", Material::CoalBlock);
    register("minecraft:raw_iron_block", Material::RawIronBlock);
    register("minecraft:raw_copper_block", Material::RawCopperBlock);
    register("minecraft:raw_gold_block", Material::RawGoldBlock);
    register("minecraft:heavy_core", Material::HeavyCore);
    register("minecraft:amethyst_block", Material::AmethystBlock);
    register("minecraft:budding_amethyst", Material::BuddingAmethyst);
    register("minecraft:iron_block", Material::IronBlock);
    register("minecraft:copper_block", Material::CopperBlock);
    register("minecraft:exposed_copper", Material::ExposedCopper);
    register("minecraft:weathered_copper", Material::WeatheredCopper);
    register("minecraft:oxidized_copper", Material::OxidizedCopper);
    register("minecraft:waxed_copper_block", Material::WaxedCopperBlock);
    register(
        "minecraft:waxed_exposed_copper",
        Material::WaxedExposedCopper,
    );
    register(
        "minecraft:waxed_weathered_copper",
        Material::WaxedWeatheredCopper,
    );
    register(
        "minecraft:waxed_oxidized_copper",
        Material::WaxedOxidizedCopper,
    );
    register("minecraft:gold_block", Material::GoldBlock);
    register("minecraft:diamond_block", Material::DiamondBlock);
    register("minecraft:netherite_block", Material::NetheriteBlock);
    register("minecraft:chiseled_copper", Material::ChiseledCopper);
    register(
        "minecraft:exposed_chiseled_copper",
        Material::ExposedChiseledCopper,
    );
    register(
        "minecraft:weathered_chiseled_copper",
        Material::WeatheredChiseledCopper,
    );
    register(
        "minecraft:oxidized_chiseled_copper",
        Material::OxidizedChiseledCopper,
    );
    register(
        "minecraft:waxed_chiseled_copper",
        Material::WaxedChiseledCopper,
    );
    register(
        "minecraft:waxed_exposed_chiseled_copper",
        Material::WaxedExposedChiseledCopper,
    );
    register(
        "minecraft:waxed_weathered_chiseled_copper",
        Material::WaxedWeatheredChiseledCopper,
    );
    register(
        "minecraft:waxed_oxidized_chiseled_copper",
        Material::WaxedOxidizedChiseledCopper,
    );
    register("minecraft:cut_copper", Material::CutCopper);
    register("minecraft:exposed_cut_copper", Material::ExposedCutCopper);
    register(
        "minecraft:weathered_cut_copper",
        Material::WeatheredCutCopper,
    );
    register("minecraft:oxidized_cut_copper", Material::OxidizedCutCopper);
    register("minecraft:waxed_cut_copper", Material::WaxedCutCopper);
    register(
        "minecraft:waxed_exposed_cut_copper",
        Material::WaxedExposedCutCopper,
    );
    register(
        "minecraft:waxed_weathered_cut_copper",
        Material::WaxedWeatheredCutCopper,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper",
        Material::WaxedOxidizedCutCopper,
    );
    register("minecraft:cut_copper_stairs", Material::CutCopperStairs);
    register(
        "minecraft:exposed_cut_copper_stairs",
        Material::ExposedCutCopperStairs,
    );
    register(
        "minecraft:weathered_cut_copper_stairs",
        Material::WeatheredCutCopperStairs,
    );
    register(
        "minecraft:oxidized_cut_copper_stairs",
        Material::OxidizedCutCopperStairs,
    );
    register(
        "minecraft:waxed_cut_copper_stairs",
        Material::WaxedCutCopperStairs,
    );
    register(
        "minecraft:waxed_exposed_cut_copper_stairs",
        Material::WaxedExposedCutCopperStairs,
    );
    register(
        "minecraft:waxed_weathered_cut_copper_stairs",
        Material::WaxedWeatheredCutCopperStairs,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper_stairs",
        Material::WaxedOxidizedCutCopperStairs,
    );
    register("minecraft:cut_copper_slab", Material::CutCopperSlab);
    register(
        "minecraft:exposed_cut_copper_slab",
        Material::ExposedCutCopperSlab,
    );
    register(
        "minecraft:weathered_cut_copper_slab",
        Material::WeatheredCutCopperSlab,
    );
    register(
        "minecraft:oxidized_cut_copper_slab",
        Material::OxidizedCutCopperSlab,
    );
    register(
        "minecraft:waxed_cut_copper_slab",
        Material::WaxedCutCopperSlab,
    );
    register(
        "minecraft:waxed_exposed_cut_copper_slab",
        Material::WaxedExposedCutCopperSlab,
    );
    register(
        "minecraft:waxed_weathered_cut_copper_slab",
        Material::WaxedWeatheredCutCopperSlab,
    );
    register(
        "minecraft:waxed_oxidized_cut_copper_slab",
        Material::WaxedOxidizedCutCopperSlab,
    );
    register("minecraft:oak_log", Material::OakLog);
    register("minecraft:spruce_log", Material::SpruceLog);
    register("minecraft:birch_log", Material::BirchLog);
    register("minecraft:jungle_log", Material::JungleLog);
    register("minecraft:acacia_log", Material::AcaciaLog);
    register("minecraft:cherry_log", Material::CherryLog);
    register("minecraft:pale_oak_log", Material::PaleOakLog);
    register("minecraft:dark_oak_log", Material::DarkOakLog);
    register("minecraft:mangrove_log", Material::MangroveLog);
    register("minecraft:mangrove_roots", Material::MangroveRoots);
    register(
        "minecraft:muddy_mangrove_roots",
        Material::MuddyMangroveRoots,
    );
    register("minecraft:crimson_stem", Material::CrimsonStem);
    register("minecraft:warped_stem", Material::WarpedStem);
    register("minecraft:bamboo_block", Material::BambooBlock);
    register("minecraft:stripped_oak_log", Material::StrippedOakLog);
    register("minecraft:stripped_spruce_log", Material::StrippedSpruceLog);
    register("minecraft:stripped_birch_log", Material::StrippedBirchLog);
    register("minecraft:stripped_jungle_log", Material::StrippedJungleLog);
    register("minecraft:stripped_acacia_log", Material::StrippedAcaciaLog);
    register("minecraft:stripped_cherry_log", Material::StrippedCherryLog);
    register(
        "minecraft:stripped_dark_oak_log",
        Material::StrippedDarkOakLog,
    );
    register(
        "minecraft:stripped_pale_oak_log",
        Material::StrippedPaleOakLog,
    );
    register(
        "minecraft:stripped_mangrove_log",
        Material::StrippedMangroveLog,
    );
    register(
        "minecraft:stripped_crimson_stem",
        Material::StrippedCrimsonStem,
    );
    register(
        "minecraft:stripped_warped_stem",
        Material::StrippedWarpedStem,
    );
    register("minecraft:stripped_oak_wood", Material::StrippedOakWood);
    register(
        "minecraft:stripped_spruce_wood",
        Material::StrippedSpruceWood,
    );
    register("minecraft:stripped_birch_wood", Material::StrippedBirchWood);
    register(
        "minecraft:stripped_jungle_wood",
        Material::StrippedJungleWood,
    );
    register(
        "minecraft:stripped_acacia_wood",
        Material::StrippedAcaciaWood,
    );
    register(
        "minecraft:stripped_cherry_wood",
        Material::StrippedCherryWood,
    );
    register(
        "minecraft:stripped_dark_oak_wood",
        Material::StrippedDarkOakWood,
    );
    register(
        "minecraft:stripped_pale_oak_wood",
        Material::StrippedPaleOakWood,
    );
    register(
        "minecraft:stripped_mangrove_wood",
        Material::StrippedMangroveWood,
    );
    register(
        "minecraft:stripped_crimson_hyphae",
        Material::StrippedCrimsonHyphae,
    );
    register(
        "minecraft:stripped_warped_hyphae",
        Material::StrippedWarpedHyphae,
    );
    register(
        "minecraft:stripped_bamboo_block",
        Material::StrippedBambooBlock,
    );
    register("minecraft:oak_wood", Material::OakWood);
    register("minecraft:spruce_wood", Material::SpruceWood);
    register("minecraft:birch_wood", Material::BirchWood);
    register("minecraft:jungle_wood", Material::JungleWood);
    register("minecraft:acacia_wood", Material::AcaciaWood);
    register("minecraft:cherry_wood", Material::CherryWood);
    register("minecraft:pale_oak_wood", Material::PaleOakWood);
    register("minecraft:dark_oak_wood", Material::DarkOakWood);
    register("minecraft:mangrove_wood", Material::MangroveWood);
    register("minecraft:crimson_hyphae", Material::CrimsonHyphae);
    register("minecraft:warped_hyphae", Material::WarpedHyphae);
    register("minecraft:oak_leaves", Material::OakLeaves);
    register("minecraft:spruce_leaves", Material::SpruceLeaves);
    register("minecraft:birch_leaves", Material::BirchLeaves);
    register("minecraft:jungle_leaves", Material::JungleLeaves);
    register("minecraft:acacia_leaves", Material::AcaciaLeaves);
    register("minecraft:cherry_leaves", Material::CherryLeaves);
    register("minecraft:dark_oak_leaves", Material::DarkOakLeaves);
    register("minecraft:pale_oak_leaves", Material::PaleOakLeaves);
    register("minecraft:mangrove_leaves", Material::MangroveLeaves);
    register("minecraft:azalea_leaves", Material::AzaleaLeaves);
    register(
        "minecraft:flowering_azalea_leaves",
        Material::FloweringAzaleaLeaves,
    );
    register("minecraft:sponge", Material::Sponge);
    register("minecraft:wet_sponge", Material::WetSponge);
    register("minecraft:glass", Material::Glass);
    register("minecraft:tinted_glass", Material::TintedGlass);
    register("minecraft:lapis_block", Material::LapisBlock);
    register("minecraft:sandstone", Material::Sandstone);
    register("minecraft:chiseled_sandstone", Material::ChiseledSandstone);
    register("minecraft:cut_sandstone", Material::CutSandstone);
    register("minecraft:cobweb", Material::Cobweb);
    register("minecraft:short_grass", Material::ShortGrass);
    register("minecraft:fern", Material::Fern);
    register("minecraft:bush", Material::Bush);
    register("minecraft:azalea", Material::Azalea);
    register("minecraft:flowering_azalea", Material::FloweringAzalea);
    register("minecraft:dead_bush", Material::DeadBush);
    register("minecraft:firefly_bush", Material::FireflyBush);
    register("minecraft:short_dry_grass", Material::ShortDryGrass);
    register("minecraft:tall_dry_grass", Material::TallDryGrass);
    register("minecraft:seagrass", Material::Seagrass);
    register("minecraft:sea_pickle", Material::SeaPickle);
    register("minecraft:white_wool", Material::WhiteWool);
    register("minecraft:orange_wool", Material::OrangeWool);
    register("minecraft:magenta_wool", Material::MagentaWool);
    register("minecraft:light_blue_wool", Material::LightBlueWool);
    register("minecraft:yellow_wool", Material::YellowWool);
    register("minecraft:lime_wool", Material::LimeWool);
    register("minecraft:pink_wool", Material::PinkWool);
    register("minecraft:gray_wool", Material::GrayWool);
    register("minecraft:light_gray_wool", Material::LightGrayWool);
    register("minecraft:cyan_wool", Material::CyanWool);
    register("minecraft:purple_wool", Material::PurpleWool);
    register("minecraft:blue_wool", Material::BlueWool);
    register("minecraft:brown_wool", Material::BrownWool);
    register("minecraft:green_wool", Material::GreenWool);
    register("minecraft:red_wool", Material::RedWool);
    register("minecraft:black_wool", Material::BlackWool);
    register("minecraft:dandelion", Material::Dandelion);
    register("minecraft:golden_dandelion", Material::GoldenDandelion);
    register("minecraft:open_eyeblossom", Material::OpenEyeblossom);
    register("minecraft:closed_eyeblossom", Material::ClosedEyeblossom);
    register("minecraft:poppy", Material::Poppy);
    register("minecraft:blue_orchid", Material::BlueOrchid);
    register("minecraft:allium", Material::Allium);
    register("minecraft:azure_bluet", Material::AzureBluet);
    register("minecraft:red_tulip", Material::RedTulip);
    register("minecraft:orange_tulip", Material::OrangeTulip);
    register("minecraft:white_tulip", Material::WhiteTulip);
    register("minecraft:pink_tulip", Material::PinkTulip);
    register("minecraft:oxeye_daisy", Material::OxeyeDaisy);
    register("minecraft:cornflower", Material::Cornflower);
    register("minecraft:lily_of_the_valley", Material::LilyOfTheValley);
    register("minecraft:wither_rose", Material::WitherRose);
    register("minecraft:torchflower", Material::Torchflower);
    register("minecraft:pitcher_plant", Material::PitcherPlant);
    register("minecraft:spore_blossom", Material::SporeBlossom);
    register("minecraft:brown_mushroom", Material::BrownMushroom);
    register("minecraft:red_mushroom", Material::RedMushroom);
    register("minecraft:crimson_fungus", Material::CrimsonFungus);
    register("minecraft:warped_fungus", Material::WarpedFungus);
    register("minecraft:crimson_roots", Material::CrimsonRoots);
    register("minecraft:warped_roots", Material::WarpedRoots);
    register("minecraft:nether_sprouts", Material::NetherSprouts);
    register("minecraft:weeping_vines", Material::WeepingVines);
    register("minecraft:twisting_vines", Material::TwistingVines);
    register("minecraft:sugar_cane", Material::SugarCane);
    register("minecraft:kelp", Material::Kelp);
    register("minecraft:pink_petals", Material::PinkPetals);
    register("minecraft:wildflowers", Material::Wildflowers);
    register("minecraft:leaf_litter", Material::LeafLitter);
    register("minecraft:moss_carpet", Material::MossCarpet);
    register("minecraft:moss_block", Material::MossBlock);
    register("minecraft:pale_moss_carpet", Material::PaleMossCarpet);
    register("minecraft:pale_hanging_moss", Material::PaleHangingMoss);
    register("minecraft:pale_moss_block", Material::PaleMossBlock);
    register("minecraft:hanging_roots", Material::HangingRoots);
    register("minecraft:big_dripleaf", Material::BigDripleaf);
    register("minecraft:small_dripleaf", Material::SmallDripleaf);
    register("minecraft:bamboo", Material::Bamboo);
    register("minecraft:oak_slab", Material::OakSlab);
    register("minecraft:spruce_slab", Material::SpruceSlab);
    register("minecraft:birch_slab", Material::BirchSlab);
    register("minecraft:jungle_slab", Material::JungleSlab);
    register("minecraft:acacia_slab", Material::AcaciaSlab);
    register("minecraft:cherry_slab", Material::CherrySlab);
    register("minecraft:dark_oak_slab", Material::DarkOakSlab);
    register("minecraft:pale_oak_slab", Material::PaleOakSlab);
    register("minecraft:mangrove_slab", Material::MangroveSlab);
    register("minecraft:bamboo_slab", Material::BambooSlab);
    register("minecraft:bamboo_mosaic_slab", Material::BambooMosaicSlab);
    register("minecraft:crimson_slab", Material::CrimsonSlab);
    register("minecraft:warped_slab", Material::WarpedSlab);
    register("minecraft:stone_slab", Material::StoneSlab);
    register("minecraft:smooth_stone_slab", Material::SmoothStoneSlab);
    register("minecraft:sandstone_slab", Material::SandstoneSlab);
    register("minecraft:cut_sandstone_slab", Material::CutSandstoneSlab);
    register("minecraft:petrified_oak_slab", Material::PetrifiedOakSlab);
    register("minecraft:cobblestone_slab", Material::CobblestoneSlab);
    register("minecraft:brick_slab", Material::BrickSlab);
    register("minecraft:stone_brick_slab", Material::StoneBrickSlab);
    register("minecraft:mud_brick_slab", Material::MudBrickSlab);
    register("minecraft:nether_brick_slab", Material::NetherBrickSlab);
    register("minecraft:quartz_slab", Material::QuartzSlab);
    register("minecraft:red_sandstone_slab", Material::RedSandstoneSlab);
    register(
        "minecraft:cut_red_sandstone_slab",
        Material::CutRedSandstoneSlab,
    );
    register("minecraft:purpur_slab", Material::PurpurSlab);
    register("minecraft:prismarine_slab", Material::PrismarineSlab);
    register(
        "minecraft:prismarine_brick_slab",
        Material::PrismarineBrickSlab,
    );
    register(
        "minecraft:dark_prismarine_slab",
        Material::DarkPrismarineSlab,
    );
    register("minecraft:smooth_quartz", Material::SmoothQuartz);
    register(
        "minecraft:smooth_red_sandstone",
        Material::SmoothRedSandstone,
    );
    register("minecraft:smooth_sandstone", Material::SmoothSandstone);
    register("minecraft:smooth_stone", Material::SmoothStone);
    register("minecraft:bricks", Material::Bricks);
    register("minecraft:acacia_shelf", Material::AcaciaShelf);
    register("minecraft:bamboo_shelf", Material::BambooShelf);
    register("minecraft:birch_shelf", Material::BirchShelf);
    register("minecraft:cherry_shelf", Material::CherryShelf);
    register("minecraft:crimson_shelf", Material::CrimsonShelf);
    register("minecraft:dark_oak_shelf", Material::DarkOakShelf);
    register("minecraft:jungle_shelf", Material::JungleShelf);
    register("minecraft:mangrove_shelf", Material::MangroveShelf);
    register("minecraft:oak_shelf", Material::OakShelf);
    register("minecraft:pale_oak_shelf", Material::PaleOakShelf);
    register("minecraft:spruce_shelf", Material::SpruceShelf);
    register("minecraft:warped_shelf", Material::WarpedShelf);
    register("minecraft:bookshelf", Material::Bookshelf);
    register("minecraft:chiseled_bookshelf", Material::ChiseledBookshelf);
    register("minecraft:decorated_pot", Material::DecoratedPot);
    register("minecraft:mossy_cobblestone", Material::MossyCobblestone);
    register("minecraft:obsidian", Material::Obsidian);
    register("minecraft:torch", Material::Torch);
    register("minecraft:end_rod", Material::EndRod);
    register("minecraft:chorus_plant", Material::ChorusPlant);
    register("minecraft:chorus_flower", Material::ChorusFlower);
    register("minecraft:purpur_block", Material::PurpurBlock);
    register("minecraft:purpur_pillar", Material::PurpurPillar);
    register("minecraft:purpur_stairs", Material::PurpurStairs);
    register("minecraft:spawner", Material::Spawner);
    register("minecraft:creaking_heart", Material::CreakingHeart);
    register("minecraft:chest", Material::Chest);
    register("minecraft:crafting_table", Material::CraftingTable);
    register("minecraft:farmland", Material::Farmland);
    register("minecraft:furnace", Material::Furnace);
    register("minecraft:ladder", Material::Ladder);
    register("minecraft:cobblestone_stairs", Material::CobblestoneStairs);
    register("minecraft:snow", Material::Snow);
    register("minecraft:ice", Material::Ice);
    register("minecraft:snow_block", Material::SnowBlock);
    register("minecraft:cactus", Material::Cactus);
    register("minecraft:cactus_flower", Material::CactusFlower);
    register("minecraft:clay", Material::Clay);
    register("minecraft:jukebox", Material::Jukebox);
    register("minecraft:oak_fence", Material::OakFence);
    register("minecraft:spruce_fence", Material::SpruceFence);
    register("minecraft:birch_fence", Material::BirchFence);
    register("minecraft:jungle_fence", Material::JungleFence);
    register("minecraft:acacia_fence", Material::AcaciaFence);
    register("minecraft:cherry_fence", Material::CherryFence);
    register("minecraft:dark_oak_fence", Material::DarkOakFence);
    register("minecraft:pale_oak_fence", Material::PaleOakFence);
    register("minecraft:mangrove_fence", Material::MangroveFence);
    register("minecraft:bamboo_fence", Material::BambooFence);
    register("minecraft:crimson_fence", Material::CrimsonFence);
    register("minecraft:warped_fence", Material::WarpedFence);
    register("minecraft:pumpkin", Material::Pumpkin);
    register("minecraft:carved_pumpkin", Material::CarvedPumpkin);
    register("minecraft:jack_o_lantern", Material::JackOLantern);
    register("minecraft:netherrack", Material::Netherrack);
    register("minecraft:soul_sand", Material::SoulSand);
    register("minecraft:soul_soil", Material::SoulSoil);
    register("minecraft:basalt", Material::Basalt);
    register("minecraft:polished_basalt", Material::PolishedBasalt);
    register("minecraft:smooth_basalt", Material::SmoothBasalt);
    register("minecraft:soul_torch", Material::SoulTorch);
    register("minecraft:copper_torch", Material::CopperTorch);
    register("minecraft:glowstone", Material::Glowstone);
    register("minecraft:infested_stone", Material::InfestedStone);
    register(
        "minecraft:infested_cobblestone",
        Material::InfestedCobblestone,
    );
    register(
        "minecraft:infested_stone_bricks",
        Material::InfestedStoneBricks,
    );
    register(
        "minecraft:infested_mossy_stone_bricks",
        Material::InfestedMossyStoneBricks,
    );
    register(
        "minecraft:infested_cracked_stone_bricks",
        Material::InfestedCrackedStoneBricks,
    );
    register(
        "minecraft:infested_chiseled_stone_bricks",
        Material::InfestedChiseledStoneBricks,
    );
    register("minecraft:infested_deepslate", Material::InfestedDeepslate);
    register("minecraft:stone_bricks", Material::StoneBricks);
    register("minecraft:mossy_stone_bricks", Material::MossyStoneBricks);
    register(
        "minecraft:cracked_stone_bricks",
        Material::CrackedStoneBricks,
    );
    register(
        "minecraft:chiseled_stone_bricks",
        Material::ChiseledStoneBricks,
    );
    register("minecraft:packed_mud", Material::PackedMud);
    register("minecraft:mud_bricks", Material::MudBricks);
    register("minecraft:deepslate_bricks", Material::DeepslateBricks);
    register(
        "minecraft:cracked_deepslate_bricks",
        Material::CrackedDeepslateBricks,
    );
    register("minecraft:deepslate_tiles", Material::DeepslateTiles);
    register(
        "minecraft:cracked_deepslate_tiles",
        Material::CrackedDeepslateTiles,
    );
    register("minecraft:chiseled_deepslate", Material::ChiseledDeepslate);
    register(
        "minecraft:reinforced_deepslate",
        Material::ReinforcedDeepslate,
    );
    register(
        "minecraft:brown_mushroom_block",
        Material::BrownMushroomBlock,
    );
    register("minecraft:red_mushroom_block", Material::RedMushroomBlock);
    register("minecraft:mushroom_stem", Material::MushroomStem);
    register("minecraft:iron_bars", Material::IronBars);
    register("minecraft:copper_bars", Material::CopperBars);
    register("minecraft:exposed_copper_bars", Material::ExposedCopperBars);
    register(
        "minecraft:weathered_copper_bars",
        Material::WeatheredCopperBars,
    );
    register(
        "minecraft:oxidized_copper_bars",
        Material::OxidizedCopperBars,
    );
    register("minecraft:waxed_copper_bars", Material::WaxedCopperBars);
    register(
        "minecraft:waxed_exposed_copper_bars",
        Material::WaxedExposedCopperBars,
    );
    register(
        "minecraft:waxed_weathered_copper_bars",
        Material::WaxedWeatheredCopperBars,
    );
    register(
        "minecraft:waxed_oxidized_copper_bars",
        Material::WaxedOxidizedCopperBars,
    );
    register("minecraft:iron_chain", Material::IronChain);
    register("minecraft:copper_chain", Material::CopperChain);
    register(
        "minecraft:exposed_copper_chain",
        Material::ExposedCopperChain,
    );
    register(
        "minecraft:weathered_copper_chain",
        Material::WeatheredCopperChain,
    );
    register(
        "minecraft:oxidized_copper_chain",
        Material::OxidizedCopperChain,
    );
    register("minecraft:waxed_copper_chain", Material::WaxedCopperChain);
    register(
        "minecraft:waxed_exposed_copper_chain",
        Material::WaxedExposedCopperChain,
    );
    register(
        "minecraft:waxed_weathered_copper_chain",
        Material::WaxedWeatheredCopperChain,
    );
    register(
        "minecraft:waxed_oxidized_copper_chain",
        Material::WaxedOxidizedCopperChain,
    );
    register("minecraft:glass_pane", Material::GlassPane);
    register("minecraft:melon", Material::Melon);
    register("minecraft:vine", Material::Vine);
    register("minecraft:glow_lichen", Material::GlowLichen);
    register("minecraft:resin_clump", Material::ResinClump);
    register("minecraft:resin_block", Material::ResinBlock);
    register("minecraft:resin_bricks", Material::ResinBricks);
    register("minecraft:resin_brick_stairs", Material::ResinBrickStairs);
    register("minecraft:resin_brick_slab", Material::ResinBrickSlab);
    register("minecraft:resin_brick_wall", Material::ResinBrickWall);
    register(
        "minecraft:chiseled_resin_bricks",
        Material::ChiseledResinBricks,
    );
    register("minecraft:brick_stairs", Material::BrickStairs);
    register("minecraft:stone_brick_stairs", Material::StoneBrickStairs);
    register("minecraft:mud_brick_stairs", Material::MudBrickStairs);
    register("minecraft:mycelium", Material::Mycelium);
    register("minecraft:lily_pad", Material::LilyPad);
    register("minecraft:nether_bricks", Material::NetherBricks);
    register(
        "minecraft:cracked_nether_bricks",
        Material::CrackedNetherBricks,
    );
    register(
        "minecraft:chiseled_nether_bricks",
        Material::ChiseledNetherBricks,
    );
    register("minecraft:nether_brick_fence", Material::NetherBrickFence);
    register("minecraft:nether_brick_stairs", Material::NetherBrickStairs);
    register("minecraft:sculk", Material::Sculk);
    register("minecraft:sculk_vein", Material::SculkVein);
    register("minecraft:sculk_catalyst", Material::SculkCatalyst);
    register("minecraft:sculk_shrieker", Material::SculkShrieker);
    register("minecraft:enchanting_table", Material::EnchantingTable);
    register("minecraft:end_portal_frame", Material::EndPortalFrame);
    register("minecraft:end_stone", Material::EndStone);
    register("minecraft:end_stone_bricks", Material::EndStoneBricks);
    register("minecraft:dragon_egg", Material::DragonEgg);
    register("minecraft:sandstone_stairs", Material::SandstoneStairs);
    register("minecraft:ender_chest", Material::EnderChest);
    register("minecraft:emerald_block", Material::EmeraldBlock);
    register("minecraft:oak_stairs", Material::OakStairs);
    register("minecraft:spruce_stairs", Material::SpruceStairs);
    register("minecraft:birch_stairs", Material::BirchStairs);
    register("minecraft:jungle_stairs", Material::JungleStairs);
    register("minecraft:acacia_stairs", Material::AcaciaStairs);
    register("minecraft:cherry_stairs", Material::CherryStairs);
    register("minecraft:dark_oak_stairs", Material::DarkOakStairs);
    register("minecraft:pale_oak_stairs", Material::PaleOakStairs);
    register("minecraft:mangrove_stairs", Material::MangroveStairs);
    register("minecraft:bamboo_stairs", Material::BambooStairs);
    register(
        "minecraft:bamboo_mosaic_stairs",
        Material::BambooMosaicStairs,
    );
    register("minecraft:crimson_stairs", Material::CrimsonStairs);
    register("minecraft:warped_stairs", Material::WarpedStairs);
    register("minecraft:command_block", Material::CommandBlock);
    register("minecraft:beacon", Material::Beacon);
    register("minecraft:cobblestone_wall", Material::CobblestoneWall);
    register(
        "minecraft:mossy_cobblestone_wall",
        Material::MossyCobblestoneWall,
    );
    register("minecraft:brick_wall", Material::BrickWall);
    register("minecraft:prismarine_wall", Material::PrismarineWall);
    register("minecraft:red_sandstone_wall", Material::RedSandstoneWall);
    register(
        "minecraft:mossy_stone_brick_wall",
        Material::MossyStoneBrickWall,
    );
    register("minecraft:granite_wall", Material::GraniteWall);
    register("minecraft:stone_brick_wall", Material::StoneBrickWall);
    register("minecraft:mud_brick_wall", Material::MudBrickWall);
    register("minecraft:nether_brick_wall", Material::NetherBrickWall);
    register("minecraft:andesite_wall", Material::AndesiteWall);
    register(
        "minecraft:red_nether_brick_wall",
        Material::RedNetherBrickWall,
    );
    register("minecraft:sandstone_wall", Material::SandstoneWall);
    register(
        "minecraft:end_stone_brick_wall",
        Material::EndStoneBrickWall,
    );
    register("minecraft:diorite_wall", Material::DioriteWall);
    register("minecraft:blackstone_wall", Material::BlackstoneWall);
    register(
        "minecraft:polished_blackstone_wall",
        Material::PolishedBlackstoneWall,
    );
    register(
        "minecraft:polished_blackstone_brick_wall",
        Material::PolishedBlackstoneBrickWall,
    );
    register(
        "minecraft:cobbled_deepslate_wall",
        Material::CobbledDeepslateWall,
    );
    register(
        "minecraft:polished_deepslate_wall",
        Material::PolishedDeepslateWall,
    );
    register(
        "minecraft:deepslate_brick_wall",
        Material::DeepslateBrickWall,
    );
    register("minecraft:deepslate_tile_wall", Material::DeepslateTileWall);
    register("minecraft:anvil", Material::Anvil);
    register("minecraft:chipped_anvil", Material::ChippedAnvil);
    register("minecraft:damaged_anvil", Material::DamagedAnvil);
    register(
        "minecraft:chiseled_quartz_block",
        Material::ChiseledQuartzBlock,
    );
    register("minecraft:quartz_block", Material::QuartzBlock);
    register("minecraft:quartz_bricks", Material::QuartzBricks);
    register("minecraft:quartz_pillar", Material::QuartzPillar);
    register("minecraft:quartz_stairs", Material::QuartzStairs);
    register("minecraft:white_terracotta", Material::WhiteTerracotta);
    register("minecraft:orange_terracotta", Material::OrangeTerracotta);
    register("minecraft:magenta_terracotta", Material::MagentaTerracotta);
    register(
        "minecraft:light_blue_terracotta",
        Material::LightBlueTerracotta,
    );
    register("minecraft:yellow_terracotta", Material::YellowTerracotta);
    register("minecraft:lime_terracotta", Material::LimeTerracotta);
    register("minecraft:pink_terracotta", Material::PinkTerracotta);
    register("minecraft:gray_terracotta", Material::GrayTerracotta);
    register(
        "minecraft:light_gray_terracotta",
        Material::LightGrayTerracotta,
    );
    register("minecraft:cyan_terracotta", Material::CyanTerracotta);
    register("minecraft:purple_terracotta", Material::PurpleTerracotta);
    register("minecraft:blue_terracotta", Material::BlueTerracotta);
    register("minecraft:brown_terracotta", Material::BrownTerracotta);
    register("minecraft:green_terracotta", Material::GreenTerracotta);
    register("minecraft:red_terracotta", Material::RedTerracotta);
    register("minecraft:black_terracotta", Material::BlackTerracotta);
    register("minecraft:barrier", Material::Barrier);
    register("minecraft:light", Material::Light);
    register("minecraft:hay_block", Material::HayBlock);
    register("minecraft:white_carpet", Material::WhiteCarpet);
    register("minecraft:orange_carpet", Material::OrangeCarpet);
    register("minecraft:magenta_carpet", Material::MagentaCarpet);
    register("minecraft:light_blue_carpet", Material::LightBlueCarpet);
    register("minecraft:yellow_carpet", Material::YellowCarpet);
    register("minecraft:lime_carpet", Material::LimeCarpet);
    register("minecraft:pink_carpet", Material::PinkCarpet);
    register("minecraft:gray_carpet", Material::GrayCarpet);
    register("minecraft:light_gray_carpet", Material::LightGrayCarpet);
    register("minecraft:cyan_carpet", Material::CyanCarpet);
    register("minecraft:purple_carpet", Material::PurpleCarpet);
    register("minecraft:blue_carpet", Material::BlueCarpet);
    register("minecraft:brown_carpet", Material::BrownCarpet);
    register("minecraft:green_carpet", Material::GreenCarpet);
    register("minecraft:red_carpet", Material::RedCarpet);
    register("minecraft:black_carpet", Material::BlackCarpet);
    register("minecraft:terracotta", Material::Terracotta);
    register("minecraft:packed_ice", Material::PackedIce);
    register("minecraft:dirt_path", Material::DirtPath);
    register("minecraft:sunflower", Material::Sunflower);
    register("minecraft:lilac", Material::Lilac);
    register("minecraft:rose_bush", Material::RoseBush);
    register("minecraft:peony", Material::Peony);
    register("minecraft:tall_grass", Material::TallGrass);
    register("minecraft:large_fern", Material::LargeFern);
    register("minecraft:white_stained_glass", Material::WhiteStainedGlass);
    register(
        "minecraft:orange_stained_glass",
        Material::OrangeStainedGlass,
    );
    register(
        "minecraft:magenta_stained_glass",
        Material::MagentaStainedGlass,
    );
    register(
        "minecraft:light_blue_stained_glass",
        Material::LightBlueStainedGlass,
    );
    register(
        "minecraft:yellow_stained_glass",
        Material::YellowStainedGlass,
    );
    register("minecraft:lime_stained_glass", Material::LimeStainedGlass);
    register("minecraft:pink_stained_glass", Material::PinkStainedGlass);
    register("minecraft:gray_stained_glass", Material::GrayStainedGlass);
    register(
        "minecraft:light_gray_stained_glass",
        Material::LightGrayStainedGlass,
    );
    register("minecraft:cyan_stained_glass", Material::CyanStainedGlass);
    register(
        "minecraft:purple_stained_glass",
        Material::PurpleStainedGlass,
    );
    register("minecraft:blue_stained_glass", Material::BlueStainedGlass);
    register("minecraft:brown_stained_glass", Material::BrownStainedGlass);
    register("minecraft:green_stained_glass", Material::GreenStainedGlass);
    register("minecraft:red_stained_glass", Material::RedStainedGlass);
    register("minecraft:black_stained_glass", Material::BlackStainedGlass);
    register(
        "minecraft:white_stained_glass_pane",
        Material::WhiteStainedGlassPane,
    );
    register(
        "minecraft:orange_stained_glass_pane",
        Material::OrangeStainedGlassPane,
    );
    register(
        "minecraft:magenta_stained_glass_pane",
        Material::MagentaStainedGlassPane,
    );
    register(
        "minecraft:light_blue_stained_glass_pane",
        Material::LightBlueStainedGlassPane,
    );
    register(
        "minecraft:yellow_stained_glass_pane",
        Material::YellowStainedGlassPane,
    );
    register(
        "minecraft:lime_stained_glass_pane",
        Material::LimeStainedGlassPane,
    );
    register(
        "minecraft:pink_stained_glass_pane",
        Material::PinkStainedGlassPane,
    );
    register(
        "minecraft:gray_stained_glass_pane",
        Material::GrayStainedGlassPane,
    );
    register(
        "minecraft:light_gray_stained_glass_pane",
        Material::LightGrayStainedGlassPane,
    );
    register(
        "minecraft:cyan_stained_glass_pane",
        Material::CyanStainedGlassPane,
    );
    register(
        "minecraft:purple_stained_glass_pane",
        Material::PurpleStainedGlassPane,
    );
    register(
        "minecraft:blue_stained_glass_pane",
        Material::BlueStainedGlassPane,
    );
    register(
        "minecraft:brown_stained_glass_pane",
        Material::BrownStainedGlassPane,
    );
    register(
        "minecraft:green_stained_glass_pane",
        Material::GreenStainedGlassPane,
    );
    register(
        "minecraft:red_stained_glass_pane",
        Material::RedStainedGlassPane,
    );
    register(
        "minecraft:black_stained_glass_pane",
        Material::BlackStainedGlassPane,
    );
    register("minecraft:prismarine", Material::Prismarine);
    register("minecraft:prismarine_bricks", Material::PrismarineBricks);
    register("minecraft:dark_prismarine", Material::DarkPrismarine);
    register("minecraft:prismarine_stairs", Material::PrismarineStairs);
    register(
        "minecraft:prismarine_brick_stairs",
        Material::PrismarineBrickStairs,
    );
    register(
        "minecraft:dark_prismarine_stairs",
        Material::DarkPrismarineStairs,
    );
    register("minecraft:sea_lantern", Material::SeaLantern);
    register("minecraft:red_sandstone", Material::RedSandstone);
    register(
        "minecraft:chiseled_red_sandstone",
        Material::ChiseledRedSandstone,
    );
    register("minecraft:cut_red_sandstone", Material::CutRedSandstone);
    register(
        "minecraft:red_sandstone_stairs",
        Material::RedSandstoneStairs,
    );
    register(
        "minecraft:repeating_command_block",
        Material::RepeatingCommandBlock,
    );
    register("minecraft:chain_command_block", Material::ChainCommandBlock);
    register("minecraft:magma_block", Material::MagmaBlock);
    register("minecraft:nether_wart_block", Material::NetherWartBlock);
    register("minecraft:warped_wart_block", Material::WarpedWartBlock);
    register("minecraft:red_nether_bricks", Material::RedNetherBricks);
    register("minecraft:bone_block", Material::BoneBlock);
    register("minecraft:structure_void", Material::StructureVoid);
    register("minecraft:shulker_box", Material::ShulkerBox);
    register("minecraft:white_shulker_box", Material::WhiteShulkerBox);
    register("minecraft:orange_shulker_box", Material::OrangeShulkerBox);
    register("minecraft:magenta_shulker_box", Material::MagentaShulkerBox);
    register(
        "minecraft:light_blue_shulker_box",
        Material::LightBlueShulkerBox,
    );
    register("minecraft:yellow_shulker_box", Material::YellowShulkerBox);
    register("minecraft:lime_shulker_box", Material::LimeShulkerBox);
    register("minecraft:pink_shulker_box", Material::PinkShulkerBox);
    register("minecraft:gray_shulker_box", Material::GrayShulkerBox);
    register(
        "minecraft:light_gray_shulker_box",
        Material::LightGrayShulkerBox,
    );
    register("minecraft:cyan_shulker_box", Material::CyanShulkerBox);
    register("minecraft:purple_shulker_box", Material::PurpleShulkerBox);
    register("minecraft:blue_shulker_box", Material::BlueShulkerBox);
    register("minecraft:brown_shulker_box", Material::BrownShulkerBox);
    register("minecraft:green_shulker_box", Material::GreenShulkerBox);
    register("minecraft:red_shulker_box", Material::RedShulkerBox);
    register("minecraft:black_shulker_box", Material::BlackShulkerBox);
    register(
        "minecraft:white_glazed_terracotta",
        Material::WhiteGlazedTerracotta,
    );
    register(
        "minecraft:orange_glazed_terracotta",
        Material::OrangeGlazedTerracotta,
    );
    register(
        "minecraft:magenta_glazed_terracotta",
        Material::MagentaGlazedTerracotta,
    );
    register(
        "minecraft:light_blue_glazed_terracotta",
        Material::LightBlueGlazedTerracotta,
    );
    register(
        "minecraft:yellow_glazed_terracotta",
        Material::YellowGlazedTerracotta,
    );
    register(
        "minecraft:lime_glazed_terracotta",
        Material::LimeGlazedTerracotta,
    );
    register(
        "minecraft:pink_glazed_terracotta",
        Material::PinkGlazedTerracotta,
    );
    register(
        "minecraft:gray_glazed_terracotta",
        Material::GrayGlazedTerracotta,
    );
    register(
        "minecraft:light_gray_glazed_terracotta",
        Material::LightGrayGlazedTerracotta,
    );
    register(
        "minecraft:cyan_glazed_terracotta",
        Material::CyanGlazedTerracotta,
    );
    register(
        "minecraft:purple_glazed_terracotta",
        Material::PurpleGlazedTerracotta,
    );
    register(
        "minecraft:blue_glazed_terracotta",
        Material::BlueGlazedTerracotta,
    );
    register(
        "minecraft:brown_glazed_terracotta",
        Material::BrownGlazedTerracotta,
    );
    register(
        "minecraft:green_glazed_terracotta",
        Material::GreenGlazedTerracotta,
    );
    register(
        "minecraft:red_glazed_terracotta",
        Material::RedGlazedTerracotta,
    );
    register(
        "minecraft:black_glazed_terracotta",
        Material::BlackGlazedTerracotta,
    );
    register("minecraft:white_concrete", Material::WhiteConcrete);
    register("minecraft:orange_concrete", Material::OrangeConcrete);
    register("minecraft:magenta_concrete", Material::MagentaConcrete);
    register("minecraft:light_blue_concrete", Material::LightBlueConcrete);
    register("minecraft:yellow_concrete", Material::YellowConcrete);
    register("minecraft:lime_concrete", Material::LimeConcrete);
    register("minecraft:pink_concrete", Material::PinkConcrete);
    register("minecraft:gray_concrete", Material::GrayConcrete);
    register("minecraft:light_gray_concrete", Material::LightGrayConcrete);
    register("minecraft:cyan_concrete", Material::CyanConcrete);
    register("minecraft:purple_concrete", Material::PurpleConcrete);
    register("minecraft:blue_concrete", Material::BlueConcrete);
    register("minecraft:brown_concrete", Material::BrownConcrete);
    register("minecraft:green_concrete", Material::GreenConcrete);
    register("minecraft:red_concrete", Material::RedConcrete);
    register("minecraft:black_concrete", Material::BlackConcrete);
    register(
        "minecraft:white_concrete_powder",
        Material::WhiteConcretePowder,
    );
    register(
        "minecraft:orange_concrete_powder",
        Material::OrangeConcretePowder,
    );
    register(
        "minecraft:magenta_concrete_powder",
        Material::MagentaConcretePowder,
    );
    register(
        "minecraft:light_blue_concrete_powder",
        Material::LightBlueConcretePowder,
    );
    register(
        "minecraft:yellow_concrete_powder",
        Material::YellowConcretePowder,
    );
    register(
        "minecraft:lime_concrete_powder",
        Material::LimeConcretePowder,
    );
    register(
        "minecraft:pink_concrete_powder",
        Material::PinkConcretePowder,
    );
    register(
        "minecraft:gray_concrete_powder",
        Material::GrayConcretePowder,
    );
    register(
        "minecraft:light_gray_concrete_powder",
        Material::LightGrayConcretePowder,
    );
    register(
        "minecraft:cyan_concrete_powder",
        Material::CyanConcretePowder,
    );
    register(
        "minecraft:purple_concrete_powder",
        Material::PurpleConcretePowder,
    );
    register(
        "minecraft:blue_concrete_powder",
        Material::BlueConcretePowder,
    );
    register(
        "minecraft:brown_concrete_powder",
        Material::BrownConcretePowder,
    );
    register(
        "minecraft:green_concrete_powder",
        Material::GreenConcretePowder,
    );
    register("minecraft:red_concrete_powder", Material::RedConcretePowder);
    register(
        "minecraft:black_concrete_powder",
        Material::BlackConcretePowder,
    );
    register("minecraft:turtle_egg", Material::TurtleEgg);
    register("minecraft:sniffer_egg", Material::SnifferEgg);
    register("minecraft:dried_ghast", Material::DriedGhast);
    register(
        "minecraft:dead_tube_coral_block",
        Material::DeadTubeCoralBlock,
    );
    register(
        "minecraft:dead_brain_coral_block",
        Material::DeadBrainCoralBlock,
    );
    register(
        "minecraft:dead_bubble_coral_block",
        Material::DeadBubbleCoralBlock,
    );
    register(
        "minecraft:dead_fire_coral_block",
        Material::DeadFireCoralBlock,
    );
    register(
        "minecraft:dead_horn_coral_block",
        Material::DeadHornCoralBlock,
    );
    register("minecraft:tube_coral_block", Material::TubeCoralBlock);
    register("minecraft:brain_coral_block", Material::BrainCoralBlock);
    register("minecraft:bubble_coral_block", Material::BubbleCoralBlock);
    register("minecraft:fire_coral_block", Material::FireCoralBlock);
    register("minecraft:horn_coral_block", Material::HornCoralBlock);
    register("minecraft:tube_coral", Material::TubeCoral);
    register("minecraft:brain_coral", Material::BrainCoral);
    register("minecraft:bubble_coral", Material::BubbleCoral);
    register("minecraft:fire_coral", Material::FireCoral);
    register("minecraft:horn_coral", Material::HornCoral);
    register("minecraft:dead_brain_coral", Material::DeadBrainCoral);
    register("minecraft:dead_bubble_coral", Material::DeadBubbleCoral);
    register("minecraft:dead_fire_coral", Material::DeadFireCoral);
    register("minecraft:dead_horn_coral", Material::DeadHornCoral);
    register("minecraft:dead_tube_coral", Material::DeadTubeCoral);
    register("minecraft:tube_coral_fan", Material::TubeCoralFan);
    register("minecraft:brain_coral_fan", Material::BrainCoralFan);
    register("minecraft:bubble_coral_fan", Material::BubbleCoralFan);
    register("minecraft:fire_coral_fan", Material::FireCoralFan);
    register("minecraft:horn_coral_fan", Material::HornCoralFan);
    register("minecraft:dead_tube_coral_fan", Material::DeadTubeCoralFan);
    register(
        "minecraft:dead_brain_coral_fan",
        Material::DeadBrainCoralFan,
    );
    register(
        "minecraft:dead_bubble_coral_fan",
        Material::DeadBubbleCoralFan,
    );
    register("minecraft:dead_fire_coral_fan", Material::DeadFireCoralFan);
    register("minecraft:dead_horn_coral_fan", Material::DeadHornCoralFan);
    register("minecraft:blue_ice", Material::BlueIce);
    register("minecraft:conduit", Material::Conduit);
    register(
        "minecraft:polished_granite_stairs",
        Material::PolishedGraniteStairs,
    );
    register(
        "minecraft:smooth_red_sandstone_stairs",
        Material::SmoothRedSandstoneStairs,
    );
    register(
        "minecraft:mossy_stone_brick_stairs",
        Material::MossyStoneBrickStairs,
    );
    register(
        "minecraft:polished_diorite_stairs",
        Material::PolishedDioriteStairs,
    );
    register(
        "minecraft:mossy_cobblestone_stairs",
        Material::MossyCobblestoneStairs,
    );
    register(
        "minecraft:end_stone_brick_stairs",
        Material::EndStoneBrickStairs,
    );
    register("minecraft:stone_stairs", Material::StoneStairs);
    register(
        "minecraft:smooth_sandstone_stairs",
        Material::SmoothSandstoneStairs,
    );
    register(
        "minecraft:smooth_quartz_stairs",
        Material::SmoothQuartzStairs,
    );
    register("minecraft:granite_stairs", Material::GraniteStairs);
    register("minecraft:andesite_stairs", Material::AndesiteStairs);
    register(
        "minecraft:red_nether_brick_stairs",
        Material::RedNetherBrickStairs,
    );
    register(
        "minecraft:polished_andesite_stairs",
        Material::PolishedAndesiteStairs,
    );
    register("minecraft:diorite_stairs", Material::DioriteStairs);
    register(
        "minecraft:cobbled_deepslate_stairs",
        Material::CobbledDeepslateStairs,
    );
    register(
        "minecraft:polished_deepslate_stairs",
        Material::PolishedDeepslateStairs,
    );
    register(
        "minecraft:deepslate_brick_stairs",
        Material::DeepslateBrickStairs,
    );
    register(
        "minecraft:deepslate_tile_stairs",
        Material::DeepslateTileStairs,
    );
    register(
        "minecraft:polished_granite_slab",
        Material::PolishedGraniteSlab,
    );
    register(
        "minecraft:smooth_red_sandstone_slab",
        Material::SmoothRedSandstoneSlab,
    );
    register(
        "minecraft:mossy_stone_brick_slab",
        Material::MossyStoneBrickSlab,
    );
    register(
        "minecraft:polished_diorite_slab",
        Material::PolishedDioriteSlab,
    );
    register(
        "minecraft:mossy_cobblestone_slab",
        Material::MossyCobblestoneSlab,
    );
    register(
        "minecraft:end_stone_brick_slab",
        Material::EndStoneBrickSlab,
    );
    register(
        "minecraft:smooth_sandstone_slab",
        Material::SmoothSandstoneSlab,
    );
    register("minecraft:smooth_quartz_slab", Material::SmoothQuartzSlab);
    register("minecraft:granite_slab", Material::GraniteSlab);
    register("minecraft:andesite_slab", Material::AndesiteSlab);
    register(
        "minecraft:red_nether_brick_slab",
        Material::RedNetherBrickSlab,
    );
    register(
        "minecraft:polished_andesite_slab",
        Material::PolishedAndesiteSlab,
    );
    register("minecraft:diorite_slab", Material::DioriteSlab);
    register(
        "minecraft:cobbled_deepslate_slab",
        Material::CobbledDeepslateSlab,
    );
    register(
        "minecraft:polished_deepslate_slab",
        Material::PolishedDeepslateSlab,
    );
    register(
        "minecraft:deepslate_brick_slab",
        Material::DeepslateBrickSlab,
    );
    register("minecraft:deepslate_tile_slab", Material::DeepslateTileSlab);
    register("minecraft:scaffolding", Material::Scaffolding);
    register("minecraft:redstone", Material::Redstone);
    register("minecraft:redstone_torch", Material::RedstoneTorch);
    register("minecraft:redstone_block", Material::RedstoneBlock);
    register("minecraft:repeater", Material::Repeater);
    register("minecraft:comparator", Material::Comparator);
    register("minecraft:piston", Material::Piston);
    register("minecraft:sticky_piston", Material::StickyPiston);
    register("minecraft:slime_block", Material::SlimeBlock);
    register("minecraft:honey_block", Material::HoneyBlock);
    register("minecraft:observer", Material::Observer);
    register("minecraft:hopper", Material::Hopper);
    register("minecraft:dispenser", Material::Dispenser);
    register("minecraft:dropper", Material::Dropper);
    register("minecraft:lectern", Material::Lectern);
    register("minecraft:target", Material::Target);
    register("minecraft:lever", Material::Lever);
    register("minecraft:lightning_rod", Material::LightningRod);
    register(
        "minecraft:exposed_lightning_rod",
        Material::ExposedLightningRod,
    );
    register(
        "minecraft:weathered_lightning_rod",
        Material::WeatheredLightningRod,
    );
    register(
        "minecraft:oxidized_lightning_rod",
        Material::OxidizedLightningRod,
    );
    register("minecraft:waxed_lightning_rod", Material::WaxedLightningRod);
    register(
        "minecraft:waxed_exposed_lightning_rod",
        Material::WaxedExposedLightningRod,
    );
    register(
        "minecraft:waxed_weathered_lightning_rod",
        Material::WaxedWeatheredLightningRod,
    );
    register(
        "minecraft:waxed_oxidized_lightning_rod",
        Material::WaxedOxidizedLightningRod,
    );
    register("minecraft:daylight_detector", Material::DaylightDetector);
    register("minecraft:sculk_sensor", Material::SculkSensor);
    register(
        "minecraft:calibrated_sculk_sensor",
        Material::CalibratedSculkSensor,
    );
    register("minecraft:tripwire_hook", Material::TripwireHook);
    register("minecraft:trapped_chest", Material::TrappedChest);
    register("minecraft:tnt", Material::Tnt);
    register("minecraft:redstone_lamp", Material::RedstoneLamp);
    register("minecraft:note_block", Material::NoteBlock);
    register("minecraft:stone_button", Material::StoneButton);
    register(
        "minecraft:polished_blackstone_button",
        Material::PolishedBlackstoneButton,
    );
    register("minecraft:oak_button", Material::OakButton);
    register("minecraft:spruce_button", Material::SpruceButton);
    register("minecraft:birch_button", Material::BirchButton);
    register("minecraft:jungle_button", Material::JungleButton);
    register("minecraft:acacia_button", Material::AcaciaButton);
    register("minecraft:cherry_button", Material::CherryButton);
    register("minecraft:dark_oak_button", Material::DarkOakButton);
    register("minecraft:pale_oak_button", Material::PaleOakButton);
    register("minecraft:mangrove_button", Material::MangroveButton);
    register("minecraft:bamboo_button", Material::BambooButton);
    register("minecraft:crimson_button", Material::CrimsonButton);
    register("minecraft:warped_button", Material::WarpedButton);
    register(
        "minecraft:stone_pressure_plate",
        Material::StonePressurePlate,
    );
    register(
        "minecraft:polished_blackstone_pressure_plate",
        Material::PolishedBlackstonePressurePlate,
    );
    register(
        "minecraft:light_weighted_pressure_plate",
        Material::LightWeightedPressurePlate,
    );
    register(
        "minecraft:heavy_weighted_pressure_plate",
        Material::HeavyWeightedPressurePlate,
    );
    register("minecraft:oak_pressure_plate", Material::OakPressurePlate);
    register(
        "minecraft:spruce_pressure_plate",
        Material::SprucePressurePlate,
    );
    register(
        "minecraft:birch_pressure_plate",
        Material::BirchPressurePlate,
    );
    register(
        "minecraft:jungle_pressure_plate",
        Material::JunglePressurePlate,
    );
    register(
        "minecraft:acacia_pressure_plate",
        Material::AcaciaPressurePlate,
    );
    register(
        "minecraft:cherry_pressure_plate",
        Material::CherryPressurePlate,
    );
    register(
        "minecraft:dark_oak_pressure_plate",
        Material::DarkOakPressurePlate,
    );
    register(
        "minecraft:pale_oak_pressure_plate",
        Material::PaleOakPressurePlate,
    );
    register(
        "minecraft:mangrove_pressure_plate",
        Material::MangrovePressurePlate,
    );
    register(
        "minecraft:bamboo_pressure_plate",
        Material::BambooPressurePlate,
    );
    register(
        "minecraft:crimson_pressure_plate",
        Material::CrimsonPressurePlate,
    );
    register(
        "minecraft:warped_pressure_plate",
        Material::WarpedPressurePlate,
    );
    register("minecraft:iron_door", Material::IronDoor);
    register("minecraft:oak_door", Material::OakDoor);
    register("minecraft:spruce_door", Material::SpruceDoor);
    register("minecraft:birch_door", Material::BirchDoor);
    register("minecraft:jungle_door", Material::JungleDoor);
    register("minecraft:acacia_door", Material::AcaciaDoor);
    register("minecraft:cherry_door", Material::CherryDoor);
    register("minecraft:dark_oak_door", Material::DarkOakDoor);
    register("minecraft:pale_oak_door", Material::PaleOakDoor);
    register("minecraft:mangrove_door", Material::MangroveDoor);
    register("minecraft:bamboo_door", Material::BambooDoor);
    register("minecraft:crimson_door", Material::CrimsonDoor);
    register("minecraft:warped_door", Material::WarpedDoor);
    register("minecraft:copper_door", Material::CopperDoor);
    register("minecraft:exposed_copper_door", Material::ExposedCopperDoor);
    register(
        "minecraft:weathered_copper_door",
        Material::WeatheredCopperDoor,
    );
    register(
        "minecraft:oxidized_copper_door",
        Material::OxidizedCopperDoor,
    );
    register("minecraft:waxed_copper_door", Material::WaxedCopperDoor);
    register(
        "minecraft:waxed_exposed_copper_door",
        Material::WaxedExposedCopperDoor,
    );
    register(
        "minecraft:waxed_weathered_copper_door",
        Material::WaxedWeatheredCopperDoor,
    );
    register(
        "minecraft:waxed_oxidized_copper_door",
        Material::WaxedOxidizedCopperDoor,
    );
    register("minecraft:iron_trapdoor", Material::IronTrapdoor);
    register("minecraft:oak_trapdoor", Material::OakTrapdoor);
    register("minecraft:spruce_trapdoor", Material::SpruceTrapdoor);
    register("minecraft:birch_trapdoor", Material::BirchTrapdoor);
    register("minecraft:jungle_trapdoor", Material::JungleTrapdoor);
    register("minecraft:acacia_trapdoor", Material::AcaciaTrapdoor);
    register("minecraft:cherry_trapdoor", Material::CherryTrapdoor);
    register("minecraft:dark_oak_trapdoor", Material::DarkOakTrapdoor);
    register("minecraft:pale_oak_trapdoor", Material::PaleOakTrapdoor);
    register("minecraft:mangrove_trapdoor", Material::MangroveTrapdoor);
    register("minecraft:bamboo_trapdoor", Material::BambooTrapdoor);
    register("minecraft:crimson_trapdoor", Material::CrimsonTrapdoor);
    register("minecraft:warped_trapdoor", Material::WarpedTrapdoor);
    register("minecraft:copper_trapdoor", Material::CopperTrapdoor);
    register(
        "minecraft:exposed_copper_trapdoor",
        Material::ExposedCopperTrapdoor,
    );
    register(
        "minecraft:weathered_copper_trapdoor",
        Material::WeatheredCopperTrapdoor,
    );
    register(
        "minecraft:oxidized_copper_trapdoor",
        Material::OxidizedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_copper_trapdoor",
        Material::WaxedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_exposed_copper_trapdoor",
        Material::WaxedExposedCopperTrapdoor,
    );
    register(
        "minecraft:waxed_weathered_copper_trapdoor",
        Material::WaxedWeatheredCopperTrapdoor,
    );
    register(
        "minecraft:waxed_oxidized_copper_trapdoor",
        Material::WaxedOxidizedCopperTrapdoor,
    );
    register("minecraft:oak_fence_gate", Material::OakFenceGate);
    register("minecraft:spruce_fence_gate", Material::SpruceFenceGate);
    register("minecraft:birch_fence_gate", Material::BirchFenceGate);
    register("minecraft:jungle_fence_gate", Material::JungleFenceGate);
    register("minecraft:acacia_fence_gate", Material::AcaciaFenceGate);
    register("minecraft:cherry_fence_gate", Material::CherryFenceGate);
    register("minecraft:dark_oak_fence_gate", Material::DarkOakFenceGate);
    register("minecraft:pale_oak_fence_gate", Material::PaleOakFenceGate);
    register("minecraft:mangrove_fence_gate", Material::MangroveFenceGate);
    register("minecraft:bamboo_fence_gate", Material::BambooFenceGate);
    register("minecraft:crimson_fence_gate", Material::CrimsonFenceGate);
    register("minecraft:warped_fence_gate", Material::WarpedFenceGate);
    register("minecraft:powered_rail", Material::PoweredRail);
    register("minecraft:detector_rail", Material::DetectorRail);
    register("minecraft:rail", Material::Rail);
    register("minecraft:activator_rail", Material::ActivatorRail);
    register("minecraft:saddle", Material::Saddle);
    register("minecraft:white_harness", Material::WhiteHarness);
    register("minecraft:orange_harness", Material::OrangeHarness);
    register("minecraft:magenta_harness", Material::MagentaHarness);
    register("minecraft:light_blue_harness", Material::LightBlueHarness);
    register("minecraft:yellow_harness", Material::YellowHarness);
    register("minecraft:lime_harness", Material::LimeHarness);
    register("minecraft:pink_harness", Material::PinkHarness);
    register("minecraft:gray_harness", Material::GrayHarness);
    register("minecraft:light_gray_harness", Material::LightGrayHarness);
    register("minecraft:cyan_harness", Material::CyanHarness);
    register("minecraft:purple_harness", Material::PurpleHarness);
    register("minecraft:blue_harness", Material::BlueHarness);
    register("minecraft:brown_harness", Material::BrownHarness);
    register("minecraft:green_harness", Material::GreenHarness);
    register("minecraft:red_harness", Material::RedHarness);
    register("minecraft:black_harness", Material::BlackHarness);
    register("minecraft:minecart", Material::Minecart);
    register("minecraft:chest_minecart", Material::ChestMinecart);
    register("minecraft:furnace_minecart", Material::FurnaceMinecart);
    register("minecraft:tnt_minecart", Material::TntMinecart);
    register("minecraft:hopper_minecart", Material::HopperMinecart);
    register("minecraft:carrot_on_a_stick", Material::CarrotOnAStick);
    register(
        "minecraft:warped_fungus_on_a_stick",
        Material::WarpedFungusOnAStick,
    );
    register("minecraft:phantom_membrane", Material::PhantomMembrane);
    register("minecraft:elytra", Material::Elytra);
    register("minecraft:oak_boat", Material::OakBoat);
    register("minecraft:oak_chest_boat", Material::OakChestBoat);
    register("minecraft:spruce_boat", Material::SpruceBoat);
    register("minecraft:spruce_chest_boat", Material::SpruceChestBoat);
    register("minecraft:birch_boat", Material::BirchBoat);
    register("minecraft:birch_chest_boat", Material::BirchChestBoat);
    register("minecraft:jungle_boat", Material::JungleBoat);
    register("minecraft:jungle_chest_boat", Material::JungleChestBoat);
    register("minecraft:acacia_boat", Material::AcaciaBoat);
    register("minecraft:acacia_chest_boat", Material::AcaciaChestBoat);
    register("minecraft:cherry_boat", Material::CherryBoat);
    register("minecraft:cherry_chest_boat", Material::CherryChestBoat);
    register("minecraft:dark_oak_boat", Material::DarkOakBoat);
    register("minecraft:dark_oak_chest_boat", Material::DarkOakChestBoat);
    register("minecraft:pale_oak_boat", Material::PaleOakBoat);
    register("minecraft:pale_oak_chest_boat", Material::PaleOakChestBoat);
    register("minecraft:mangrove_boat", Material::MangroveBoat);
    register("minecraft:mangrove_chest_boat", Material::MangroveChestBoat);
    register("minecraft:bamboo_raft", Material::BambooRaft);
    register("minecraft:bamboo_chest_raft", Material::BambooChestRaft);
    register("minecraft:structure_block", Material::StructureBlock);
    register("minecraft:jigsaw", Material::Jigsaw);
    register("minecraft:test_block", Material::TestBlock);
    register("minecraft:test_instance_block", Material::TestInstanceBlock);
    register("minecraft:turtle_helmet", Material::TurtleHelmet);
    register("minecraft:turtle_scute", Material::TurtleScute);
    register("minecraft:armadillo_scute", Material::ArmadilloScute);
    register("minecraft:wolf_armor", Material::WolfArmor);
    register("minecraft:flint_and_steel", Material::FlintAndSteel);
    register("minecraft:bowl", Material::Bowl);
    register("minecraft:apple", Material::Apple);
    register("minecraft:bow", Material::Bow);
    register("minecraft:arrow", Material::Arrow);
    register("minecraft:coal", Material::Coal);
    register("minecraft:charcoal", Material::Charcoal);
    register("minecraft:diamond", Material::Diamond);
    register("minecraft:emerald", Material::Emerald);
    register("minecraft:lapis_lazuli", Material::LapisLazuli);
    register("minecraft:quartz", Material::Quartz);
    register("minecraft:amethyst_shard", Material::AmethystShard);
    register("minecraft:raw_iron", Material::RawIron);
    register("minecraft:iron_ingot", Material::IronIngot);
    register("minecraft:raw_copper", Material::RawCopper);
    register("minecraft:copper_ingot", Material::CopperIngot);
    register("minecraft:raw_gold", Material::RawGold);
    register("minecraft:gold_ingot", Material::GoldIngot);
    register("minecraft:netherite_ingot", Material::NetheriteIngot);
    register("minecraft:netherite_scrap", Material::NetheriteScrap);
    register("minecraft:wooden_sword", Material::WoodenSword);
    register("minecraft:wooden_shovel", Material::WoodenShovel);
    register("minecraft:wooden_pickaxe", Material::WoodenPickaxe);
    register("minecraft:wooden_axe", Material::WoodenAxe);
    register("minecraft:wooden_hoe", Material::WoodenHoe);
    register("minecraft:copper_sword", Material::CopperSword);
    register("minecraft:copper_shovel", Material::CopperShovel);
    register("minecraft:copper_pickaxe", Material::CopperPickaxe);
    register("minecraft:copper_axe", Material::CopperAxe);
    register("minecraft:copper_hoe", Material::CopperHoe);
    register("minecraft:stone_sword", Material::StoneSword);
    register("minecraft:stone_shovel", Material::StoneShovel);
    register("minecraft:stone_pickaxe", Material::StonePickaxe);
    register("minecraft:stone_axe", Material::StoneAxe);
    register("minecraft:stone_hoe", Material::StoneHoe);
    register("minecraft:golden_sword", Material::GoldenSword);
    register("minecraft:golden_shovel", Material::GoldenShovel);
    register("minecraft:golden_pickaxe", Material::GoldenPickaxe);
    register("minecraft:golden_axe", Material::GoldenAxe);
    register("minecraft:golden_hoe", Material::GoldenHoe);
    register("minecraft:iron_sword", Material::IronSword);
    register("minecraft:iron_shovel", Material::IronShovel);
    register("minecraft:iron_pickaxe", Material::IronPickaxe);
    register("minecraft:iron_axe", Material::IronAxe);
    register("minecraft:iron_hoe", Material::IronHoe);
    register("minecraft:diamond_sword", Material::DiamondSword);
    register("minecraft:diamond_shovel", Material::DiamondShovel);
    register("minecraft:diamond_pickaxe", Material::DiamondPickaxe);
    register("minecraft:diamond_axe", Material::DiamondAxe);
    register("minecraft:diamond_hoe", Material::DiamondHoe);
    register("minecraft:netherite_sword", Material::NetheriteSword);
    register("minecraft:netherite_shovel", Material::NetheriteShovel);
    register("minecraft:netherite_pickaxe", Material::NetheritePickaxe);
    register("minecraft:netherite_axe", Material::NetheriteAxe);
    register("minecraft:netherite_hoe", Material::NetheriteHoe);
    register("minecraft:stick", Material::Stick);
    register("minecraft:mushroom_stew", Material::MushroomStew);
    register("minecraft:string", Material::String);
    register("minecraft:feather", Material::Feather);
    register("minecraft:gunpowder", Material::Gunpowder);
    register("minecraft:wheat_seeds", Material::WheatSeeds);
    register("minecraft:wheat", Material::Wheat);
    register("minecraft:bread", Material::Bread);
    register("minecraft:leather_helmet", Material::LeatherHelmet);
    register("minecraft:leather_chestplate", Material::LeatherChestplate);
    register("minecraft:leather_leggings", Material::LeatherLeggings);
    register("minecraft:leather_boots", Material::LeatherBoots);
    register("minecraft:copper_helmet", Material::CopperHelmet);
    register("minecraft:copper_chestplate", Material::CopperChestplate);
    register("minecraft:copper_leggings", Material::CopperLeggings);
    register("minecraft:copper_boots", Material::CopperBoots);
    register("minecraft:chainmail_helmet", Material::ChainmailHelmet);
    register(
        "minecraft:chainmail_chestplate",
        Material::ChainmailChestplate,
    );
    register("minecraft:chainmail_leggings", Material::ChainmailLeggings);
    register("minecraft:chainmail_boots", Material::ChainmailBoots);
    register("minecraft:iron_helmet", Material::IronHelmet);
    register("minecraft:iron_chestplate", Material::IronChestplate);
    register("minecraft:iron_leggings", Material::IronLeggings);
    register("minecraft:iron_boots", Material::IronBoots);
    register("minecraft:diamond_helmet", Material::DiamondHelmet);
    register("minecraft:diamond_chestplate", Material::DiamondChestplate);
    register("minecraft:diamond_leggings", Material::DiamondLeggings);
    register("minecraft:diamond_boots", Material::DiamondBoots);
    register("minecraft:golden_helmet", Material::GoldenHelmet);
    register("minecraft:golden_chestplate", Material::GoldenChestplate);
    register("minecraft:golden_leggings", Material::GoldenLeggings);
    register("minecraft:golden_boots", Material::GoldenBoots);
    register("minecraft:netherite_helmet", Material::NetheriteHelmet);
    register(
        "minecraft:netherite_chestplate",
        Material::NetheriteChestplate,
    );
    register("minecraft:netherite_leggings", Material::NetheriteLeggings);
    register("minecraft:netherite_boots", Material::NetheriteBoots);
    register("minecraft:flint", Material::Flint);
    register("minecraft:porkchop", Material::Porkchop);
    register("minecraft:cooked_porkchop", Material::CookedPorkchop);
    register("minecraft:painting", Material::Painting);
    register("minecraft:golden_apple", Material::GoldenApple);
    register(
        "minecraft:enchanted_golden_apple",
        Material::EnchantedGoldenApple,
    );
    register("minecraft:oak_sign", Material::OakSign);
    register("minecraft:spruce_sign", Material::SpruceSign);
    register("minecraft:birch_sign", Material::BirchSign);
    register("minecraft:jungle_sign", Material::JungleSign);
    register("minecraft:acacia_sign", Material::AcaciaSign);
    register("minecraft:cherry_sign", Material::CherrySign);
    register("minecraft:dark_oak_sign", Material::DarkOakSign);
    register("minecraft:pale_oak_sign", Material::PaleOakSign);
    register("minecraft:mangrove_sign", Material::MangroveSign);
    register("minecraft:bamboo_sign", Material::BambooSign);
    register("minecraft:crimson_sign", Material::CrimsonSign);
    register("minecraft:warped_sign", Material::WarpedSign);
    register("minecraft:oak_hanging_sign", Material::OakHangingSign);
    register("minecraft:spruce_hanging_sign", Material::SpruceHangingSign);
    register("minecraft:birch_hanging_sign", Material::BirchHangingSign);
    register("minecraft:jungle_hanging_sign", Material::JungleHangingSign);
    register("minecraft:acacia_hanging_sign", Material::AcaciaHangingSign);
    register("minecraft:cherry_hanging_sign", Material::CherryHangingSign);
    register(
        "minecraft:dark_oak_hanging_sign",
        Material::DarkOakHangingSign,
    );
    register(
        "minecraft:pale_oak_hanging_sign",
        Material::PaleOakHangingSign,
    );
    register(
        "minecraft:mangrove_hanging_sign",
        Material::MangroveHangingSign,
    );
    register("minecraft:bamboo_hanging_sign", Material::BambooHangingSign);
    register(
        "minecraft:crimson_hanging_sign",
        Material::CrimsonHangingSign,
    );
    register("minecraft:warped_hanging_sign", Material::WarpedHangingSign);
    register("minecraft:bucket", Material::Bucket);
    register("minecraft:water_bucket", Material::WaterBucket);
    register("minecraft:lava_bucket", Material::LavaBucket);
    register("minecraft:powder_snow_bucket", Material::PowderSnowBucket);
    register("minecraft:snowball", Material::Snowball);
    register("minecraft:leather", Material::Leather);
    register("minecraft:milk_bucket", Material::MilkBucket);
    register("minecraft:pufferfish_bucket", Material::PufferfishBucket);
    register("minecraft:salmon_bucket", Material::SalmonBucket);
    register("minecraft:cod_bucket", Material::CodBucket);
    register(
        "minecraft:tropical_fish_bucket",
        Material::TropicalFishBucket,
    );
    register("minecraft:axolotl_bucket", Material::AxolotlBucket);
    register("minecraft:sulfur_cube_bucket", Material::SulfurCubeBucket);
    register("minecraft:tadpole_bucket", Material::TadpoleBucket);
    register("minecraft:brick", Material::Brick);
    register("minecraft:clay_ball", Material::ClayBall);
    register("minecraft:dried_kelp_block", Material::DriedKelpBlock);
    register("minecraft:paper", Material::Paper);
    register("minecraft:book", Material::Book);
    register("minecraft:slime_ball", Material::SlimeBall);
    register("minecraft:egg", Material::Egg);
    register("minecraft:blue_egg", Material::BlueEgg);
    register("minecraft:brown_egg", Material::BrownEgg);
    register("minecraft:compass", Material::Compass);
    register("minecraft:recovery_compass", Material::RecoveryCompass);
    register("minecraft:bundle", Material::Bundle);
    register("minecraft:white_bundle", Material::WhiteBundle);
    register("minecraft:orange_bundle", Material::OrangeBundle);
    register("minecraft:magenta_bundle", Material::MagentaBundle);
    register("minecraft:light_blue_bundle", Material::LightBlueBundle);
    register("minecraft:yellow_bundle", Material::YellowBundle);
    register("minecraft:lime_bundle", Material::LimeBundle);
    register("minecraft:pink_bundle", Material::PinkBundle);
    register("minecraft:gray_bundle", Material::GrayBundle);
    register("minecraft:light_gray_bundle", Material::LightGrayBundle);
    register("minecraft:cyan_bundle", Material::CyanBundle);
    register("minecraft:purple_bundle", Material::PurpleBundle);
    register("minecraft:blue_bundle", Material::BlueBundle);
    register("minecraft:brown_bundle", Material::BrownBundle);
    register("minecraft:green_bundle", Material::GreenBundle);
    register("minecraft:red_bundle", Material::RedBundle);
    register("minecraft:black_bundle", Material::BlackBundle);
    register("minecraft:fishing_rod", Material::FishingRod);
    register("minecraft:clock", Material::Clock);
    register("minecraft:spyglass", Material::Spyglass);
    register("minecraft:glowstone_dust", Material::GlowstoneDust);
    register("minecraft:cod", Material::Cod);
    register("minecraft:salmon", Material::Salmon);
    register("minecraft:tropical_fish", Material::TropicalFish);
    register("minecraft:pufferfish", Material::Pufferfish);
    register("minecraft:cooked_cod", Material::CookedCod);
    register("minecraft:cooked_salmon", Material::CookedSalmon);
    register("minecraft:ink_sac", Material::InkSac);
    register("minecraft:glow_ink_sac", Material::GlowInkSac);
    register("minecraft:cocoa_beans", Material::CocoaBeans);
    register("minecraft:white_dye", Material::WhiteDye);
    register("minecraft:orange_dye", Material::OrangeDye);
    register("minecraft:magenta_dye", Material::MagentaDye);
    register("minecraft:light_blue_dye", Material::LightBlueDye);
    register("minecraft:yellow_dye", Material::YellowDye);
    register("minecraft:lime_dye", Material::LimeDye);
    register("minecraft:pink_dye", Material::PinkDye);
    register("minecraft:gray_dye", Material::GrayDye);
    register("minecraft:light_gray_dye", Material::LightGrayDye);
    register("minecraft:cyan_dye", Material::CyanDye);
    register("minecraft:purple_dye", Material::PurpleDye);
    register("minecraft:blue_dye", Material::BlueDye);
    register("minecraft:brown_dye", Material::BrownDye);
    register("minecraft:green_dye", Material::GreenDye);
    register("minecraft:red_dye", Material::RedDye);
    register("minecraft:black_dye", Material::BlackDye);
    register("minecraft:bone_meal", Material::BoneMeal);
    register("minecraft:bone", Material::Bone);
    register("minecraft:sugar", Material::Sugar);
    register("minecraft:cake", Material::Cake);
    register("minecraft:white_bed", Material::WhiteBed);
    register("minecraft:orange_bed", Material::OrangeBed);
    register("minecraft:magenta_bed", Material::MagentaBed);
    register("minecraft:light_blue_bed", Material::LightBlueBed);
    register("minecraft:yellow_bed", Material::YellowBed);
    register("minecraft:lime_bed", Material::LimeBed);
    register("minecraft:pink_bed", Material::PinkBed);
    register("minecraft:gray_bed", Material::GrayBed);
    register("minecraft:light_gray_bed", Material::LightGrayBed);
    register("minecraft:cyan_bed", Material::CyanBed);
    register("minecraft:purple_bed", Material::PurpleBed);
    register("minecraft:blue_bed", Material::BlueBed);
    register("minecraft:brown_bed", Material::BrownBed);
    register("minecraft:green_bed", Material::GreenBed);
    register("minecraft:red_bed", Material::RedBed);
    register("minecraft:black_bed", Material::BlackBed);
    register("minecraft:cookie", Material::Cookie);
    register("minecraft:crafter", Material::Crafter);
    register("minecraft:filled_map", Material::FilledMap);
    register("minecraft:shears", Material::Shears);
    register("minecraft:melon_slice", Material::MelonSlice);
    register("minecraft:dried_kelp", Material::DriedKelp);
    register("minecraft:pumpkin_seeds", Material::PumpkinSeeds);
    register("minecraft:melon_seeds", Material::MelonSeeds);
    register("minecraft:beef", Material::Beef);
    register("minecraft:cooked_beef", Material::CookedBeef);
    register("minecraft:chicken", Material::Chicken);
    register("minecraft:cooked_chicken", Material::CookedChicken);
    register("minecraft:rotten_flesh", Material::RottenFlesh);
    register("minecraft:ender_pearl", Material::EnderPearl);
    register("minecraft:blaze_rod", Material::BlazeRod);
    register("minecraft:ghast_tear", Material::GhastTear);
    register("minecraft:gold_nugget", Material::GoldNugget);
    register("minecraft:nether_wart", Material::NetherWart);
    register("minecraft:glass_bottle", Material::GlassBottle);
    register("minecraft:potion", Material::Potion);
    register("minecraft:spider_eye", Material::SpiderEye);
    register(
        "minecraft:fermented_spider_eye",
        Material::FermentedSpiderEye,
    );
    register("minecraft:blaze_powder", Material::BlazePowder);
    register("minecraft:magma_cream", Material::MagmaCream);
    register("minecraft:brewing_stand", Material::BrewingStand);
    register("minecraft:cauldron", Material::Cauldron);
    register("minecraft:ender_eye", Material::EnderEye);
    register(
        "minecraft:glistering_melon_slice",
        Material::GlisteringMelonSlice,
    );
    register("minecraft:chicken_spawn_egg", Material::ChickenSpawnEgg);
    register("minecraft:cow_spawn_egg", Material::CowSpawnEgg);
    register("minecraft:pig_spawn_egg", Material::PigSpawnEgg);
    register("minecraft:sheep_spawn_egg", Material::SheepSpawnEgg);
    register("minecraft:camel_spawn_egg", Material::CamelSpawnEgg);
    register("minecraft:donkey_spawn_egg", Material::DonkeySpawnEgg);
    register("minecraft:horse_spawn_egg", Material::HorseSpawnEgg);
    register("minecraft:mule_spawn_egg", Material::MuleSpawnEgg);
    register("minecraft:cat_spawn_egg", Material::CatSpawnEgg);
    register("minecraft:parrot_spawn_egg", Material::ParrotSpawnEgg);
    register("minecraft:wolf_spawn_egg", Material::WolfSpawnEgg);
    register("minecraft:armadillo_spawn_egg", Material::ArmadilloSpawnEgg);
    register("minecraft:bat_spawn_egg", Material::BatSpawnEgg);
    register("minecraft:bee_spawn_egg", Material::BeeSpawnEgg);
    register("minecraft:fox_spawn_egg", Material::FoxSpawnEgg);
    register("minecraft:goat_spawn_egg", Material::GoatSpawnEgg);
    register("minecraft:llama_spawn_egg", Material::LlamaSpawnEgg);
    register("minecraft:ocelot_spawn_egg", Material::OcelotSpawnEgg);
    register("minecraft:panda_spawn_egg", Material::PandaSpawnEgg);
    register(
        "minecraft:polar_bear_spawn_egg",
        Material::PolarBearSpawnEgg,
    );
    register("minecraft:rabbit_spawn_egg", Material::RabbitSpawnEgg);
    register("minecraft:axolotl_spawn_egg", Material::AxolotlSpawnEgg);
    register("minecraft:cod_spawn_egg", Material::CodSpawnEgg);
    register("minecraft:dolphin_spawn_egg", Material::DolphinSpawnEgg);
    register("minecraft:frog_spawn_egg", Material::FrogSpawnEgg);
    register(
        "minecraft:glow_squid_spawn_egg",
        Material::GlowSquidSpawnEgg,
    );
    register("minecraft:nautilus_spawn_egg", Material::NautilusSpawnEgg);
    register(
        "minecraft:pufferfish_spawn_egg",
        Material::PufferfishSpawnEgg,
    );
    register("minecraft:salmon_spawn_egg", Material::SalmonSpawnEgg);
    register("minecraft:squid_spawn_egg", Material::SquidSpawnEgg);
    register("minecraft:tadpole_spawn_egg", Material::TadpoleSpawnEgg);
    register(
        "minecraft:tropical_fish_spawn_egg",
        Material::TropicalFishSpawnEgg,
    );
    register("minecraft:turtle_spawn_egg", Material::TurtleSpawnEgg);
    register("minecraft:allay_spawn_egg", Material::AllaySpawnEgg);
    register("minecraft:mooshroom_spawn_egg", Material::MooshroomSpawnEgg);
    register("minecraft:sniffer_spawn_egg", Material::SnifferSpawnEgg);
    register(
        "minecraft:sulfur_cube_spawn_egg",
        Material::SulfurCubeSpawnEgg,
    );
    register(
        "minecraft:copper_golem_spawn_egg",
        Material::CopperGolemSpawnEgg,
    );
    register(
        "minecraft:iron_golem_spawn_egg",
        Material::IronGolemSpawnEgg,
    );
    register(
        "minecraft:snow_golem_spawn_egg",
        Material::SnowGolemSpawnEgg,
    );
    register(
        "minecraft:trader_llama_spawn_egg",
        Material::TraderLlamaSpawnEgg,
    );
    register("minecraft:villager_spawn_egg", Material::VillagerSpawnEgg);
    register(
        "minecraft:wandering_trader_spawn_egg",
        Material::WanderingTraderSpawnEgg,
    );
    register("minecraft:bogged_spawn_egg", Material::BoggedSpawnEgg);
    register(
        "minecraft:camel_husk_spawn_egg",
        Material::CamelHuskSpawnEgg,
    );
    register("minecraft:drowned_spawn_egg", Material::DrownedSpawnEgg);
    register("minecraft:husk_spawn_egg", Material::HuskSpawnEgg);
    register("minecraft:parched_spawn_egg", Material::ParchedSpawnEgg);
    register("minecraft:skeleton_spawn_egg", Material::SkeletonSpawnEgg);
    register(
        "minecraft:skeleton_horse_spawn_egg",
        Material::SkeletonHorseSpawnEgg,
    );
    register("minecraft:stray_spawn_egg", Material::StraySpawnEgg);
    register("minecraft:wither_spawn_egg", Material::WitherSpawnEgg);
    register(
        "minecraft:wither_skeleton_spawn_egg",
        Material::WitherSkeletonSpawnEgg,
    );
    register("minecraft:zombie_spawn_egg", Material::ZombieSpawnEgg);
    register(
        "minecraft:zombie_horse_spawn_egg",
        Material::ZombieHorseSpawnEgg,
    );
    register(
        "minecraft:zombie_nautilus_spawn_egg",
        Material::ZombieNautilusSpawnEgg,
    );
    register(
        "minecraft:zombie_villager_spawn_egg",
        Material::ZombieVillagerSpawnEgg,
    );
    register(
        "minecraft:cave_spider_spawn_egg",
        Material::CaveSpiderSpawnEgg,
    );
    register("minecraft:spider_spawn_egg", Material::SpiderSpawnEgg);
    register("minecraft:breeze_spawn_egg", Material::BreezeSpawnEgg);
    register("minecraft:creaking_spawn_egg", Material::CreakingSpawnEgg);
    register("minecraft:creeper_spawn_egg", Material::CreeperSpawnEgg);
    register(
        "minecraft:elder_guardian_spawn_egg",
        Material::ElderGuardianSpawnEgg,
    );
    register("minecraft:guardian_spawn_egg", Material::GuardianSpawnEgg);
    register("minecraft:phantom_spawn_egg", Material::PhantomSpawnEgg);
    register(
        "minecraft:silverfish_spawn_egg",
        Material::SilverfishSpawnEgg,
    );
    register("minecraft:slime_spawn_egg", Material::SlimeSpawnEgg);
    register("minecraft:warden_spawn_egg", Material::WardenSpawnEgg);
    register("minecraft:witch_spawn_egg", Material::WitchSpawnEgg);
    register("minecraft:evoker_spawn_egg", Material::EvokerSpawnEgg);
    register("minecraft:pillager_spawn_egg", Material::PillagerSpawnEgg);
    register("minecraft:ravager_spawn_egg", Material::RavagerSpawnEgg);
    register(
        "minecraft:vindicator_spawn_egg",
        Material::VindicatorSpawnEgg,
    );
    register("minecraft:vex_spawn_egg", Material::VexSpawnEgg);
    register("minecraft:blaze_spawn_egg", Material::BlazeSpawnEgg);
    register("minecraft:ghast_spawn_egg", Material::GhastSpawnEgg);
    register(
        "minecraft:happy_ghast_spawn_egg",
        Material::HappyGhastSpawnEgg,
    );
    register("minecraft:hoglin_spawn_egg", Material::HoglinSpawnEgg);
    register(
        "minecraft:magma_cube_spawn_egg",
        Material::MagmaCubeSpawnEgg,
    );
    register("minecraft:piglin_spawn_egg", Material::PiglinSpawnEgg);
    register(
        "minecraft:piglin_brute_spawn_egg",
        Material::PiglinBruteSpawnEgg,
    );
    register("minecraft:strider_spawn_egg", Material::StriderSpawnEgg);
    register("minecraft:zoglin_spawn_egg", Material::ZoglinSpawnEgg);
    register(
        "minecraft:zombified_piglin_spawn_egg",
        Material::ZombifiedPiglinSpawnEgg,
    );
    register(
        "minecraft:ender_dragon_spawn_egg",
        Material::EnderDragonSpawnEgg,
    );
    register("minecraft:enderman_spawn_egg", Material::EndermanSpawnEgg);
    register("minecraft:endermite_spawn_egg", Material::EndermiteSpawnEgg);
    register("minecraft:shulker_spawn_egg", Material::ShulkerSpawnEgg);
    register("minecraft:experience_bottle", Material::ExperienceBottle);
    register("minecraft:fire_charge", Material::FireCharge);
    register("minecraft:wind_charge", Material::WindCharge);
    register("minecraft:writable_book", Material::WritableBook);
    register("minecraft:written_book", Material::WrittenBook);
    register("minecraft:breeze_rod", Material::BreezeRod);
    register("minecraft:mace", Material::Mace);
    register("minecraft:item_frame", Material::ItemFrame);
    register("minecraft:glow_item_frame", Material::GlowItemFrame);
    register("minecraft:flower_pot", Material::FlowerPot);
    register("minecraft:carrot", Material::Carrot);
    register("minecraft:potato", Material::Potato);
    register("minecraft:baked_potato", Material::BakedPotato);
    register("minecraft:poisonous_potato", Material::PoisonousPotato);
    register("minecraft:map", Material::Map);
    register("minecraft:golden_carrot", Material::GoldenCarrot);
    register("minecraft:skeleton_skull", Material::SkeletonSkull);
    register(
        "minecraft:wither_skeleton_skull",
        Material::WitherSkeletonSkull,
    );
    register("minecraft:player_head", Material::PlayerHead);
    register("minecraft:zombie_head", Material::ZombieHead);
    register("minecraft:creeper_head", Material::CreeperHead);
    register("minecraft:dragon_head", Material::DragonHead);
    register("minecraft:piglin_head", Material::PiglinHead);
    register("minecraft:nether_star", Material::NetherStar);
    register("minecraft:pumpkin_pie", Material::PumpkinPie);
    register("minecraft:firework_rocket", Material::FireworkRocket);
    register("minecraft:firework_star", Material::FireworkStar);
    register("minecraft:enchanted_book", Material::EnchantedBook);
    register("minecraft:nether_brick", Material::NetherBrick);
    register("minecraft:resin_brick", Material::ResinBrick);
    register("minecraft:prismarine_shard", Material::PrismarineShard);
    register(
        "minecraft:prismarine_crystals",
        Material::PrismarineCrystals,
    );
    register("minecraft:rabbit", Material::Rabbit);
    register("minecraft:cooked_rabbit", Material::CookedRabbit);
    register("minecraft:rabbit_stew", Material::RabbitStew);
    register("minecraft:rabbit_foot", Material::RabbitFoot);
    register("minecraft:rabbit_hide", Material::RabbitHide);
    register("minecraft:armor_stand", Material::ArmorStand);
    register("minecraft:copper_horse_armor", Material::CopperHorseArmor);
    register("minecraft:iron_horse_armor", Material::IronHorseArmor);
    register("minecraft:golden_horse_armor", Material::GoldenHorseArmor);
    register("minecraft:diamond_horse_armor", Material::DiamondHorseArmor);
    register(
        "minecraft:netherite_horse_armor",
        Material::NetheriteHorseArmor,
    );
    register("minecraft:leather_horse_armor", Material::LeatherHorseArmor);
    register("minecraft:lead", Material::Lead);
    register("minecraft:name_tag", Material::NameTag);
    register(
        "minecraft:command_block_minecart",
        Material::CommandBlockMinecart,
    );
    register("minecraft:mutton", Material::Mutton);
    register("minecraft:cooked_mutton", Material::CookedMutton);
    register("minecraft:white_banner", Material::WhiteBanner);
    register("minecraft:orange_banner", Material::OrangeBanner);
    register("minecraft:magenta_banner", Material::MagentaBanner);
    register("minecraft:light_blue_banner", Material::LightBlueBanner);
    register("minecraft:yellow_banner", Material::YellowBanner);
    register("minecraft:lime_banner", Material::LimeBanner);
    register("minecraft:pink_banner", Material::PinkBanner);
    register("minecraft:gray_banner", Material::GrayBanner);
    register("minecraft:light_gray_banner", Material::LightGrayBanner);
    register("minecraft:cyan_banner", Material::CyanBanner);
    register("minecraft:purple_banner", Material::PurpleBanner);
    register("minecraft:blue_banner", Material::BlueBanner);
    register("minecraft:brown_banner", Material::BrownBanner);
    register("minecraft:green_banner", Material::GreenBanner);
    register("minecraft:red_banner", Material::RedBanner);
    register("minecraft:black_banner", Material::BlackBanner);
    register("minecraft:end_crystal", Material::EndCrystal);
    register("minecraft:chorus_fruit", Material::ChorusFruit);
    register("minecraft:popped_chorus_fruit", Material::PoppedChorusFruit);
    register("minecraft:torchflower_seeds", Material::TorchflowerSeeds);
    register("minecraft:pitcher_pod", Material::PitcherPod);
    register("minecraft:beetroot", Material::Beetroot);
    register("minecraft:beetroot_seeds", Material::BeetrootSeeds);
    register("minecraft:beetroot_soup", Material::BeetrootSoup);
    register("minecraft:dragon_breath", Material::DragonBreath);
    register("minecraft:splash_potion", Material::SplashPotion);
    register("minecraft:spectral_arrow", Material::SpectralArrow);
    register("minecraft:tipped_arrow", Material::TippedArrow);
    register("minecraft:lingering_potion", Material::LingeringPotion);
    register("minecraft:shield", Material::Shield);
    register("minecraft:wooden_spear", Material::WoodenSpear);
    register("minecraft:stone_spear", Material::StoneSpear);
    register("minecraft:copper_spear", Material::CopperSpear);
    register("minecraft:iron_spear", Material::IronSpear);
    register("minecraft:golden_spear", Material::GoldenSpear);
    register("minecraft:diamond_spear", Material::DiamondSpear);
    register("minecraft:netherite_spear", Material::NetheriteSpear);
    register("minecraft:totem_of_undying", Material::TotemOfUndying);
    register("minecraft:shulker_shell", Material::ShulkerShell);
    register("minecraft:iron_nugget", Material::IronNugget);
    register("minecraft:copper_nugget", Material::CopperNugget);
    register("minecraft:knowledge_book", Material::KnowledgeBook);
    register("minecraft:debug_stick", Material::DebugStick);
    register("minecraft:music_disc_13", Material::MusicDisc13);
    register("minecraft:music_disc_cat", Material::MusicDiscCat);
    register("minecraft:music_disc_blocks", Material::MusicDiscBlocks);
    register("minecraft:music_disc_bounce", Material::MusicDiscBounce);
    register("minecraft:music_disc_chirp", Material::MusicDiscChirp);
    register("minecraft:music_disc_creator", Material::MusicDiscCreator);
    register(
        "minecraft:music_disc_creator_music_box",
        Material::MusicDiscCreatorMusicBox,
    );
    register("minecraft:music_disc_far", Material::MusicDiscFar);
    register(
        "minecraft:music_disc_lava_chicken",
        Material::MusicDiscLavaChicken,
    );
    register("minecraft:music_disc_mall", Material::MusicDiscMall);
    register("minecraft:music_disc_mellohi", Material::MusicDiscMellohi);
    register("minecraft:music_disc_stal", Material::MusicDiscStal);
    register("minecraft:music_disc_strad", Material::MusicDiscStrad);
    register("minecraft:music_disc_ward", Material::MusicDiscWard);
    register("minecraft:music_disc_11", Material::MusicDisc11);
    register("minecraft:music_disc_wait", Material::MusicDiscWait);
    register(
        "minecraft:music_disc_otherside",
        Material::MusicDiscOtherside,
    );
    register("minecraft:music_disc_relic", Material::MusicDiscRelic);
    register("minecraft:music_disc_5", Material::MusicDisc5);
    register("minecraft:music_disc_pigstep", Material::MusicDiscPigstep);
    register(
        "minecraft:music_disc_precipice",
        Material::MusicDiscPrecipice,
    );
    register("minecraft:music_disc_tears", Material::MusicDiscTears);
    register("minecraft:disc_fragment_5", Material::DiscFragment5);
    register("minecraft:trident", Material::Trident);
    register("minecraft:nautilus_shell", Material::NautilusShell);
    register("minecraft:iron_nautilus_armor", Material::IronNautilusArmor);
    register(
        "minecraft:golden_nautilus_armor",
        Material::GoldenNautilusArmor,
    );
    register(
        "minecraft:diamond_nautilus_armor",
        Material::DiamondNautilusArmor,
    );
    register(
        "minecraft:netherite_nautilus_armor",
        Material::NetheriteNautilusArmor,
    );
    register(
        "minecraft:copper_nautilus_armor",
        Material::CopperNautilusArmor,
    );
    register("minecraft:heart_of_the_sea", Material::HeartOfTheSea);
    register("minecraft:crossbow", Material::Crossbow);
    register("minecraft:suspicious_stew", Material::SuspiciousStew);
    register("minecraft:loom", Material::Loom);
    register(
        "minecraft:flower_banner_pattern",
        Material::FlowerBannerPattern,
    );
    register(
        "minecraft:creeper_banner_pattern",
        Material::CreeperBannerPattern,
    );
    register(
        "minecraft:skull_banner_pattern",
        Material::SkullBannerPattern,
    );
    register(
        "minecraft:mojang_banner_pattern",
        Material::MojangBannerPattern,
    );
    register(
        "minecraft:globe_banner_pattern",
        Material::GlobeBannerPattern,
    );
    register(
        "minecraft:piglin_banner_pattern",
        Material::PiglinBannerPattern,
    );
    register("minecraft:flow_banner_pattern", Material::FlowBannerPattern);
    register(
        "minecraft:guster_banner_pattern",
        Material::GusterBannerPattern,
    );
    register(
        "minecraft:field_masoned_banner_pattern",
        Material::FieldMasonedBannerPattern,
    );
    register(
        "minecraft:bordure_indented_banner_pattern",
        Material::BordureIndentedBannerPattern,
    );
    register("minecraft:goat_horn", Material::GoatHorn);
    register("minecraft:composter", Material::Composter);
    register("minecraft:barrel", Material::Barrel);
    register("minecraft:smoker", Material::Smoker);
    register("minecraft:blast_furnace", Material::BlastFurnace);
    register("minecraft:cartography_table", Material::CartographyTable);
    register("minecraft:fletching_table", Material::FletchingTable);
    register("minecraft:grindstone", Material::Grindstone);
    register("minecraft:smithing_table", Material::SmithingTable);
    register("minecraft:stonecutter", Material::Stonecutter);
    register("minecraft:bell", Material::Bell);
    register("minecraft:lantern", Material::Lantern);
    register("minecraft:soul_lantern", Material::SoulLantern);
    register("minecraft:copper_lantern", Material::CopperLantern);
    register(
        "minecraft:exposed_copper_lantern",
        Material::ExposedCopperLantern,
    );
    register(
        "minecraft:weathered_copper_lantern",
        Material::WeatheredCopperLantern,
    );
    register(
        "minecraft:oxidized_copper_lantern",
        Material::OxidizedCopperLantern,
    );
    register(
        "minecraft:waxed_copper_lantern",
        Material::WaxedCopperLantern,
    );
    register(
        "minecraft:waxed_exposed_copper_lantern",
        Material::WaxedExposedCopperLantern,
    );
    register(
        "minecraft:waxed_weathered_copper_lantern",
        Material::WaxedWeatheredCopperLantern,
    );
    register(
        "minecraft:waxed_oxidized_copper_lantern",
        Material::WaxedOxidizedCopperLantern,
    );
    register("minecraft:sweet_berries", Material::SweetBerries);
    register("minecraft:glow_berries", Material::GlowBerries);
    register("minecraft:campfire", Material::Campfire);
    register("minecraft:soul_campfire", Material::SoulCampfire);
    register("minecraft:shroomlight", Material::Shroomlight);
    register("minecraft:honeycomb", Material::Honeycomb);
    register("minecraft:bee_nest", Material::BeeNest);
    register("minecraft:beehive", Material::Beehive);
    register("minecraft:honey_bottle", Material::HoneyBottle);
    register("minecraft:honeycomb_block", Material::HoneycombBlock);
    register("minecraft:lodestone", Material::Lodestone);
    register("minecraft:crying_obsidian", Material::CryingObsidian);
    register("minecraft:blackstone", Material::Blackstone);
    register("minecraft:blackstone_slab", Material::BlackstoneSlab);
    register("minecraft:blackstone_stairs", Material::BlackstoneStairs);
    register("minecraft:gilded_blackstone", Material::GildedBlackstone);
    register(
        "minecraft:polished_blackstone",
        Material::PolishedBlackstone,
    );
    register(
        "minecraft:polished_blackstone_slab",
        Material::PolishedBlackstoneSlab,
    );
    register(
        "minecraft:polished_blackstone_stairs",
        Material::PolishedBlackstoneStairs,
    );
    register(
        "minecraft:chiseled_polished_blackstone",
        Material::ChiseledPolishedBlackstone,
    );
    register(
        "minecraft:polished_blackstone_bricks",
        Material::PolishedBlackstoneBricks,
    );
    register(
        "minecraft:polished_blackstone_brick_slab",
        Material::PolishedBlackstoneBrickSlab,
    );
    register(
        "minecraft:polished_blackstone_brick_stairs",
        Material::PolishedBlackstoneBrickStairs,
    );
    register(
        "minecraft:cracked_polished_blackstone_bricks",
        Material::CrackedPolishedBlackstoneBricks,
    );
    register("minecraft:respawn_anchor", Material::RespawnAnchor);
    register("minecraft:candle", Material::Candle);
    register("minecraft:white_candle", Material::WhiteCandle);
    register("minecraft:orange_candle", Material::OrangeCandle);
    register("minecraft:magenta_candle", Material::MagentaCandle);
    register("minecraft:light_blue_candle", Material::LightBlueCandle);
    register("minecraft:yellow_candle", Material::YellowCandle);
    register("minecraft:lime_candle", Material::LimeCandle);
    register("minecraft:pink_candle", Material::PinkCandle);
    register("minecraft:gray_candle", Material::GrayCandle);
    register("minecraft:light_gray_candle", Material::LightGrayCandle);
    register("minecraft:cyan_candle", Material::CyanCandle);
    register("minecraft:purple_candle", Material::PurpleCandle);
    register("minecraft:blue_candle", Material::BlueCandle);
    register("minecraft:brown_candle", Material::BrownCandle);
    register("minecraft:green_candle", Material::GreenCandle);
    register("minecraft:red_candle", Material::RedCandle);
    register("minecraft:black_candle", Material::BlackCandle);
    register("minecraft:small_amethyst_bud", Material::SmallAmethystBud);
    register("minecraft:medium_amethyst_bud", Material::MediumAmethystBud);
    register("minecraft:large_amethyst_bud", Material::LargeAmethystBud);
    register("minecraft:amethyst_cluster", Material::AmethystCluster);
    register("minecraft:pointed_dripstone", Material::PointedDripstone);
    register("minecraft:sulfur_spike", Material::SulfurSpike);
    register("minecraft:ochre_froglight", Material::OchreFroglight);
    register("minecraft:verdant_froglight", Material::VerdantFroglight);
    register(
        "minecraft:pearlescent_froglight",
        Material::PearlescentFroglight,
    );
    register("minecraft:frogspawn", Material::Frogspawn);
    register("minecraft:echo_shard", Material::EchoShard);
    register("minecraft:brush", Material::Brush);
    register(
        "minecraft:netherite_upgrade_smithing_template",
        Material::NetheriteUpgradeSmithingTemplate,
    );
    register(
        "minecraft:sentry_armor_trim_smithing_template",
        Material::SentryArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:dune_armor_trim_smithing_template",
        Material::DuneArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:coast_armor_trim_smithing_template",
        Material::CoastArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:wild_armor_trim_smithing_template",
        Material::WildArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:ward_armor_trim_smithing_template",
        Material::WardArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:eye_armor_trim_smithing_template",
        Material::EyeArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:vex_armor_trim_smithing_template",
        Material::VexArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:tide_armor_trim_smithing_template",
        Material::TideArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:snout_armor_trim_smithing_template",
        Material::SnoutArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:rib_armor_trim_smithing_template",
        Material::RibArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:spire_armor_trim_smithing_template",
        Material::SpireArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:wayfinder_armor_trim_smithing_template",
        Material::WayfinderArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:shaper_armor_trim_smithing_template",
        Material::ShaperArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:silence_armor_trim_smithing_template",
        Material::SilenceArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:raiser_armor_trim_smithing_template",
        Material::RaiserArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:host_armor_trim_smithing_template",
        Material::HostArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:flow_armor_trim_smithing_template",
        Material::FlowArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:bolt_armor_trim_smithing_template",
        Material::BoltArmorTrimSmithingTemplate,
    );
    register(
        "minecraft:angler_pottery_sherd",
        Material::AnglerPotterySherd,
    );
    register(
        "minecraft:archer_pottery_sherd",
        Material::ArcherPotterySherd,
    );
    register(
        "minecraft:arms_up_pottery_sherd",
        Material::ArmsUpPotterySherd,
    );
    register("minecraft:blade_pottery_sherd", Material::BladePotterySherd);
    register(
        "minecraft:brewer_pottery_sherd",
        Material::BrewerPotterySherd,
    );
    register("minecraft:burn_pottery_sherd", Material::BurnPotterySherd);
    register(
        "minecraft:danger_pottery_sherd",
        Material::DangerPotterySherd,
    );
    register(
        "minecraft:explorer_pottery_sherd",
        Material::ExplorerPotterySherd,
    );
    register("minecraft:flow_pottery_sherd", Material::FlowPotterySherd);
    register(
        "minecraft:friend_pottery_sherd",
        Material::FriendPotterySherd,
    );
    register(
        "minecraft:guster_pottery_sherd",
        Material::GusterPotterySherd,
    );
    register("minecraft:heart_pottery_sherd", Material::HeartPotterySherd);
    register(
        "minecraft:heartbreak_pottery_sherd",
        Material::HeartbreakPotterySherd,
    );
    register("minecraft:howl_pottery_sherd", Material::HowlPotterySherd);
    register("minecraft:miner_pottery_sherd", Material::MinerPotterySherd);
    register(
        "minecraft:mourner_pottery_sherd",
        Material::MournerPotterySherd,
    );
    register(
        "minecraft:plenty_pottery_sherd",
        Material::PlentyPotterySherd,
    );
    register("minecraft:prize_pottery_sherd", Material::PrizePotterySherd);
    register(
        "minecraft:scrape_pottery_sherd",
        Material::ScrapePotterySherd,
    );
    register("minecraft:sheaf_pottery_sherd", Material::SheafPotterySherd);
    register(
        "minecraft:shelter_pottery_sherd",
        Material::ShelterPotterySherd,
    );
    register("minecraft:skull_pottery_sherd", Material::SkullPotterySherd);
    register("minecraft:snort_pottery_sherd", Material::SnortPotterySherd);
    register("minecraft:copper_grate", Material::CopperGrate);
    register(
        "minecraft:exposed_copper_grate",
        Material::ExposedCopperGrate,
    );
    register(
        "minecraft:weathered_copper_grate",
        Material::WeatheredCopperGrate,
    );
    register(
        "minecraft:oxidized_copper_grate",
        Material::OxidizedCopperGrate,
    );
    register("minecraft:waxed_copper_grate", Material::WaxedCopperGrate);
    register(
        "minecraft:waxed_exposed_copper_grate",
        Material::WaxedExposedCopperGrate,
    );
    register(
        "minecraft:waxed_weathered_copper_grate",
        Material::WaxedWeatheredCopperGrate,
    );
    register(
        "minecraft:waxed_oxidized_copper_grate",
        Material::WaxedOxidizedCopperGrate,
    );
    register("minecraft:copper_bulb", Material::CopperBulb);
    register("minecraft:exposed_copper_bulb", Material::ExposedCopperBulb);
    register(
        "minecraft:weathered_copper_bulb",
        Material::WeatheredCopperBulb,
    );
    register(
        "minecraft:oxidized_copper_bulb",
        Material::OxidizedCopperBulb,
    );
    register("minecraft:waxed_copper_bulb", Material::WaxedCopperBulb);
    register(
        "minecraft:waxed_exposed_copper_bulb",
        Material::WaxedExposedCopperBulb,
    );
    register(
        "minecraft:waxed_weathered_copper_bulb",
        Material::WaxedWeatheredCopperBulb,
    );
    register(
        "minecraft:waxed_oxidized_copper_bulb",
        Material::WaxedOxidizedCopperBulb,
    );
    register("minecraft:copper_chest", Material::CopperChest);
    register(
        "minecraft:exposed_copper_chest",
        Material::ExposedCopperChest,
    );
    register(
        "minecraft:weathered_copper_chest",
        Material::WeatheredCopperChest,
    );
    register(
        "minecraft:oxidized_copper_chest",
        Material::OxidizedCopperChest,
    );
    register("minecraft:waxed_copper_chest", Material::WaxedCopperChest);
    register(
        "minecraft:waxed_exposed_copper_chest",
        Material::WaxedExposedCopperChest,
    );
    register(
        "minecraft:waxed_weathered_copper_chest",
        Material::WaxedWeatheredCopperChest,
    );
    register(
        "minecraft:waxed_oxidized_copper_chest",
        Material::WaxedOxidizedCopperChest,
    );
    register("minecraft:copper_golem_statue", Material::CopperGolemStatue);
    register(
        "minecraft:exposed_copper_golem_statue",
        Material::ExposedCopperGolemStatue,
    );
    register(
        "minecraft:weathered_copper_golem_statue",
        Material::WeatheredCopperGolemStatue,
    );
    register(
        "minecraft:oxidized_copper_golem_statue",
        Material::OxidizedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_copper_golem_statue",
        Material::WaxedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_exposed_copper_golem_statue",
        Material::WaxedExposedCopperGolemStatue,
    );
    register(
        "minecraft:waxed_weathered_copper_golem_statue",
        Material::WaxedWeatheredCopperGolemStatue,
    );
    register(
        "minecraft:waxed_oxidized_copper_golem_statue",
        Material::WaxedOxidizedCopperGolemStatue,
    );
    register("minecraft:trial_spawner", Material::TrialSpawner);
    register("minecraft:trial_key", Material::TrialKey);
    register("minecraft:ominous_trial_key", Material::OminousTrialKey);
    register("minecraft:vault", Material::Vault);
    register("minecraft:ominous_bottle", Material::OminousBottle);
}
