use std::borrow::Cow;

use cerium_macros::StaticObject;
use cerium_macros::UnitEnum;

use crate::registry::{Id, Registries, Registry};
use crate::util::Key;
use crate::world::Particle;
use crate::world::attribute::{
    AmbientSounds, AttributeType, AttributeValue, BackgroundMusic, BedRule, BedRuleKind, MoonPhase,
    TriState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StaticObject, UnitEnum)]
#[repr(u16)]
pub enum EnvironmentAttribute {
    FogColor,
    FogStartDistance,
    FogEndDistance,
    SkyFogEndDistance,
    CloudFogEndDistance,
    WaterFogColor,
    WaterFogStartDistance,
    WaterFogEndDistance,
    SkyColor,
    SunriseSunsetColor,
    CloudColor,
    CloudHeight,
    SunAngle,
    MoonAngle,
    StarAngle,
    MoonPhase,
    StarBrightness,
    BlockLightTint,
    SkyLightColor,
    SkyLightFactor,
    NightVisionColor,
    AmbientLightColor,
    DefaultDripstoneParticle,
    AmbientParticles,
    BackgroundMusic,
    MusicVolume,
    AmbientSounds,
    FireflyBushSounds,
    SkyLightLevel,
    CanStartRaid,
    WaterEvaporates,
    BedRule,
    RespawnAnchorWorks,
    NetherPortalSpawnsPiglin,
    FastLava,
    IncreasedFireBurnout,
    EyeblossomOpen,
    TurtleEggHatchChance,
    PiglinsZombify,
    SnowGolemMelts,
    CreakingActive,
    SurfaceSlimeSpawnChance,
    CatWakingUpGiftChance,
    BeesStayInHive,
    MonstersBurn,
    CanPillagerPatrolSpawn,
    VillagerActivity,
    BabyVillagerActivity,
}

pub struct EnvironmentAttributeData {
    pub key: Key,
    pub r#type: AttributeType,
    pub default_value: AttributeValue,
}

impl EnvironmentAttribute {
    pub fn key(&self) -> &'static Key {
        &self.data().key
    }

    pub fn r#type(&self) -> AttributeType {
        self.data().r#type
    }

    pub fn default_value(&self) -> &'static AttributeValue {
        &self.data().default_value
    }

    pub fn from_id(id: Id) -> Option<EnvironmentAttribute> {
        Self::all().get(id as usize).copied()
    }

    pub fn from_key(key: &Key) -> Option<EnvironmentAttribute> {
        Registries::ENVIRONMENT_ATTRIBUTE.by_key(key).copied()
    }
}

const fn attribute(
    path: &'static str,
    r#type: AttributeType,
    default_value: AttributeValue,
) -> EnvironmentAttributeData {
    EnvironmentAttributeData {
        key: Key::const_vanilla(path),
        r#type,
        default_value,
    }
}

const fn color(value: &'static str) -> AttributeValue {
    AttributeValue::Color(Cow::Borrowed(value))
}

#[rustfmt::skip]
pub static FOG_COLOR: EnvironmentAttributeData = attribute("visual/fog_color", AttributeType::RgbColor, color("#000000"));
#[rustfmt::skip]
pub static FOG_START_DISTANCE: EnvironmentAttributeData = attribute("visual/fog_start_distance", AttributeType::Float, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static FOG_END_DISTANCE: EnvironmentAttributeData = attribute("visual/fog_end_distance", AttributeType::Float, AttributeValue::Float(1024.0));
#[rustfmt::skip]
pub static SKY_FOG_END_DISTANCE: EnvironmentAttributeData = attribute("visual/sky_fog_end_distance", AttributeType::Float, AttributeValue::Float(512.0));
#[rustfmt::skip]
pub static CLOUD_FOG_END_DISTANCE: EnvironmentAttributeData = attribute("visual/cloud_fog_end_distance", AttributeType::Float, AttributeValue::Float(2048.0));
#[rustfmt::skip]
pub static WATER_FOG_COLOR: EnvironmentAttributeData = attribute("visual/water_fog_color", AttributeType::RgbColor, color("#050533"));
#[rustfmt::skip]
pub static WATER_FOG_START_DISTANCE: EnvironmentAttributeData = attribute("visual/water_fog_start_distance", AttributeType::Float, AttributeValue::Float(-8.0));
#[rustfmt::skip]
pub static WATER_FOG_END_DISTANCE: EnvironmentAttributeData = attribute("visual/water_fog_end_distance", AttributeType::Float, AttributeValue::Float(96.0));
#[rustfmt::skip]
pub static SKY_COLOR: EnvironmentAttributeData = attribute("visual/sky_color", AttributeType::RgbColor, color("#000000"));
#[rustfmt::skip]
pub static SUNRISE_SUNSET_COLOR: EnvironmentAttributeData = attribute("visual/sunrise_sunset_color", AttributeType::ArgbColor, color("#00000000"));
#[rustfmt::skip]
pub static CLOUD_COLOR: EnvironmentAttributeData = attribute("visual/cloud_color", AttributeType::ArgbColor, color("#00000000"));
#[rustfmt::skip]
pub static CLOUD_HEIGHT: EnvironmentAttributeData = attribute("visual/cloud_height", AttributeType::Float, AttributeValue::Float(192.33));
#[rustfmt::skip]
pub static SUN_ANGLE: EnvironmentAttributeData = attribute("visual/sun_angle", AttributeType::AngleDegrees, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static MOON_ANGLE: EnvironmentAttributeData = attribute("visual/moon_angle", AttributeType::AngleDegrees, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static STAR_ANGLE: EnvironmentAttributeData = attribute("visual/star_angle", AttributeType::AngleDegrees, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static MOON_PHASE: EnvironmentAttributeData = attribute("visual/moon_phase", AttributeType::MoonPhase, AttributeValue::MoonPhase(MoonPhase::FullMoon));
#[rustfmt::skip]
pub static STAR_BRIGHTNESS: EnvironmentAttributeData = attribute("visual/star_brightness", AttributeType::Float, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static BLOCK_LIGHT_TINT: EnvironmentAttributeData = attribute("visual/block_light_tint", AttributeType::RgbColor, color("#FFD88C"));
#[rustfmt::skip]
pub static SKY_LIGHT_COLOR: EnvironmentAttributeData = attribute("visual/sky_light_color", AttributeType::RgbColor, color("#FFFFFF"));
#[rustfmt::skip]
pub static SKY_LIGHT_FACTOR: EnvironmentAttributeData = attribute("visual/sky_light_factor", AttributeType::Float, AttributeValue::Float(1.0));
#[rustfmt::skip]
pub static NIGHT_VISION_COLOR: EnvironmentAttributeData = attribute("visual/night_vision_color", AttributeType::RgbColor, color("#999999"));
#[rustfmt::skip]
pub static AMBIENT_LIGHT_COLOR: EnvironmentAttributeData = attribute("visual/ambient_light_color", AttributeType::RgbColor, color("#000000"));
#[rustfmt::skip]
pub static DEFAULT_DRIPSTONE_PARTICLE: EnvironmentAttributeData = attribute("visual/default_dripstone_particle", AttributeType::Particle, AttributeValue::Particle(Particle { kind: Cow::Borrowed("minecraft:dripping_dripstone_water"), options: std::collections::BTreeMap::new() }));
#[rustfmt::skip]
pub static AMBIENT_PARTICLES: EnvironmentAttributeData = attribute("visual/ambient_particles", AttributeType::AmbientParticles, AttributeValue::AmbientParticles(Vec::new()));
#[rustfmt::skip]
pub static BACKGROUND_MUSIC: EnvironmentAttributeData = attribute("audio/background_music", AttributeType::BackgroundMusic, AttributeValue::BackgroundMusic(BackgroundMusic { default_music: None, creative_music: None, underwater_music: None }));
#[rustfmt::skip]
pub static MUSIC_VOLUME: EnvironmentAttributeData = attribute("audio/music_volume", AttributeType::Float, AttributeValue::Float(1.0));
#[rustfmt::skip]
pub static AMBIENT_SOUNDS: EnvironmentAttributeData = attribute("audio/ambient_sounds", AttributeType::AmbientSounds, AttributeValue::AmbientSounds(AmbientSounds { sound_loop: None, mood: None, additions: None }));
#[rustfmt::skip]
pub static FIREFLY_BUSH_SOUNDS: EnvironmentAttributeData = attribute("audio/firefly_bush_sounds", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static SKY_LIGHT_LEVEL: EnvironmentAttributeData = attribute("gameplay/sky_light_level", AttributeType::Float, AttributeValue::Float(15.0));
#[rustfmt::skip]
pub static CAN_START_RAID: EnvironmentAttributeData = attribute("gameplay/can_start_raid", AttributeType::Boolean, AttributeValue::Boolean(true));
#[rustfmt::skip]
pub static WATER_EVAPORATES: EnvironmentAttributeData = attribute("gameplay/water_evaporates", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static BED_RULE: EnvironmentAttributeData = attribute("gameplay/bed_rule", AttributeType::BedRule, AttributeValue::BedRule(BedRule { can_sleep: BedRuleKind::WhenDark, can_set_spawn: BedRuleKind::Always, explodes: None, error_message: None }));
#[rustfmt::skip]
pub static RESPAWN_ANCHOR_WORKS: EnvironmentAttributeData = attribute("gameplay/respawn_anchor_works", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static NETHER_PORTAL_SPAWNS_PIGLIN: EnvironmentAttributeData = attribute("gameplay/nether_portal_spawns_piglin", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static FAST_LAVA: EnvironmentAttributeData = attribute("gameplay/fast_lava", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static INCREASED_FIRE_BURNOUT: EnvironmentAttributeData = attribute("gameplay/increased_fire_burnout", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static EYEBLOSSOM_OPEN: EnvironmentAttributeData = attribute("gameplay/eyeblossom_open", AttributeType::TriState, AttributeValue::TriState(TriState::Default));
#[rustfmt::skip]
pub static TURTLE_EGG_HATCH_CHANCE: EnvironmentAttributeData = attribute("gameplay/turtle_egg_hatch_chance", AttributeType::Float, AttributeValue::Float(0.002));
#[rustfmt::skip]
pub static PIGLINS_ZOMBIFY: EnvironmentAttributeData = attribute("gameplay/piglins_zombify", AttributeType::Boolean, AttributeValue::Boolean(true));
#[rustfmt::skip]
pub static SNOW_GOLEM_MELTS: EnvironmentAttributeData = attribute("gameplay/snow_golem_melts", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static CREAKING_ACTIVE: EnvironmentAttributeData = attribute("gameplay/creaking_active", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static SURFACE_SLIME_SPAWN_CHANCE: EnvironmentAttributeData = attribute("gameplay/surface_slime_spawn_chance", AttributeType::Float, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static CAT_WAKING_UP_GIFT_CHANCE: EnvironmentAttributeData = attribute("gameplay/cat_waking_up_gift_chance", AttributeType::Float, AttributeValue::Float(0.0));
#[rustfmt::skip]
pub static BEES_STAY_IN_HIVE: EnvironmentAttributeData = attribute("gameplay/bees_stay_in_hive", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static MONSTERS_BURN: EnvironmentAttributeData = attribute("gameplay/monsters_burn", AttributeType::Boolean, AttributeValue::Boolean(false));
#[rustfmt::skip]
pub static CAN_PILLAGER_PATROL_SPAWN: EnvironmentAttributeData = attribute("gameplay/can_pillager_patrol_spawn", AttributeType::Boolean, AttributeValue::Boolean(true));
#[rustfmt::skip]
pub static VILLAGER_ACTIVITY: EnvironmentAttributeData = attribute("gameplay/villager_activity", AttributeType::Activity, AttributeValue::Activity(Key::const_vanilla("idle")));
#[rustfmt::skip]
pub static BABY_VILLAGER_ACTIVITY: EnvironmentAttributeData = attribute("gameplay/baby_villager_activity", AttributeType::Activity, AttributeValue::Activity(Key::const_vanilla("idle")));

pub(crate) fn register_all(registry: &mut Registry<EnvironmentAttribute>) {
    for attribute in EnvironmentAttribute::all() {
        Registry::register(registry, attribute.key().clone(), *attribute);
    }
}
